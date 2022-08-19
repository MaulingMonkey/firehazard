use crate::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::*;
use winapi::um::handleapi::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
pub fn close_handle(object: handle::Owned) -> Result<(), Error> {
    let h = object.as_handle();
    std::mem::forget(object);
    Error::get_last_if(FALSE == unsafe { CloseHandle(h) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-compareobjecthandles)\]
/// <strike>CompareObjectHandles</strike>
#[cfg(doc)] /// NYI (windows SDK too early to link this win10+ API?)
pub fn compare_object_handles(first: &handle::Owned, second: &handle::Owned) -> bool {
    // #[link(name = "kernelbase")] extern {} // unable to link against kernelbase?
    FALSE != unsafe { CompareObjectHandles(first.as_handle(), second.as_handle()) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle
pub fn duplicate_handle<'t>(
    source_process: &process::Handle,
    source:         &handle::Owned,
    target_process: impl Into<Option<&'t process::Handle>>,
    desired_access: u32,                                    // TODO: type
    inherit_handle: bool,
    options:        u32,                                    // TODO: type
) -> Result<handle::Owned, Error> {
    let mut target = null_mut();
    Error::get_last_if(FALSE == unsafe { DuplicateHandle(
        source_process.as_handle(),
        source.as_handle(),
        target_process.into().map_or(null_mut(), |h| h.as_handle()),
        &mut target,
        desired_access,
        inherit_handle as _,
        options,
    )})?;

    unsafe { handle::Owned::from_raw(target) }.ok_or(Error(ERROR_INVALID_HANDLE))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-gethandleinformation)\]
/// GetHandleInformation
pub fn get_handle_information(object: &handle::Owned) ->  Result<u32, Error> { // TODO: type
    let mut flags = 0;
    Error::get_last_if(FALSE == unsafe { GetHandleInformation(object.as_handle(), &mut flags) })?;
    Ok(flags)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-sethandleinformation)\]
/// SetHandleInformation
pub fn set_handle_information(object: &handle::Owned, mask: u32, flags: u32) -> Result<(), Error> { // TODO: type
    Error::get_last_if(FALSE == unsafe { SetHandleInformation(object.as_handle(), mask, flags) })
}
