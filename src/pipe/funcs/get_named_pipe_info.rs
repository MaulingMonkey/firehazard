#[doc(alias = "GetNamedPipeInfo")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-getnamedpipeinfo)\]
/// GetNamedPipeInfo
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// #
/// let (read, write) = create_pipe(None, 0).unwrap();
/// let local = create_named_pipe_w(
///     cstr16!(r"\\.\pipe\local\firehazard_get_named_pipe_info_example"),
///     pipe::ACCESS_DUPLEX, 0, pipe::UNLIMITED_INSTANCES, 0, 0, None, None,
/// ).unwrap();
///
/// let mut flags = 0;
/// let mut out_buffer_size = 0;
/// let mut in_buffer_size = 0;
/// let mut max_instances = pipe::UNLIMITED_INSTANCES;
///
/// get_named_pipe_info(&read, &mut flags, &mut out_buffer_size, &mut in_buffer_size, &mut max_instances).unwrap();
/// assert_eq!(flags, pipe::SERVER_END | pipe::TYPE_BYTE);
/// dbg!(out_buffer_size); // 4 KiB?
/// dbg!( in_buffer_size); // 4 KiB?
/// assert_eq!(max_instances, pipe::MaxInstances::ONE);
///
/// get_named_pipe_info(&write, &mut flags, &mut out_buffer_size, &mut in_buffer_size, &mut max_instances).unwrap();
/// assert_eq!(flags, pipe::CLIENT_END | pipe::TYPE_BYTE);
/// dbg!(out_buffer_size); // 4 KiB?
/// dbg!( in_buffer_size); // 4 KiB?
/// assert_eq!(max_instances, pipe::MaxInstances::ONE);
///
/// get_named_pipe_info(&local, &mut flags, &mut out_buffer_size, &mut in_buffer_size, &mut max_instances).unwrap();
/// assert_eq!(flags, pipe::SERVER_END | pipe::TYPE_BYTE);
/// dbg!(out_buffer_size); // 0?
/// dbg!(in_buffer_size ); // 0?
/// assert_eq!(max_instances, pipe::UNLIMITED_INSTANCES);
///
/// // Pointless noop, but they do at least "succeed":
/// get_named_pipe_info(&read,  None, None, None, None).unwrap();
/// get_named_pipe_info(&write, None, None, None, None).unwrap();
/// get_named_pipe_info(&local, None, None, None, None).unwrap();
/// ```
///
pub fn get_named_pipe_info<'a, 'b, 'c, 'd>(
    handle:             &impl firehazard::AsLocalHandle,
    flags:              impl Into<Option<&'a mut u32>>,
    out_buffer_size:    impl Into<Option<&'b mut u32>>,
    in_buffer_size:     impl Into<Option<&'c mut u32>>,
    max_instances:      impl Into<Option<&'d mut firehazard::pipe::MaxInstances>>,
) -> Result<(), firehazard::Error> {
    use core::ptr::null_mut;

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::GetNamedPipeInfo(
        handle          .as_handle(),
        flags           .into().map_or(null_mut(), |r| r),
        out_buffer_size .into().map_or(null_mut(), |r| r),
        in_buffer_size  .into().map_or(null_mut(), |r| r),
        max_instances   .into().map_or(null_mut(), |mi| &mut mi.0),
    )})
}
