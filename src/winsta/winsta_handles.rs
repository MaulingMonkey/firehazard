use crate::prelude::*;

use winapi::shared::minwindef::HWINSTA__;
use winapi::um::winuser::CloseWindowStation;



#[doc(alias = "HWINSTA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// _Owned_, _non-null_ `HWINSTA` to a *window station*.
///
#[repr(transparent)] pub struct OwnedHandle(NonNull<HWINSTA__>);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindowstation)\]
/// CloseWindowStation
impl Drop for OwnedHandle { fn drop(&mut self) {
    let h = self.as_handle();
    assert!(0 != unsafe { CloseWindowStation(h) }, "CloseWindowStation({h:?}) failed with GetLastError()={:?}", firehazard::Error::get_last());
}}



handles!(unsafe impl *LocalHandleNN<HWINSTA__>      for winsta::{OwnedHandle});
handles!(unsafe impl TryCloneToOwned<OwnedHandle>   for winsta::{OwnedHandle});
handles!(unsafe impl Send                           for winsta::{OwnedHandle});
handles!(       impl Debug                          for winsta::{OwnedHandle});

//ndles!(unsafe impl @convert     winsta::OwnedHandle   => handle::Owned        ); // XXX: closed via CloseWindowStation, not CloseHandle
handles!(unsafe impl @convert &'_ winsta::OwnedHandle   => handle::Borrowed<'_> );
handles!(unsafe impl @convert &'_ winsta::OwnedHandle   => handle::Pseudo<'_>   );

unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = NonNull<HWINSTA__>; }

impl CloneToOwned for OwnedHandle {}
