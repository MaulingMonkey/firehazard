//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session)\]
//! Pseudo Console APIs
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
//!         assert_ne!(0, bytes, "surely `cmd.exe /C ver` should've caused *some* output");
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



pub use funcs::*;
pub(crate) mod funcs {
    use crate as firehazard;
    include!(r"funcs\close_pseudo_console.rs");
    include!(r"funcs\create_pseudo_console.rs");
    include!(r"funcs\resize_pseudo_console.rs");
}

mod pseudoconsole_constants;                pub use pseudoconsole_constants::*;
mod pseudoconsole_handles;                  pub use pseudoconsole_handles::*;
mod pseudoconsole_traits;                   pub use pseudoconsole_traits::*;
