use crate::prelude::*;



#[doc(alias = "HPCON")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session)\]
/// _Owned_, _non-null_ `HPCON` to a *pseudo console*.
///
#[repr(transparent)] pub struct Owned(pub(super) HANDLENN);



handles!(unsafe impl *LocalHandleNN<c_void>     for pseudoconsole::{Owned});
handles!(unsafe impl TryCloneToOwned<Owned>     for pseudoconsole::{Owned});
handles!(unsafe impl Send                       for pseudoconsole::{Owned});
handles!(       impl Debug                      for pseudoconsole::{Owned});

//ndles!(unsafe impl @convert     pseudoconsole::Owned   => handle::Owned        ); // XXX: closed via ClosePseudoConsole, not CloseHandle
handles!(unsafe impl @convert &'_ pseudoconsole::Owned   => handle::Borrowed<'_> );
handles!(unsafe impl @convert &'_ pseudoconsole::Owned   => handle::Pseudo<'_>   );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/closepseudoconsole)\]
/// ClosePseudoConsole
impl Drop for Owned { fn drop(&mut self) {
    // NOTE: ClosePseudoConsole returns no errors
    let h = self.as_handle();
    unsafe { winapi::um::consoleapi::ClosePseudoConsole(h) };
}}

unsafe impl valrow::Borrowable for Owned { type Abi = NonNull<c_void>; }

impl CloneToOwned for Owned {}
