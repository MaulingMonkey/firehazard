use crate::*;

use winapi::shared::windef::HDESK__;
use winapi::um::winuser::CloseDesktop;

use core::ptr::NonNull;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// `HDESK` to a desktop
#[repr(transparent)] pub struct OwnedHandle(NonNull<HDESK__>);

handles!(unsafe impl *LocalHandleNN<HDESK__>        for desktop::{OwnedHandle});
handles!(unsafe impl AsRef<Self>                    for desktop::{OwnedHandle});
handles!(unsafe impl Send                           for desktop::{OwnedHandle});
handles!(unsafe impl {AsRef, From}                  for desktop::{OwnedHandle});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for desktop::{OwnedHandle});
handles!(       impl Debug                          for desktop::{OwnedHandle});

impl Drop for OwnedHandle { fn drop(&mut self) {
    let h = self.as_handle();
    assert!(0 != unsafe { CloseDesktop(h) }, "CloseDesktop({h:?}) failed with GetLastError()={:?}", Error::get_last());
}}

unsafe impl valrow::Borrowable for OwnedHandle   { type Abi = NonNull<HDESK__>; }
