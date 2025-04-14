#[doc(alias = "GetFinalPathNameByHandleA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlea)\]
/// GetFinalPathNameByHandleA
///
pub fn get_final_path_name_by_handle_a_inplace<'path>(
    handle: &impl firehazard::AsLocalHandle,
    path:   &'path mut [core::mem::MaybeUninit<u8>],
    flags:  u32, // TODO: type
) -> Result<&'path mut [u8], firehazard::Error> {
    use crate::From32;
    let buf_chars = path.len().try_into().unwrap_or(!0_u32);
    let full_chars = usize::from32(unsafe { winapi::um::fileapi::GetFinalPathNameByHandleA(
        handle.as_handle().cast(),
        path.as_mut_ptr().cast(),
        buf_chars,
        flags
    )});
    firehazard::Error::get_last_if(full_chars == 0 || full_chars > path.len())?;
    Ok(unsafe { crate::slice_assume_init_mut(&mut path[..full_chars]) })
}



#[doc(alias = "GetFinalPathNameByHandleW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// GetFinalPathNameByHandleW
///
pub fn get_final_path_name_by_handle_w_inplace<'path>(
    handle: &impl firehazard::AsLocalHandle,
    path:   &'path mut [core::mem::MaybeUninit<u16>],
    flags:  u32, // TODO: type
) -> Result<&'path mut [u16], firehazard::Error> {
    use crate::From32;
    let buf_chars = path.len().try_into().unwrap_or(!0_u32);
    let full_chars = usize::from32(unsafe { winapi::um::fileapi::GetFinalPathNameByHandleW(
        handle.as_handle().cast(),
        path.as_mut_ptr().cast(),
        buf_chars,
        flags
    )});
    firehazard::Error::get_last_if(full_chars == 0 || full_chars > path.len())?;
    Ok(unsafe { crate::slice_assume_init_mut(&mut path[..full_chars]) })
}



#[doc(alias = "GetFinalPathNameByHandle")]
#[doc(alias = "GetFinalPathNameByHandleW")]
#[cfg(std)]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// GetFinalPathNameByHandleW
///
/// ### Alternatives
/// *   [`std::fs::canonicalize`] &mdash; cross platform, requires std
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use std::path::*;
/// #
/// let file = std::fs::File::open("Readme.md").unwrap();
///
/// # if false {
/// assert_eq!(
///     get_final_path_name_by_handle(&file, volume::NAME_DOS ).as_deref(),
///     Ok(Path::new(r"\\?\C:\local\firehazard\Readme.md"))
/// );
///
/// # #[cfg(nope)] // GUID scrubbed
/// assert_eq!(
///     get_final_path_name_by_handle(&file, volume::NAME_GUID).as_deref().unwrap(),
///     Path::new(r"\\?\Volume{12345678-1234-1234-1234-123456789abc}\local\firehazard\Readme.md")
/// );
///
/// assert_eq!(
///     get_final_path_name_by_handle(&file, volume::NAME_NT  ).as_deref(),
///     Ok(Path::new(r"\Device\HarddiskVolume4\local\firehazard\Readme.md"))
/// );
///
/// assert_eq!(
///     get_final_path_name_by_handle(&file, volume::NAME_NONE).as_deref(),
///     Ok(Path::new(r"\local\firehazard\Readme.md"))
/// );
/// #
/// # }
/// ```
///
/// ### Exceptions
/// *   `STATUS_INVALID_HANDLE`             &mdash; if strict handle checks are enabled and `handle` is bad/dangling?
///
/// ### Errors
/// *   `ERROR_BAD_PATHNAME`                &mdash; if `handle` is an anonymous pipe
/// *   `ERROR_INVALID_HANDLE`              &mdash; if `handle` is a job object
/// *   `ERROR_INVALID_HANDLE`              &mdash; if `handle` is null/INVALID_HANDLE_VALUE/bad/dangling
/// *   `ERROR_INVALID_PARAMETER`           &mdash; if `flags` is invalid
///
pub fn get_final_path_name_by_handle(handle: &impl firehazard::AsLocalHandle, flags: u32) -> Result<std::path::PathBuf, firehazard::Error> {
    use crate::From32;
    use std::ffi::OsString;
    use std::os::windows::prelude::OsStringExt;
    use std::path::PathBuf;
    use core::mem::MaybeUninit;

    let mut buf = [MaybeUninit::uninit(); 260];
    let full_chars = usize::from32(unsafe { winapi::um::fileapi::GetFinalPathNameByHandleW(
        handle.as_handle().cast(),
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



tests! {
    #[test] fn get_final_path_name_by_handle_null() {
        assert_eq!(
            get_final_path_name_by_handle(&crate::handle::invalid::null(), 0),
            Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)),
        );
    }

    #[test] fn get_final_path_name_by_handle_invalid_handle_value() {
        assert_eq!(
            get_final_path_name_by_handle(&crate::handle::invalid::invalid_value(), 0),
            Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)),
        );
    }

    #[test] #[isolate] fn get_final_path_name_by_handle_bad_handle_value() {
        assert_eq!(
            get_final_path_name_by_handle(&crate::handle::invalid::never_valid(), 0),
            Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)),
        );
    }

    #[test] #[isolate] fn get_final_path_name_by_handle_dangling() {
        assert_eq!(
            get_final_path_name_by_handle(&crate::handle::invalid::dangling(), 0),
            Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)),
        );
    }

    #[test] fn get_final_path_name_by_handle_bad_flags() {
        let file = std::fs::File::open("Readme.md").unwrap();
        get_final_path_name_by_handle(&file, 0).unwrap(); // good flags
        assert_eq!(
            get_final_path_name_by_handle(&file, !0),
            Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_PARAMETER))
        );
    }

    #[test] fn get_final_path_name_by_handle_anonymous_pipe() {
        let (read, write) = firehazard::create_pipe(None, 0).unwrap();

        for flags in [0, 1, 2, 4, 8, 8|1, 8|2, 8|4] {
            assert_eq!(
                get_final_path_name_by_handle(&read, flags),
                Err(firehazard::Error(winapi::shared::winerror::ERROR_BAD_PATHNAME)),
            );

            assert_eq!(
                get_final_path_name_by_handle(&write, flags),
                Err(firehazard::Error(winapi::shared::winerror::ERROR_BAD_PATHNAME)),
            );
        }
    }

    #[test] fn get_final_path_name_by_handle_job() {
        use firehazard::abistr::cstr16;
        let anon_job  = firehazard::create_job_object_w(None, ()).unwrap();
        let named_job = firehazard::create_job_object_w(None, cstr16!("Local/firehazard/get_final_path_name_by_handle_job")).unwrap();

        for job in [&anon_job, &named_job] {
            for flags in [0, 1, 2, 4, 8, 8|1, 8|2, 8|4] {
                assert_eq!(
                    get_final_path_name_by_handle(job, flags),
                    Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)),
                );
            }
        }
    }
}
