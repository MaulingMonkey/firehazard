/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlea)\]
/// GetFinalPathNameByHandleA
///
pub fn get_final_path_name_by_handle_a_inplace<'a>(
    handle: impl AsRef<firehazard::io::FileHandle<'a>>,
    path:   &mut [core::mem::MaybeUninit<u8>],
    flags:  u32, // TODO: type
) -> Result<&mut [u8], firehazard::Error> {
    use crate::{AsLocalHandle, From32};
    let buf_chars = path.len().try_into().unwrap_or(!0_u32);
    let full_chars = usize::from32(unsafe { winapi::um::fileapi::GetFinalPathNameByHandleA(
        handle.as_ref().as_handle(),
        path.as_mut_ptr().cast(),
        buf_chars,
        flags
    )});
    firehazard::Error::get_last_if(full_chars == 0 || full_chars > path.len())?;
    Ok(unsafe { crate::slice_assume_init_mut(&mut path[..full_chars]) })
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlea)\]
/// GetFinalPathNameByHandleW
///
pub fn get_final_path_name_by_handle_w_inplace<'a>(
    handle: impl AsRef<firehazard::io::FileHandle<'a>>,
    path:   &mut [core::mem::MaybeUninit<u16>],
    flags:  u32, // TODO: type
) -> Result<&mut [u16], firehazard::Error> {
    use crate::{AsLocalHandle, From32};
    let buf_chars = path.len().try_into().unwrap_or(!0_u32);
    let full_chars = usize::from32(unsafe { winapi::um::fileapi::GetFinalPathNameByHandleW(
        handle.as_ref().as_handle(),
        path.as_mut_ptr().cast(),
        buf_chars,
        flags
    )});
    firehazard::Error::get_last_if(full_chars == 0 || full_chars > path.len())?;
    Ok(unsafe { crate::slice_assume_init_mut(&mut path[..full_chars]) })
}



#[cfg(std)]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlea)\]
/// GetFinalPathNameByHandleW
///
pub fn get_final_path_name_by_handle<'a>(handle: impl AsRef<firehazard::io::FileHandle<'a>>, flags: u32) -> Result<std::path::PathBuf, firehazard::Error> {
    use crate::{AsLocalHandle, From32};
    use std::ffi::OsString;
    use std::os::windows::prelude::OsStringExt;
    use std::path::PathBuf;
    use core::mem::MaybeUninit;

    let handle = handle.as_ref();

    let mut buf = [MaybeUninit::uninit(); 260];
    let full_chars = usize::from32(unsafe { winapi::um::fileapi::GetFinalPathNameByHandleW(
        handle.as_handle(),
        buf.as_mut_ptr().cast(),
        buf.len() as _,
        flags
    )});
    firehazard::Error::get_last_if(full_chars == 0)?;
    if let Some(buf) = buf.get_mut(..full_chars) { return Ok(PathBuf::from(OsString::from_wide(unsafe { crate::slice_assume_init_mut(buf) }))) }

    // else `buf` was too small:
    let mut buf = std::vec![MaybeUninit::uninit(); full_chars+1];
    let path = get_final_path_name_by_handle_w_inplace(handle, &mut buf[..], flags)?;
    Ok(PathBuf::from(OsString::from_wide(path)))
}
