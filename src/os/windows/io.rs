//! [`std::os::windows::io`] re-export or [`no_std`](https://docs.rust-embedded.org/book/intro/no-std.html) placeholders

use crate::*;
use crate::os::windows::raw::{*, HANDLE};

use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::{CloseHandle, DuplicateHandle, INVALID_HANDLE_VALUE};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winnt::DUPLICATE_SAME_ACCESS;

use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ptr::null_mut;



#[derive(Clone, Copy)] #[repr(transparent)] pub struct BorrowedHandle<'handle> {
    handle:     HANDLE,
    lifetime:   PhantomData<&'handle OwnedHandle>,
}

// XXX: BorrowedSocket

#[repr(transparent)] pub struct HandleOrInvalid {
    handle:     HANDLE,
}

#[repr(transparent)] pub struct HandleOrNull {
    handle:     HANDLE,
}

#[derive(Clone, Copy, PartialEq, Eq)] pub struct InvalidHandleError(());
#[derive(Clone, Copy, PartialEq, Eq)] pub struct NullHandleError(());

#[repr(transparent)] pub struct OwnedHandle {
    handle:     HANDLE,
}

// XXX: OwnedSocket

pub trait AsHandle {
    fn as_handle(&self) -> BorrowedHandle<'_>;
}

pub trait AsRawHandle {
    fn as_raw_handle(&self) -> RawHandle;
}

//b trait AsRawSocket { ... }

//b trait AsSocket { ... }

pub trait FromRawHandle {
    unsafe fn from_raw_handle(handle: RawHandle) -> Self;
}

//b trait FromRawSocket { ... }

pub trait IntoRawHandle {
    fn into_raw_handle(self) -> RawHandle;
}

//b trait IntoRawSocket { ... }

pub type RawHandle = HANDLE;

//b type RawSocket = SOCKET;



impl AsRawHandle for BorrowedHandle<'_> { fn as_raw_handle(&self) -> RawHandle { self.handle } }
impl AsRawHandle for OwnedHandle        { fn as_raw_handle(&self) -> RawHandle { self.handle } }

impl AsHandle for BorrowedHandle<'_>    { fn as_handle(&self) -> BorrowedHandle<'_> { *self } }
impl AsHandle for OwnedHandle           { fn as_handle(&self) -> BorrowedHandle<'_> { BorrowedHandle { handle: self.handle, lifetime: PhantomData } } }
impl<T: AsHandle + ?Sized> AsHandle for &    T      { fn as_handle(&self) -> BorrowedHandle<'_> { (**self).as_handle() } }
impl<T: AsHandle + ?Sized> AsHandle for &mut T      { fn as_handle(&self) -> BorrowedHandle<'_> { (**self).as_handle() } }
//pl<T: AsHandle + ?Sized> AsHandle for Box<T>      { ... } // XXX: requires std, this mod is no_std only
//pl<T: AsHandle + ?Sized> AsHandle for Rc<T>       { ... } // XXX: requires std, this mod is no_std only
//pl<T: AsHandle + ?Sized> AsHandle for UniqueRc<T> { ... } // XXX: requires std, this mod is no_std only
//pl<T: AsHandle + ?Sized> AsHandle for Arc<T>      { ... } // XXX: requires std, this mod is no_std only

//pl Drop for BorrowedHandle        { ...it's borrowed, don't close the handle on drop... }
impl Drop for HandleOrInvalid       { fn drop(&mut self) { if self.handle.cast() == INVALID_HANDLE_VALUE    { return }  unsafe { CloseHandle(self.handle.cast()); } } }
impl Drop for HandleOrNull          { fn drop(&mut self) { if self.handle.is_null()                         { return }  unsafe { CloseHandle(self.handle.cast()); } } }
impl Drop for OwnedHandle           { fn drop(&mut self) {                                                              unsafe { CloseHandle(self.handle.cast()); } } }

impl Debug for BorrowedHandle<'_>   { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_struct("BorrowedHandle" ).field("handle", &self.handle).finish() } }
impl Debug for HandleOrInvalid      { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_struct("HandleOrInvalid").field("handle", &self.handle).finish() } }
impl Debug for HandleOrNull         { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_struct("HandleOrNull"   ).field("handle", &self.handle).finish() } }
impl Debug for OwnedHandle          { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_struct("OwnedHandle"    ).field("handle", &self.handle).finish() } }

