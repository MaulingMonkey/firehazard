//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session)\]
//! Pseudo Console APIs
//!
//!
//!
//! ### Version History
//!
//! According to [the announcement blog post](https://devblogs.microsoft.com/commandline/windows-command-line-introducing-the-windows-pseudo-console-conpty/#detecting-the-conpty-api):
//!
//! > The new ConPTY API will be available for the first time in the Autumn/Fall 2018 release of Windows 10.
//!
//! I interpret this to mean they should've become available as part of the October 2018 Update (10.0.17763.*)
//!
//! The APIs may be present then, but they appear either non-functional, or their behavior later changed.
//! In particular, while the relevant APIs all exist and return successfully, they don't actually seem to redirect the output of the process being run.
//! Additionally, these functions will presumably cause your executable to fail to link on earlier versions of Windows.
//!
//! [Create an issue](https://github.com/MaulingMonkey/firehazard/issues) if you want me to replace
//! the implementation of these psuedoconsole functions with gracefully failing dynamic loading.
//! Or if you figure out I just need to enable handle inheritance or somesuch.
//!
//! | Version           | Description                           | Notes     |
//! | ------------------| --------------------------------------| ----------|
//! | 10.0.17763.*      | October 2018 Update                   |           |
//! | 10.0.17763.7009   | GitHub Runner "Windows 2019 Server"   | pseudoconsole APIs fail to redirect output
//! | 10.0.17763.7009   | GitHub Runner "Windows 2025 Server"   | pseudoconsole APIs fail to redirect output
//! | 10.0.19045.*      | 2022 Update                           |
//! | 10.0.19045.5737   | My local machine                      | pseudoconsole API redirect output OK
//! | 10.0.20348.*      | Windows 2022 Server                   |
//! | 10.0.20348.3328   | GitHub Runner "Windows 2022 Server"   | pseudoconsole API redirect output OK
//!
//! References:
//! *   <https://en.wikipedia.org/wiki/Windows_10_version_history>
//! *   <https://en.wikipedia.org/wiki/Windows_11_version_history>
//! *   <https://learn.microsoft.com/en-us/windows/release-health/release-information>
//! *   <https://learn.microsoft.com/en-us/windows/release-health/windows-server-release-info>
//!
//!
//!
//! ### Example
//! ```
//! # #[cfg(std)] {
//! use firehazard::*;
//!
//! let (input, _into_pcon) = pipe::create(None, 0).unwrap();
//! let (mut from_pcon, output) = pipe::create(None, 0).unwrap();
//! let pcon = unsafe { create_pseudo_console([40, 30], &input, &output, 0) }.unwrap();
//!
//! std::thread::scope(move |scope|{
//!     // https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session
//!     // Contains several warnings about deadlocks.  It's even worse than it sounds!
//!     //
//!     //  1. ClosePseudoConsole will block waiting for `output` to drain, so we need a worker.
//!     //  2. The worker thread block closing the scope until `output` is dropped.
//!     //  3. We shouldn't drop `output` before dropping `pcon`.
//!     //
//!     // Ergo, these rebindings are load bearing (and presumably ordering sensitive):
//!     let _output = output;   // otherwise the scope.spawn(...)ed thread will never quit
//!     let pcon    = pcon;     // otherwise `pcon` will outlive `output` which it references
//!
//!     scope.spawn(move ||{
//!         use std::io::Read;
//!         let mut bytes = 0;
//!         while let Ok(read) = from_pcon.read(&mut [0u8; 1024]) {
//!             if read == 0 { break }
//!             bytes += read;
//!         }
//!         assert!(
//!             // TODO: replace "CI" with `cmd /C ver` check?
//!             bytes != 0 || std::env::var_os("CI").is_some(),
//!             "surely `cmd.exe /C ver` should've caused *some* output on 10.0.19045.*+?",
//!         );
//!     });
//!
//!     let attributes = [process::ThreadAttributeRef::pseudoconsole(&pcon)];
//!     let list = Some(process::ThreadAttributeList::try_from(&attributes[..]).unwrap());
//!     let startup = process::StartupInfoExW { attribute_list: list, .. Default::default() };
//!     let mut command_line = argv_to_command_line_0("cmd.exe", ["/C", "ver"]);
//!     let process_info = create_process_w(
//!         (), Some(&mut command_line[..]), None, None, false,
//!         process::EXTENDED_STARTUPINFO_PRESENT,
//!         process::environment::Inherit, (), &startup,
//!     ).unwrap();
//!
//!     let exit_code = wait_for_process(&process_info.process).unwrap();
//!     assert_eq!(0, exit_code);
//! });
//! # }
//! ```
//!



pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\close_pseudo_console.rs");
    include!(r"funcs\create_pseudo_console.rs");
    include!(r"funcs\resize_pseudo_console.rs");
}

mod pseudoconsole_constants;                pub use pseudoconsole_constants::*;
mod pseudoconsole_handles;                  pub use pseudoconsole_handles::*;
mod pseudoconsole_traits;                   pub use pseudoconsole_traits::*;
