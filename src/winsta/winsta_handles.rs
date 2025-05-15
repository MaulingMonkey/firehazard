use crate::prelude::*;

use winapi::shared::minwindef::HWINSTA__;
use winapi::um::winuser::CloseWindowStation;

type HWINSTANN = NonNull<HWINSTA__>;



#[doc(alias = "HWINSTA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// _Owned_, _non-null_ `HWINSTA` to a *window station*.
///
#[repr(transparent)] pub struct OwnedHandle(HWINSTANN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindowstation)\]
/// CloseWindowStation
impl Drop for OwnedHandle { #[doc(alias = "CloseWindowStation")] fn drop(&mut self) {
    let h = self.as_handle();
    assert!(0 != unsafe { CloseWindowStation(h) }, "CloseWindowStation({h:?}) failed with GetLastError()={:?}", firehazard::Error::get_last());
}}



#[doc(alias = "HWINSTA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// _Borrowed_, _non-null_ `HWINSTA` to a *window station*.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HWINSTANN, PhantomData<&'a HWINSTANN>);



// No pseudo window station handles



handles!(unsafe impl *LocalHandleNN<HWINSTA__>      for winsta::{OwnedHandle, Handle<'_>});
handles!(unsafe impl TryCloneToOwned<OwnedHandle>   for winsta::{OwnedHandle, Handle<'_>});
handles!(unsafe impl Send                           for winsta::{OwnedHandle, Handle<'_>});
handles!(unsafe impl Sync                           for winsta::{OwnedHandle, Handle<'_>});
handles!(       impl Debug                          for winsta::{OwnedHandle, Handle<'_>});

handles!(unsafe impl @convert &'_ winsta::OwnedHandle   => winsta::Handle<'_>   );
//ndles!(unsafe impl @convert     winsta::OwnedHandle   => handle::Owned        ); // XXX: inappropriate - closed via CloseWindowStation, not CloseHandle
handles!(unsafe impl @convert &'_ winsta::OwnedHandle   => handle::Borrowed<'_> ); //      appropriate - desktops can be `GetHandleInformation(...)ed`.
handles!(unsafe impl @convert &'_ winsta::OwnedHandle   => handle::Pseudo<'_>   );
handles!(unsafe impl @convert winsta::Handle<'_>        => handle::Borrowed<'_>  );
handles!(unsafe impl @convert winsta::Handle<'_>        => handle::Pseudo<'_>    );

unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = HWINSTANN; }
unsafe impl valrow::Borrowable for Handle<'_>        { type Abi = HWINSTANN; }

impl CloneToOwned for OwnedHandle   {}
impl CloneToOwned for Handle<'_>    {}



impl<'a> TryFrom<handle::Borrowed<'a>> for winsta::Handle<'a> {
    type Error = firehazard::Error;
    fn try_from(handle: handle::Borrowed<'a>) -> Result<Self, Self::Error> {
        verify_winsta_handle(handle)?;
        Ok(unsafe { core::mem::transmute(handle) })
    }
}

/// Check if `handle` is a window station handle.
/// May fail on valid window station handles if both [`nt_query_object_type_name`] and [`enum_desktops_w`] fail (lacks the `WINSTA_ENUMDESKTOPS` access right?)
///
fn verify_winsta_handle(handle: handle::Borrowed<'_>) -> firehazard::Result<()> {
    // Perform the hopefully fast check first
    #[cfg(std)] match nt_query_object_type_name(handle) {
        Ok(ty) if ty == "WindowStation"                                                                 => return Ok(()),
        Ok(ty) if "Desktop File Job Process Thread Token".split(' ').any(|reject| ty == reject)         => return Err(ERROR::INVALID_HANDLE.into()), // considered ERROR::UNSUPPORTED_TYPE, but INVALID_HANDLE is more consistent
        Err(err) if err == ERROR::INVALID_HANDLE                                                        => return Err(err.into()),
        Ok(_ty)                                                                                         => {}, // continue - perhaps `Desktop` got renamed?
        Err(_err)                                                                                       => {}, // continue - perhaps `nt_query_object_type_name` was simply unavailable?
    }
    // `nt_query_object_type_name` failed to provide a conclusive answer, slow path it is

    firehazard::Error::get_last_if(FALSE == unsafe { winapi::um::winuser::EnumDesktopsW(
        handle.as_handle().cast(),
        Some({
            extern "system" fn noop(_desktop: *mut u16, _param: winapi::shared::minwindef::LPARAM) -> BOOL {
                unsafe { winapi::um::errhandlingapi::SetLastError(0) };
                FALSE
            }
            noop
        }),
        0,
    )}).unerr(0, ())
}

#[test] fn try_from_winsta() {
    let owned = open_process_window_station().unwrap();
    let untyped_borrowed = handle::Borrowed::from(&owned);
    let _borrowed = winsta::Handle::try_from(untyped_borrowed).unwrap();
}

#[test] fn try_from_thread() {
    let owned = get_current_thread().try_clone_to_owned().unwrap();
    let untyped_borrowed = handle::Borrowed::from(&owned);
    let err = winsta::Handle::try_from(untyped_borrowed).unwrap_err();
    assert_eq!(err, ERROR::INVALID_HANDLE);
}