impl Debug for InvalidHandleError   { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("InvalidHandleError(...)") } }
impl Debug for NullHandleError      { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("NullHandleError(...)") } }

impl Display for InvalidHandleError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("`handle` was `INVALID_HANDLE_VALUE`") } }
impl Display for NullHandleError    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str("`handle` was null") } }

impl Error for InvalidHandleError   { fn description(&self) -> &str { "`handle` was `INVALID_HANDLE_VALUE`" } }
impl Error for NullHandleError      { fn description(&self) -> &str { "`handle` was null" } }

impl FromRawHandle for OwnedHandle { unsafe fn from_raw_handle(handle: RawHandle) -> Self { Self { handle } } }
impl IntoRawHandle for OwnedHandle { fn into_raw_handle(self) -> RawHandle { let this = ManuallyDrop::new(self); this.handle } }

//pl IsTerminal for BorrowedHandle<'_>  { ... }
//pl IsTerminal for OwnedHandle         { ... }

unsafe impl Send for BorrowedHandle<'_> {}
unsafe impl Send for HandleOrInvalid {}
unsafe impl Send for HandleOrNull {}
unsafe impl Send for OwnedHandle {}

unsafe impl Sync for BorrowedHandle<'_> {}
unsafe impl Sync for HandleOrInvalid {}
unsafe impl Sync for HandleOrNull {}
unsafe impl Sync for OwnedHandle {}

impl TryFrom<HandleOrInvalid> for OwnedHandle {
    type Error = InvalidHandleError;
    fn try_from(hoi: HandleOrInvalid) -> Result<Self, Self::Error> {
        let handle = ManuallyDrop::new(hoi).handle;
        if handle.cast() == INVALID_HANDLE_VALUE {
            Err(InvalidHandleError(()))
        } else {
            Ok(Self { handle })
        }
    }
}

impl TryFrom<HandleOrNull> for OwnedHandle {
    type Error = NullHandleError;
    fn try_from(hon: HandleOrNull) -> Result<Self, Self::Error> {
        let handle = ManuallyDrop::new(hon).handle;
        if handle.is_null() {
            Err(NullHandleError(()))
        } else {
            Ok(Self { handle })
        }
    }
}



impl BorrowedHandle<'_> {
    pub const unsafe fn borrow_raw(handle: RawHandle) -> Self { Self { handle, lifetime: PhantomData } }
    pub fn try_clone_to_owned(&self) -> io::Result<OwnedHandle> { OwnedHandle::duplicate_from(self.handle) }
}

impl HandleOrInvalid {
    pub unsafe fn from_raw_handle(handle: RawHandle) -> Self { Self { handle } }
}

impl HandleOrNull {
    pub unsafe fn from_raw_handle(handle: RawHandle) -> Self { Self { handle } }
}

impl OwnedHandle {
    pub fn try_clone(&self) -> io::Result<Self> { Self::duplicate_from(self.handle) }

    fn duplicate_from(original: RawHandle) -> io::Result<Self> {
        // XXX: simplify via duplicate_handle_local_same_access?
        if original.is_null() { return Ok(OwnedHandle { handle: original }) }
        let process = unsafe { GetCurrentProcess() };
        let mut duplicate = null_mut();
        if FALSE == unsafe { DuplicateHandle(process, original.cast(), process, &mut duplicate, 0, FALSE, DUPLICATE_SAME_ACCESS) } {
            Err(io::Error::last_os_error())
        } else {
            Ok(OwnedHandle { handle: duplicate.cast() })
        }
    }
}
