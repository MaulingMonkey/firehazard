use crate::prelude::*;

use winapi::shared::windef::HDESK__;
use winapi::um::winuser::CloseDesktop;

type HDESKNN = NonNull<HDESK__>;



#[doc(alias = "HDESK")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// _Owned_, _non-null_ `HDESK` to a *desktop*.
///
#[repr(transparent)] pub struct OwnedHandle(HDESKNN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closedesktop)\]
/// CloseDesktop
impl Drop for OwnedHandle { #[doc(alias = "CloseDesktop")] fn drop(&mut self) {
    let h = self.as_handle();
    assert!(0 != unsafe { CloseDesktop(h) }, "CloseDesktop({h:?}) failed with GetLastError()={:?}", firehazard::Error::get_last());
}}



#[doc(alias = "HDESK")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// _Borrowed_, _non-null_ `HDESK` to a *desktop*.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HDESKNN, PhantomData<&'a HDESKNN>);



// No pseudo desktop handles



handles!(unsafe impl *LocalHandleNN<HDESK__>        for desktop::{OwnedHandle, Handle<'_>});
handles!(unsafe impl TryCloneToOwned<OwnedHandle>   for desktop::{OwnedHandle, Handle<'_>});
handles!(unsafe impl Send                           for desktop::{OwnedHandle, Handle<'_>});
handles!(unsafe impl Sync                           for desktop::{OwnedHandle, Handle<'_>});
handles!(       impl Debug                          for desktop::{OwnedHandle, Handle<'_>});

handles!(unsafe impl @convert &'_ desktop::OwnedHandle => desktop::Handle<'_>   );
//ndles!(unsafe impl @convert     desktop::OwnedHandle => handle::Owned         ); // XXX: inappropriate - desktops cannot be `CloseHandle(...)d`.
handles!(unsafe impl @convert &'_ desktop::OwnedHandle => handle::Borrowed<'_>  ); //      appropriate - desktops can be `GetHandleInformation(...)ed`.
handles!(unsafe impl @convert &'_ desktop::OwnedHandle => handle::Pseudo<'_>    );
handles!(unsafe impl @convert desktop::Handle<'_>      => handle::Borrowed<'_>  );
handles!(unsafe impl @convert desktop::Handle<'_>      => handle::Pseudo<'_>    );

unsafe impl valrow::Borrowable for OwnedHandle   { type Abi = HDESKNN; }
unsafe impl valrow::Borrowable for Handle<'_>    { type Abi = HDESKNN; }

impl CloneToOwned for OwnedHandle   {}
impl CloneToOwned for Handle<'_>    {}



#[cfg(std)] // XXX: could be relaxed if we create a non-std thread wrapper
impl<'a> TryFrom<handle::Borrowed<'a>> for desktop::Handle<'a> {
    type Error = firehazard::Error;
    fn try_from(handle: handle::Borrowed<'a>) -> Result<Self, Self::Error> {
        verify_desktop_handle(handle)?;
        Ok(unsafe { core::mem::transmute(handle) })
    }
}

/// Check if `handle` is a desktop handle.
/// Hopefully, this is just a matter of checking the result of [`nt_query_object_type_name`].
/// A fallback path, however, fires up an entirely new thread to call [`SetThreadDesktop`].
///
/// This new thread serves multiple purpouses:
/// *   Avoids failures if the current thread has created any windows or hooks, which may cause [`SetThreadDesktop`] to fail.
/// *   Prevents any race conditions with other threads checking the *current* thread's active desktop (however unlikely that might be.)
/// *   Isolates state change such that we can simply discard the thread and it's associated state, instead of attempting to restore state, which could fail.
///
/// [`SetThreadDesktop`]:   https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddesktop
///
#[cfg(std)] // XXX: could be relaxed if we create a non-std thread wrapper
fn verify_desktop_handle(handle: handle::Borrowed<'_>) -> firehazard::Result<()> {
    // Perform the hopefully fast check first
    match nt_query_object_type_name(handle) {
        Ok(ty) if ty == "Desktop"                                                                       => return Ok(()),
        Ok(ty) if "File Job Process Thread Token WindowStation".split(' ').any(|reject| ty == reject)   => return Err(ERROR::INVALID_HANDLE.into()), // considered ERROR::UNSUPPORTED_TYPE, but INVALID_HANDLE is more consistent
        Err(err) if err == ERROR::INVALID_HANDLE                                                        => return Err(err.into()),
        Ok(_ty)                                                                                         => {}, // continue - perhaps `Desktop` got renamed?
        Err(_err)                                                                                       => {}, // continue - perhaps `nt_query_object_type_name` was simply unavailable?
    }
    // `nt_query_object_type_name` failed to provide a conclusive answer, slow path it is

    // SAFETY:  I'm assuming `handle` is `Send`able only for the limited purpouse of attempting to call `SetThreadDesktop` on it.
    let handle = unsafe { AssertSendable::new(handle.as_handle().cast()) };
    std::thread::spawn(move || {
        firehazard::Error::get_last_if(0 != unsafe { winapi::um::winuser::SetThreadDesktop(handle.into_inner()) })
    }).join().map_err(|_| ERROR::NO_SYSTEM_RESOURCES)?
}

#[cfg(std)] #[test] fn try_from_desktop() {
    let owned = open_thread_desktop(get_current_thread_id()).unwrap();
    let untyped_borrowed = handle::Borrowed::from(&owned);
    let _borrowed = desktop::Handle::try_from(untyped_borrowed).unwrap();
}

#[cfg(std)] #[test] fn try_from_thread() {
    let owned = get_current_thread().try_clone_to_owned().unwrap();
    let untyped_borrowed = handle::Borrowed::from(&owned);
    let err = desktop::Handle::try_from(untyped_borrowed).unwrap_err();
    assert_eq!(err, ERROR::INVALID_HANDLE);
}
