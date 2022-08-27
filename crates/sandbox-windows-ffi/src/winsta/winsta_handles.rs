use crate::*;

use winapi::shared::minwindef::HWINSTA;
use winapi::um::winuser::CloseWindowStation;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// `HWINSTA` to a window station
#[repr(transparent)] pub struct OwnedHandle(HWINSTA);

handles!(impl *LocalHandle<HWINSTA> for winsta::{OwnedHandle});
handles!(impl Debug                 for winsta::{OwnedHandle});
handles!(impl {AsRef, From}         for winsta::{OwnedHandle});
handles!(impl {AsRef<@base>, From}  for winsta::{OwnedHandle});

impl Drop for OwnedHandle { fn drop(&mut self) {
    assert!(self.0.is_null() || (0 != unsafe { CloseWindowStation(self.0) }), "CloseWindowStation({:?}) failed with GetLastError()={:?}", self.0, Error::get_last());
}}
