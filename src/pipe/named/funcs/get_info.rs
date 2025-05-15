#[doc(alias = "GetNamedPipeInfo")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-getnamedpipeinfo)\]
/// GetNamedPipeInfo
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
/// #
/// let (read, write) = pipe::create(None, 0).unwrap();
/// let local = pipe::named::create_w(
///     cstr16!(r"\\.\pipe\local\firehazard_get_named_pipe_info_example_bytes"),
///     pipe::ACCESS_DUPLEX, 0, pipe::UNLIMITED_INSTANCES, 0, 0, None, None,
/// ).unwrap();
/// let messages = pipe::named::create_w(
///     cstr16!(r"\\.\pipe\local\firehazard_get_named_pipe_info_example_messages"),
///     pipe::ACCESS_DUPLEX, pipe::TYPE_MESSAGE | pipe::READMODE_MESSAGE, pipe::UNLIMITED_INSTANCES, 0, 0, None, None,
/// ).unwrap();
///
/// let mut flags = 0;
/// let mut out_buffer_size = 0;
/// let mut in_buffer_size = 0;
/// let mut max_instances = pipe::UNLIMITED_INSTANCES;
///
/// pipe::named::get_info(&read, &mut flags, &mut out_buffer_size, &mut in_buffer_size, &mut max_instances).unwrap();
/// assert_eq!(flags, pipe::SERVER_END | pipe::TYPE_BYTE);
/// dbg!(out_buffer_size); // 4 KiB?
/// dbg!( in_buffer_size); // 4 KiB?
/// assert_eq!(max_instances, pipe::MaxInstances::ONE);
///
/// pipe::named::get_info(&write, &mut flags, &mut out_buffer_size, &mut in_buffer_size, &mut max_instances).unwrap();
/// assert_eq!(flags, pipe::CLIENT_END | pipe::TYPE_BYTE);
/// dbg!(out_buffer_size); // 4 KiB?
/// dbg!( in_buffer_size); // 4 KiB?
/// assert_eq!(max_instances, pipe::MaxInstances::ONE);
///
/// pipe::named::get_info(&local, &mut flags, &mut out_buffer_size, &mut in_buffer_size, &mut max_instances).unwrap();
/// assert_eq!(flags, pipe::SERVER_END | pipe::TYPE_BYTE);
/// dbg!(out_buffer_size); // 0?
/// dbg!(in_buffer_size ); // 0?
/// assert_eq!(max_instances, pipe::UNLIMITED_INSTANCES);
///
/// pipe::named::get_info(&messages, &mut flags, &mut out_buffer_size, &mut in_buffer_size, &mut max_instances).unwrap();
/// assert_eq!(flags, pipe::SERVER_END | pipe::TYPE_MESSAGE); // n.b. no pipe::READMODE_MESSAGE
/// dbg!(out_buffer_size); // 0?
/// dbg!(in_buffer_size ); // 0?
/// assert_eq!(max_instances, pipe::UNLIMITED_INSTANCES);
///
/// // Pointless noop, but they do at least "succeed":
/// pipe::named::get_info(&read,     None, None, None, None).unwrap();
/// pipe::named::get_info(&write,    None, None, None, None).unwrap();
/// pipe::named::get_info(&local,    None, None, None, None).unwrap();
/// pipe::named::get_info(&messages, None, None, None, None).unwrap();
/// ```
///
pub fn get_info<'a, 'b, 'c, 'd>(
    handle:             &impl AsLocalHandle,
    flags:              impl Into<Option<&'a mut u32>>,
    out_buffer_size:    impl Into<Option<&'b mut u32>>,
    in_buffer_size:     impl Into<Option<&'c mut u32>>,
    max_instances:      impl Into<Option<&'d mut pipe::MaxInstances>>,
) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::GetNamedPipeInfo(
        handle          .as_handle(),
        flags           .into().map_or(null_mut(), |r| r),
        out_buffer_size .into().map_or(null_mut(), |r| r),
        in_buffer_size  .into().map_or(null_mut(), |r| r),
        max_instances   .into().map_or(null_mut(), |mi| &mut mi.0),
    )})
}
