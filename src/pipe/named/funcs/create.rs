#[doc(no_inline)] pub use create_w as create;



#[doc(alias = "CreateNamedPipeA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createnamedpipea)\]
/// CreateNamedPipeA
///
/// Create a handle to listen for inbound connections to a named pipe.
///
///
///
/// ### Arguments
/// -   `name`                          &mdash; Name of the pipe to create for listening to, such as `\\.\pipe\example` (traditional) or `\\.\pipe\local\example` (AppContainer-friendly, inaccessible outside the current app's identifier.)
/// -   `open_mode`                     &mdash; Direction data flows, from the perspective of the server.
///     -   [`pipe::ACCESS_INBOUND`]        &mdash; client to server
///     -   [`pipe::ACCESS_OUTBOUND`]       &mdash; server to client
///     -   [`pipe::ACCESS_DUPLEX`]         &mdash; client to/from server
/// -   `pipe_mode`                     &mdash; Various flags
// TODO: more
/// -   `max_instances`                 &mdash; Limit how many server pipes can be created.  All calls must agree on a limit for a given name.
/// -   `out_buffer_size`               &mdash; Advisory/requested size, in bytes, for the server to client buffer size.  May be clamped by system limits / rounded up to page sizes.
/// -   `in_buffer_size`                &mdash; Advisory/requested size, in bytes, for the client to server buffer size.  May be clamped by system limits / rounded up to page sizes.
/// -   `default_timeout`               &mdash; How long [`pipe::named::wait`] should wait for an available pipe ([`NMPWAIT::USE_DEFAULT_WAIT`] → system default (50ms?) in this context.)
/// -   `security_attributes`           &mdash; Security attributes for the pipe.  If [`None`] is specified, the default ACL grants full control to LocalSystem, administrators, and the creator, **and read access to Everyone + Anonymous!**
///
///
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// #
/// let pipe = pipe::named::create_a(
///     cr#"\\.\pipe\local\firehazard-create_named_pipe_a-example"#,
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
///     cr#"\\.\pipe\local\firehazard-create_named_pipe_a-example-error-bad-max_instances"#,
///     pipe::ACCESS_DUPLEX, 0, pipe::MaxInstances::from_unchecked(0), 0, 0, None, None,
/// ).unwrap_err());
///
/// // An open_mode of 0 will result in ERROR_INVALID_PARAMETER
/// assert_eq!(ERROR_INVALID_PARAMETER, pipe::named::create_a(
///     cr#"\\.\pipe\local\firehazard-create_named_pipe_a-example-bad-open_mode"#,
///     0, 0, pipe::UNLIMITED_INSTANCES, 0, 0, None, None,
/// ).unwrap_err());
/// ```
///
pub fn create_a<'a, 'b: 'a>(
    name:                   impl string::InNarrow,
    open_mode:              u32, // TODO: type
    pipe_mode:              u32, // TODO: type
    max_instances:          impl Into<pipe::MaxInstances>,
    out_buffer_size:        u32,
    in_buffer_size:         u32,
    default_timeout:        impl Into<NMPWAIT>, // A slightly awkward fit, but 0 (NMPWAIT::USE_DEFAULT_WAIT) gets interpreted as "make WaitNamedPipe(..., NMPWAIT::USE_DEFAULT_WAIT) use the default timeout of 50ms"
    security_attributes:    impl Into<Option<&'a security::Attributes<'b>>>,
) -> firehazard::Result<pipe::named::Listener> {
    let max_instances       = max_instances.into().0;
    let default_timeout     = default_timeout.into().0;
    let security_attributes = security_attributes.into().map_or(null(), |sa| sa) as *mut _;

    string::convert_to_cstrnn::<{limit::stack::PIPE_NAME}, _, _>(name, |name| {
        let handle = unsafe { winapi::um::winbase::CreateNamedPipeA(
            name.as_cstr(), open_mode, pipe_mode, max_instances,
            out_buffer_size, in_buffer_size, default_timeout, security_attributes,
        )};
        firehazard::Error::get_last_if(handle.is_null() || handle == winapi::um::handleapi::INVALID_HANDLE_VALUE)?;
        unsafe { pipe::named::Listener::from_raw(handle) }
    })?
}



#[doc(alias = "CreateNamedPipe")]
#[doc(alias = "CreateNamedPipeW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\]
/// CreateNamedPipeW
///
/// Create a handle to listen for inbound connections to a named pipe.
///
///
///
/// ### Arguments
/// -   `name`                          &mdash; Name of the pipe to create for listening to, such as `\\.\pipe\example` (traditional) or `\\.\pipe\local\example` (AppContainer-friendly, inaccessible outside the current app's identifier.)
/// -   `open_mode`                     &mdash; Direction data flows, from the perspective of the server.
///     -   [`pipe::ACCESS_INBOUND`]        &mdash; client to server
///     -   [`pipe::ACCESS_OUTBOUND`]       &mdash; server to client
///     -   [`pipe::ACCESS_DUPLEX`]         &mdash; client to/from server
/// -   `pipe_mode`                     &mdash; Various flags
// TODO: more
/// -   `max_instances`                 &mdash; Limit how many server pipes can be created.  All calls must agree on a limit for a given name.
/// -   `out_buffer_size`               &mdash; Advisory/requested size, in bytes, for the server to client buffer size.  May be clamped by system limits / rounded up to page sizes.
/// -   `in_buffer_size`                &mdash; Advisory/requested size, in bytes, for the client to server buffer size.  May be clamped by system limits / rounded up to page sizes.
/// -   `default_timeout`               &mdash; How long [`pipe::named::wait`] should wait for an available pipe ([`NMPWAIT::USE_DEFAULT_WAIT`] → system default (50ms?) in this context.)
/// -   `security_attributes`           &mdash; Security attributes for the pipe.  If [`None`] is specified, the default ACL grants full control to LocalSystem, administrators, and the creator, **and read access to Everyone + Anonymous!**
///
///
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
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
    name:                   impl string::InWide,
    open_mode:              u32, // TODO: type
    pipe_mode:              u32, // TODO: type
    max_instances:          impl Into<pipe::MaxInstances>,
    out_buffer_size:        u32,
    in_buffer_size:         u32,
    default_timeout:        impl Into<NMPWAIT>, // A slightly awkward fit, but 0 (NMPWAIT::USE_DEFAULT_WAIT) gets interpreted as "make WaitNamedPipe(..., NMPWAIT::USE_DEFAULT_WAIT) use the default timeout of 50ms"
    security_attributes:    impl Into<Option<&'a security::Attributes<'b>>>,
) -> firehazard::Result<pipe::named::Listener> {
    let max_instances       = max_instances.into().0;
    let default_timeout     = default_timeout.into().0;
    let security_attributes = security_attributes.into().map_or(null(), |sa| sa) as *mut _;

    string::convert_to_cstrnn::<{limit::stack::PIPE_NAME}, _, _>(name, |name| {
        let handle = unsafe { winapi::um::namedpipeapi::CreateNamedPipeW(
            name.as_cstr(), open_mode, pipe_mode, max_instances,
            out_buffer_size, in_buffer_size, default_timeout, security_attributes,
        )};
        firehazard::Error::get_last_if(handle.is_null() || handle == winapi::um::handleapi::INVALID_HANDLE_VALUE)?;
        unsafe { pipe::named::Listener::from_raw(handle) }
    })?
}
