#[doc(alias = "CreateNamedPipe")]
#[doc(alias = "CreateNamedPipeW")]
#[cfg(std)]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\]
/// CreateNamedPipeW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// # use winapi::shared::winerror::*;
/// #
/// let pipe = pipe::named::create(
///     r#"\\.\pipe\local\firehazard-create_named_pipe-example"#,
///     pipe::ACCESS_DUPLEX,                                 // (3) open_mode
///     pipe::TYPE_BYTE | pipe::READMODE_BYTE | pipe::WAIT | pipe::REJECT_REMOTE_CLIENTS, // (0) pipe_mode
///     pipe::UNLIMITED_INSTANCES,                          // max_instances
///     0,                                                  // out_buffer_size (advisory)
///     0,                                                  // in_buffer_size  (advisory)
///     None,                                               // default timeout (50ms)
///     None,                                               // overlapped
/// ).unwrap();
///
/// // A max_instances count of 0 will result in ERROR_INVALID_PARAMETER
/// assert_eq!(ERROR_INVALID_PARAMETER, pipe::named::create(
///     r#"\\.\pipe\local\firehazard-create_named_pipe-example-error-bad-max_instances"#,
///     pipe::ACCESS_DUPLEX, 0, pipe::MaxInstances::from_unchecked(0), 0, 0, None, None,
/// ).unwrap_err());
///
/// // An open_mode of 0 will result in ERROR_INVALID_PARAMETER
/// assert_eq!(ERROR_INVALID_PARAMETER, pipe::named::create(
///     r#"\\.\pipe\local\firehazard-create_named_pipe-example-error-bad-open_mode"#,
///     0, 0, pipe::UNLIMITED_INSTANCES, 0, 0, None, None,
/// ).unwrap_err());
/// ```
///
pub fn create<'a, 'b: 'a>(
    name:                   impl AsRef<std::ffi::OsStr>,
    open_mode:              u32, // TODO: type
    pipe_mode:              u32, // TODO: type
    max_instances:          impl Into<firehazard::pipe::MaxInstances>,
    out_buffer_size:        u32,
    in_buffer_size:         u32,
    default_timeout:        impl Into<firehazard::NMPWAIT>, // A slightly awkward fit, but 0 (NMPWAIT::USE_DEFAULT_WAIT) gets interpreted as "make WaitNamedPipe(..., NMPWAIT::USE_DEFAULT_WAIT) use the default timeout of 50ms"
    security_attributes:    impl Into<Option<&'a firehazard::security::Attributes<'b>>>,
) -> Result<firehazard::handle::Owned, firehazard::Error> {
    create_w(
        crate::util::osstr_to_wide0(name.as_ref(), &mut std::vec::Vec::new())?,
        open_mode, pipe_mode, max_instances, out_buffer_size,
        in_buffer_size, default_timeout, security_attributes,
    )
}



#[doc(alias = "CreateNamedPipeA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createnamedpipea)\]
/// CreateNamedPipeA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// # use winapi::shared::winerror::*;
/// #
/// let pipe = pipe::named::create_a(
///     cstr8!(r#"\\.\pipe\local\firehazard-create_named_pipe_a-example"#),
///     pipe::ACCESS_DUPLEX,                                 // (3) open_mode
///     pipe::TYPE_BYTE | pipe::READMODE_BYTE | pipe::WAIT | pipe::REJECT_REMOTE_CLIENTS, // (0) pipe_mode
///     pipe::UNLIMITED_INSTANCES,                          // max_instances
///     0,                                                  // out_buffer_size (advisory)
///     0,                                                  // in_buffer_size  (advisory)
///     None,                                               // default timeout (50ms)
///     None,                                               // overlapped
/// ).unwrap();
///
/// // A max_instances count of 0 will result in ERROR_INVALID_PARAMETER
/// assert_eq!(ERROR_INVALID_PARAMETER, pipe::named::create_a(
///     cstr8!(r#"\\.\pipe\local\firehazard-create_named_pipe_a-example-error-bad-max_instances"#),
///     pipe::ACCESS_DUPLEX, 0, pipe::MaxInstances::from_unchecked(0), 0, 0, None, None,
/// ).unwrap_err());
///
/// // An open_mode of 0 will result in ERROR_INVALID_PARAMETER
/// assert_eq!(ERROR_INVALID_PARAMETER, pipe::named::create_a(
///     cstr8!(r#"\\.\pipe\local\firehazard-create_named_pipe_a-example-bad-open_mode"#),
///     0, 0, pipe::UNLIMITED_INSTANCES, 0, 0, None, None,
/// ).unwrap_err());
/// ```
///
pub fn create_a<'a, 'b: 'a>(
    name:                   impl abistr::TryIntoAsCStr,
    open_mode:              u32, // TODO: type
    pipe_mode:              u32, // TODO: type
    max_instances:          impl Into<firehazard::pipe::MaxInstances>,
    out_buffer_size:        u32,
    in_buffer_size:         u32,
    default_timeout:        impl Into<firehazard::NMPWAIT>, // A slightly awkward fit, but 0 (NMPWAIT::USE_DEFAULT_WAIT) gets interpreted as "make WaitNamedPipe(..., NMPWAIT::USE_DEFAULT_WAIT) use the default timeout of 50ms"
    security_attributes:    impl Into<Option<&'a firehazard::security::Attributes<'b>>>,
) -> Result<firehazard::handle::Owned, firehazard::Error> {
    use crate::FromLocalHandle;
    use abistr::AsCStr;

    let handle = unsafe { winapi::um::winbase::CreateNamedPipeA(
        name.try_into()?.as_cstr(),
        open_mode,
        pipe_mode,
        max_instances.into().0,
        out_buffer_size,
        in_buffer_size,
        default_timeout.into().0,
        security_attributes.into().map_or(core::ptr::null(), |sa| sa) as *mut _,
    )};
    firehazard::Error::get_last_if(handle.is_null() || handle == winapi::um::handleapi::INVALID_HANDLE_VALUE)?;
    unsafe { firehazard::handle::Owned::from_raw(handle) }
}



