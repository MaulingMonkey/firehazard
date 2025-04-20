#[doc(alias = "GetNamedPipeHandleStateA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getnamedpipehandlestatea)\]
/// GetNamedPipeHandleStateA
///
pub fn get_handle_state_a<'a, 'b, 'c, 'd, 'e>(
    handle:                     &impl AsLocalHandle, // XXX
    state:                      impl Into<Option<&'a mut u32>>,
    current_instances:          impl Into<Option<&'b mut u32>>,
    max_collection_count:       impl Into<Option<&'c mut u32>>,
    collect_data_timeout_ms:    impl Into<Option<&'d mut u32>>,
    user_name:                  impl Into<Option<&'e mut [u8]>>,
) -> firehazard::Result<()> {
    let user_name = user_name.into();
    let user_name_len = user_name.as_ref().map_or(0, |un| un.len());

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::winbase::GetNamedPipeHandleStateA(
        handle                  .as_handle(),
        state                   .into().map_or(null_mut(), |v| v),
        current_instances       .into().map_or(null_mut(), |v| v),
        max_collection_count    .into().map_or(null_mut(), |v| v),
        collect_data_timeout_ms .into().map_or(null_mut(), |v| v),
        user_name               .map_or(null_mut(), |v| v.as_mut_ptr().cast()),
        user_name_len           .try_into().unwrap_or(!0),
    )})
}



#[doc(alias = "GetNamedPipeHandleState")]
#[doc(alias = "GetNamedPipeHandleStateW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-getnamedpipehandlestatew)\]
/// GetNamedPipeHandleStateW
///
pub fn get_handle_state_w<'a, 'b, 'c, 'd, 'e>(
    handle:                     &impl AsLocalHandle, // XXX
    state:                      impl Into<Option<&'a mut u32>>,
    current_instances:          impl Into<Option<&'b mut u32>>,
    max_collection_count:       impl Into<Option<&'c mut u32>>,
    collect_data_timeout_ms:    impl Into<Option<&'d mut u32>>,
    user_name:                  impl Into<Option<&'e mut [u16]>>,
) -> firehazard::Result<()> {
    let user_name = user_name.into();
    let user_name_len = user_name.as_ref().map_or(0, |un| un.len());

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::GetNamedPipeHandleStateW(
        handle                  .as_handle(),
        state                   .into().map_or(null_mut(), |v| v),
        current_instances       .into().map_or(null_mut(), |v| v),
        max_collection_count    .into().map_or(null_mut(), |v| v),
        collect_data_timeout_ms .into().map_or(null_mut(), |v| v),
        user_name               .map_or(null_mut(), |v| v.as_mut_ptr().cast()),
        user_name_len           .try_into().unwrap_or(!0),
    )})
}
