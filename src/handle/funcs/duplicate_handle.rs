#[doc(alias = "DuplicateHandle")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle(-1, source, -1, access ?? 0, inherit, access ? 0 : DUPLICATE_SAME_ACCESS)
///
pub(crate) unsafe fn duplicate_handle_local_raw<'h, H>(
    source:             *mut H,
    replaced_access:    impl Into<Option<access::Mask>>,
    inherit_handle:     bool,
) -> firehazard::Result<*mut H> {
    let process = get_current_process();
    let replaced_access = replaced_access.into();

    let mut target = null_mut();
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::handleapi::DuplicateHandle(
        process.as_handle(),
        source.cast(),
        process.as_handle(),
        &mut target,
        replaced_access.map_or(0, |a| a.into()),
        inherit_handle as _,
        replaced_access.map_or(winapi::um::winnt::DUPLICATE_SAME_ACCESS, |_| 0),
    )})?;

    Ok(target.cast())
}



#[doc(alias = "DuplicateHandle")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle(-1, source, -1, access, inherit, 0)
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// // You can duplicate process/thread pseudo-handles to get real handles:
/// let process = duplicate_handle_local(get_current_process(), access::GENERIC_ALL, false).unwrap();
/// let thread  = duplicate_handle_local(get_current_thread(),  access::GENERIC_ALL, false).unwrap();
///
/// # #[cfg(nope)] { // current disabled: token::Pseudo -> handle::Pseudo conversion
/// // You *cannot* duplicate token pseudo-handles to get real handles:
/// let t = duplicate_handle_local(get_current_process_token(), access::GENERIC_ALL, false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local(get_current_thread_token(), access::GENERIC_ALL, false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local(get_current_thread_effective_token(), access::GENERIC_ALL, false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// # }
/// ```
///
pub fn duplicate_handle_local<H>(
    source:         H,
    access:         impl Into<access::Mask>,
    inherit_handle: bool,
) -> firehazard::Result<H::Owned> where
    H           : CloneToOwned, // TODO: relax to TryCloneToOwned?
    H           : AsLocalHandle,
    H::Owned    : FromLocalHandle,
{
    Ok(unsafe { H::Owned::from_raw(duplicate_handle_local_raw(
        source.as_handle().cast(),
        access.into(),
        inherit_handle,
    )?)?})
}



#[doc(alias = "DuplicateHandle")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle(-1, source, -1, 0, inherit, DUPLICATE_SAME_ACCESS)
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// // You can duplicate process/thread pseudo-handles to get real handles:
/// let process = duplicate_handle_local_same_access(get_current_process(), false).unwrap();
/// let thread  = duplicate_handle_local_same_access(get_current_thread(),  false).unwrap();
///
/// # #[cfg(nope)] { // current disabled: token::Pseudo -> handle::Pseudo conversion
/// // You *cannot* duplicate token pseudo-handles to get real handles:
/// let t = duplicate_handle_local_same_access(get_current_process_token(), false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local_same_access(get_current_thread_token(), false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local_same_access(get_current_thread_effective_token(), false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// # }
/// ```
///
pub fn duplicate_handle_local_same_access<H>(
    source:         H,
    inherit_handle: bool,
) -> firehazard::Result<H::Owned> where
    H           : CloneToOwned, // TODO: relax to TryCloneToOwned?
    H           : AsLocalHandle,
    H::Owned    : FromLocalHandle,
{
    Ok(unsafe { H::Owned::from_raw(duplicate_handle_local_raw(
        source.as_handle().cast(),
        None,
        inherit_handle,
    )?)?})
}
