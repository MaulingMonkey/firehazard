use crate::*;

use winapi::shared::windef::HDESK__;
use winapi::um::winuser::CloseDesktop;

use core::ptr::NonNull;



#[doc(alias = "HDESK")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// _Owned_, _non-null_ `HDESK` to a *desktop*.
///
#[repr(transparent)] pub struct OwnedHandle(NonNull<HDESK__>);

handles!(unsafe impl *LocalHandleNN<HDESK__>        for desktop::{OwnedHandle});
handles!(       impl AsRef<Self>                    for desktop::{OwnedHandle});
handles!(unsafe impl Send                           for desktop::{OwnedHandle});
handles!(       impl Debug                          for desktop::{OwnedHandle});

//ndles!(unsafe impl @convert     desktop::OwnedHandle => handle::Owned         ); // XXX: inappropriate - desktops cannot be `CloseHandle(...)d`.
handles!(unsafe impl @convert &'_ desktop::OwnedHandle => handle::Borrowed<'_>  ); //      appropriate - desktops can be `GetHandleInformation(...)ed`.
handles!(unsafe impl @convert &'_ desktop::OwnedHandle => handle::Pseudo<'_>    );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closedesktop)\]
/// CloseDesktop
impl Drop for OwnedHandle { fn drop(&mut self) {
    let h = self.as_handle();
    assert!(0 != unsafe { CloseDesktop(h) }, "CloseDesktop({h:?}) failed with GetLastError()={:?}", Error::get_last());
}}

unsafe impl valrow::Borrowable for OwnedHandle   { type Abi = NonNull<HDESK__>; }

impl OwnedHandle { #[doc(alias = "DuplicateHandle")] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\] DuplicateHandle"] pub fn try_clone(&self) -> Result<OwnedHandle, Error> { Ok(OwnedHandle(duplicate_handle_local_same_access(self, false)?.into_handle_nn().cast())) } }