#[doc(alias = "CreateNamedPipe")]
#[doc(alias = "CreateNamedPipeW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\]
/// CreateNamedPipeW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// # use winapi::shared::winerror::*;
/// #
/// let pipe = pipe::named::create_w(
///     cstr16!(r#"\\.\pipe\local\firehazard-create_named_pipe_w-example"#),
///     pipe::ACCESS_DUPLEX,                                 // (3) open_mode
///     pipe::TYPE_BYTE | pipe::READMODE_BYTE | pipe::WAIT | pipe::REJECT_REMOTE_CLIENTS, // (0) pipe_mode
///     pipe::UNLIMITED_INSTANCES,                          // max_instances
///     0,                                                  // out_buffer_size (advisory)
///     0,                                                  // in_buffer_size  (advisory)
///     None,                                               // default timeout (50ms)
///     None,                                               // overlapped
/// ).unwrap();
///
/// // A max_instances count of 0 will result in ERROR_INVALID_PARAMETER
/// assert_eq!(ERROR_INVALID_PARAMETER, pipe::named::create_w(
///     cstr16!(r#"\\.\pipe\local\firehazard-create_named_pipe_w-example-error-bad-max_instances"#),
///     pipe::ACCESS_DUPLEX, 0, pipe::MaxInstances::from_unchecked(0), 0, 0, None, None,
/// ).unwrap_err());
///
/// // An open_mode of 0 will result in ERROR_INVALID_PARAMETER
/// assert_eq!(ERROR_INVALID_PARAMETER, pipe::named::create_w(
///     cstr16!(r#"\\.\pipe\local\firehazard-create_named_pipe_w-example-error-bad-open_mode"#),
///     0, 0, pipe::UNLIMITED_INSTANCES, 0, 0, None, None,
/// ).unwrap_err());
/// ```
///
pub fn create_w<'a, 'b: 'a>(
    name:                   impl abistr::TryIntoAsCStr<u16>,
    open_mode:              u32, // TODO: type
    pipe_mode:              u32, // TODO: type
    max_instances:          impl Into<firehazard::pipe::MaxInstances>,
    out_buffer_size:        u32,
    in_buffer_size:         u32,
    default_timeout:        impl Into<firehazard::NMPWAIT>, // A slightly awkward fit, but 0 (NMPWAIT::USE_DEFAULT_WAIT) gets interpreted as "make WaitNamedPipe(..., NMPWAIT::USE_DEFAULT_WAIT) use the default timeout of 50ms"
    security_attributes:    impl Into<Option<&'a firehazard::security::Attributes<'b>>>,
) -> Result<firehazard::handle::Owned, firehazard::Error> {
    use crate::FromLocalHandle;
    use abistr::AsCStr;

    let handle = unsafe { winapi::um::namedpipeapi::CreateNamedPipeW(
        name.try_into()?.as_cstr(),
        open_mode,
        pipe_mode,
        max_instances.into().0,
        out_buffer_size,
        in_buffer_size,
        default_timeout.into().0,
        security_attributes.into().map_or(core::ptr::null(), |sa| sa) as *mut _,
    )};
    firehazard::Error::get_last_if(handle.is_null() || handle == winapi::um::handleapi::INVALID_HANDLE_VALUE)?;
    unsafe { firehazard::handle::Owned::from_raw(handle) }
}
