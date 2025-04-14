use crate::*;

use core::ptr::NonNull;

use winapi::shared::minwindef::HWINSTA__;
use winapi::um::winuser::CloseWindowStation;



#[doc(alias = "HWINSTA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// `HWINSTA` to a window station
///
#[repr(transparent)] pub struct OwnedHandle(NonNull<HWINSTA__>);



handles!(unsafe impl *LocalHandleNN<HWINSTA__>      for winsta::{OwnedHandle});
handles!(unsafe impl AsRef<Self>                    for winsta::{OwnedHandle});
handles!(unsafe impl Send                           for winsta::{OwnedHandle});
handles!(unsafe impl {AsRef, From}                  for winsta::{OwnedHandle});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for winsta::{OwnedHandle});
handles!(impl Debug                                 for winsta::{OwnedHandle});

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindowstation)\]
/// CloseWindowStation
impl Drop for OwnedHandle { fn drop(&mut self) {
    let h = self.as_handle();
    assert!(0 != unsafe { CloseWindowStation(h) }, "CloseWindowStation({h:?}) failed with GetLastError()={:?}", Error::get_last());
}}

unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = NonNull<HWINSTA__>; }
