/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle(-1, source, -1, access, inherit, 0)
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// // You can duplicate process/thread psuedo-handles to get real handles:
/// let process = duplicate_handle_local(get_current_process(), access::GENERIC_ALL, false).unwrap();
/// let thread  = duplicate_handle_local(get_current_thread(),  access::GENERIC_ALL, false).unwrap();
///
/// # #[cfg(nope)] { // current disabled: token::Psuedo -> handle::Psuedo conversion
/// // You *cannot* duplicate token psuedo-handles to get real handles:
/// let t = duplicate_handle_local(get_current_process_token(), access::GENERIC_ALL, false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local(get_current_thread_token(), access::GENERIC_ALL, false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local(get_current_thread_effective_token(), access::GENERIC_ALL, false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// # }
/// ```
pub fn duplicate_handle_local<'a>(
    source:         impl AsRef<firehazard::handle::Psuedo<'a>>,
    access:         impl Into<firehazard::access::Mask>,
    inherit_handle: bool,
) -> Result<firehazard::handle::Owned, firehazard::Error> { // TODO: better handle type inference
    use firehazard::*;

    let process = get_current_process();
    let source = source.as_ref();

    let mut target = core::ptr::null_mut();
    Error::get_last_if(0 == unsafe { winapi::um::handleapi::DuplicateHandle(
        process.as_handle(),
        source.as_handle(),
        process.as_handle(),
        &mut target,
        access.into().into(),
        inherit_handle as _,
        0
    )})?;

    unsafe { handle::Owned::from_raw(target) }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle(-1, source, -1, 0, inherit, DUPLICATE_SAME_ACCESS)
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// // You can duplicate process/thread psuedo-handles to get real handles:
/// let process = duplicate_handle_local_same_access(get_current_process(), false).unwrap();
/// let thread  = duplicate_handle_local_same_access(get_current_thread(),  false).unwrap();
///
/// # #[cfg(nope)] { // current disabled: token::Psuedo -> handle::Psuedo conversion
/// // You *cannot* duplicate token psuedo-handles to get real handles:
/// let t = duplicate_handle_local_same_access(get_current_process_token(), false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local_same_access(get_current_thread_token(), false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local_same_access(get_current_thread_effective_token(), false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// # }
/// ```
pub fn duplicate_handle_local_same_access<'a>(
    source:         impl AsRef<firehazard::handle::Psuedo<'a>>,
    inherit_handle: bool,
) -> Result<firehazard::handle::Owned, firehazard::Error> { // TODO: better handle type inference
    use firehazard::*;

    let process = get_current_process();
    let source = source.as_ref();

    let mut target = core::ptr::null_mut();
    Error::get_last_if(0 == unsafe { winapi::um::handleapi::DuplicateHandle(
        process.as_handle(),
        source.as_handle(),
        process.as_handle(),
        &mut target,
        0,
        inherit_handle as _,
        winapi::um::winnt::DUPLICATE_SAME_ACCESS,
    )})?;

    unsafe { handle::Owned::from_raw(target) }
}
