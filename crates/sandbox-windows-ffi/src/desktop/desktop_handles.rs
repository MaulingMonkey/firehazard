use crate::*;

use winapi::shared::windef::HDESK;
use winapi::um::winuser::CloseDesktop;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// `HDESK` to a desktop
#[repr(transparent)] pub struct OwnedHandle(HDESK);

handles!(impl *LocalHandle<HDESK>   for desktop::{OwnedHandle});
handles!(impl Debug                 for desktop::{OwnedHandle});
handles!(impl {AsRef, From}         for desktop::{OwnedHandle});
handles!(impl {AsRef<@base>, From}  for desktop::{OwnedHandle});

impl Drop for OwnedHandle { fn drop(&mut self) {
    assert!(self.0.is_null() || (0 != unsafe { CloseDesktop(self.0) }), "CloseDesktop({:?}) failed with GetLastError()={:?}", self.0, Error::get_last());
}}
