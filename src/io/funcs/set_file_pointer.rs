#[allow(non_camel_case_types)] #[doc(hidden)] #[allow(dead_code)] pub(crate) trait SetFilePointer_IntoSeekFrom   { fn into_lo_hi_method(self) -> (i32, Option<i32>, u32); }
#[allow(non_camel_case_types)] #[doc(hidden)] pub(crate) trait SetFilePointerEx_IntoSeekFrom { fn into_distance_method(self) -> (i64, u32); }
//impl SetFilePointer_IntoSeekFrom for i32   { fn into_lo_hi_method(self) -> (i32, Option<i32>, u32) { (self, None) } }
//impl SetFilePointer_IntoSeekFrom for i64   { fn into_lo_hi_method(self) -> (i32, Option<i32>, u32) { (self as i32, Some(self >> 32)) } }
//#[cfg(target_pointer_width = "32")] impl SetFilePointer_IntoSeekFrom for isize { fn into_lo_hi_method(self) -> (i32, Option<i32>, u32) { SetFilePointer_IntoSeekFrom::into_lo_hi_method(self as i32) } }
//#[cfg(target_pointer_width = "64")] impl SetFilePointer_IntoSeekFrom for isize { fn into_lo_hi_method(self) -> (i32, Option<i32>, u32) { SetFilePointer_IntoSeekFrom::into_lo_hi_method(self as i64) } }

impl SetFilePointer_IntoSeekFrom for crate::io::SeekFrom { fn into_lo_hi_method(self) -> (i32, Option<i32>, u32) {
    match self {
        crate::io::SeekFrom::Start(pos)     => (pos as _, Some((pos >> 32) as _), winapi::um::winbase::FILE_BEGIN),
        crate::io::SeekFrom::Current(pos)   => (pos as _, Some((pos >> 32) as _), winapi::um::winbase::FILE_CURRENT),
        crate::io::SeekFrom::End(pos)       => (pos as _, Some((pos >> 32) as _), winapi::um::winbase::FILE_END),
    }
}}

impl SetFilePointerEx_IntoSeekFrom for crate::io::SeekFrom { fn into_distance_method(self) -> (i64, u32) {
    match self {
        crate::io::SeekFrom::Start(pos)     => (pos as _, winapi::um::winbase::FILE_BEGIN   ),
        crate::io::SeekFrom::Current(pos)   => (pos as _, winapi::um::winbase::FILE_CURRENT ),
        crate::io::SeekFrom::End(pos)       => (pos as _, winapi::um::winbase::FILE_END     ),
    }
}}



#[doc(alias = "SetFilePointer")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfilepointer)\]
/// SetFilePointer
///
/// | `distance_method`                 | `method`      |
/// | ----------------------------------| --------------|
/// | [`std::io::SeekFrom::Start`]      | FILE_BEGIN    |
/// | [`std::io::SeekFrom::Current`]    | FILE_CURRENT  |
/// | [`std::io::SeekFrom::End`]        | FILE_END      |
///
/// ### Alternatives
/// *   [`std::io::Seek::seek`]             &mdash; cross platform, requires std
/// *   [`firehazard::set_file_pointer_ex`] &mdash; fewer fiddly edge cases
///
// TODO: Errors
#[allow(dead_code)]
pub(crate) unsafe fn set_file_pointer(handle: &impl firehazard::AsLocalHandle, distance_method: impl SetFilePointer_IntoSeekFrom) -> Result<u64, firehazard::Error> {
    let (lo, mut hi, method) = distance_method.into_lo_hi_method();

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::SetFilePointer(
        handle.as_handle().cast(),
        lo,
        hi.as_mut().map_or(core::ptr::null_mut(), |hi| hi), // in + out
        method,
    )})?;

    Ok(((hi.unwrap_or(0) as u32 as u64) << 32) | (lo as u32 as u64))
}



#[doc(alias = "SetFilePointerEx")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfilepointerex)\]
/// SetFilePointerEx
///
/// | `distance_method`                 | `method`      |
/// | ----------------------------------| --------------|
/// | [`std::io::SeekFrom::Start`]      | FILE_BEGIN    |
/// | [`std::io::SeekFrom::Current`]    | FILE_CURRENT  |
/// | [`std::io::SeekFrom::End`]        | FILE_END      |
///
/// ### Alternatives
/// *   [`std::io::Seek::seek`]             &mdash; cross platform, requires std
/// *   [`firehazard::set_file_pointer`]    &mdash; fiddly edge cases
///
// TODO: Errors
pub(crate) unsafe fn set_file_pointer_ex(handle: &impl firehazard::AsLocalHandle, distance_method: impl SetFilePointerEx_IntoSeekFrom) -> Result<u64, firehazard::Error> {
    let (distance, method) = distance_method.into_distance_method();
    let mut new_file_pointer : u64 = 0;

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::SetFilePointerEx(
        handle.as_handle().cast(),
        core::mem::transmute::<i64, winapi::um::winnt::LARGE_INTEGER>(distance),
        core::ptr::from_mut(&mut new_file_pointer).cast(),
        method,
    )})?;

    Ok(new_file_pointer)
}
