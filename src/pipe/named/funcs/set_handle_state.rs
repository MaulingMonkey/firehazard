#[doc(alias = "SetNamedPipeHandleState")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-setnamedpipehandlestate)\]
/// SetNamedPipeHandleState
///
pub fn set_handle_state(
    handle:                     &impl AsLocalHandle, // XXX
    mode:                       impl Into<Option<u32>>,
    max_collection_count:       impl Into<Option<u32>>,
    collect_data_timeout_ms:    impl Into<Option<u32>>,
) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::SetNamedPipeHandleState(
        handle                  .as_handle(),
        mode                    .into().as_mut().map_or(null_mut(), |v| v),
        max_collection_count    .into().as_mut().map_or(null_mut(), |v| v),
        collect_data_timeout_ms .into().as_mut().map_or(null_mut(), |v| v),
    )})
}
