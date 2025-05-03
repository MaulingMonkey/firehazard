#[doc(alias = "GetFileAttributesA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesa)\]
/// GetFileAttributesA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use winapi::um::winnt::FILE_ATTRIBUTE_ARCHIVE;
/// let attributes = get_file_attributes_a(abistr::cstr!("Readme.md")).unwrap();
/// # return; // likely, but technically not guaranteed:
/// assert!(attributes & FILE_ATTRIBUTE_ARCHIVE != 0, "file is marked as 'ready for archiving'");
/// ```
///
/// ### Errata: Named Pipes
///
/// This (currently) works by (attempting to) open a handle to `path` under the hood,
/// which may cause some suprising results if used on single-shot named pipes:
///
/// -   [Calling GetFileAttributesW() removes a pipe](https://stackoverflow.com/questions/28769237/calling-getfileattributesw-removes-a-pipe) (stackoverflow.com)
/// -   [`dotnet/runtime#69604`: Using File.Exists to check the pipe created will make the NamedPipeClientStream connect fail](https://github.com/dotnet/runtime/issues/69604) (github.com)
///
/// To avoid these problems, as well as possible [TOC/TOU](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use) bugs, instead consider:
///
/// -   Calling [`CreateFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew) and handling the error (if you wish to connect to a pipe only if it exists.)
/// -   Calling [`pipe::named::create`] or similar with e.g. [`file::FLAG_FIRST_PIPE_INSTANCE`] (if you wish to avoid establishing a server pipe if it already exists.)
///
pub fn get_file_attributes_a(path: impl TryIntoAsCStr) -> firehazard::Result<u32> {
    let attributes = unsafe { winapi::um::fileapi::GetFileAttributesA(
        path.try_into()?.as_cstr()
    )};
    firehazard::Error::get_last_if(attributes == winapi::um::fileapi::INVALID_FILE_ATTRIBUTES)?; // -1
    Ok(attributes)
}

#[doc(alias = "GetFileAttributesW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesw)\]
/// GetFileAttributesW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use winapi::um::winnt::FILE_ATTRIBUTE_ARCHIVE;
/// let attributes = get_file_attributes_w(abistr::cstr16!("Readme.md")).unwrap();
/// # return; // likely, but technically not guaranteed:
/// assert!(attributes & FILE_ATTRIBUTE_ARCHIVE != 0, "file is marked as 'ready for archiving'");
/// ```
///
/// ### Errata: Named Pipes
///
/// This (currently) works by (attempting to) open a handle to `path` under the hood,
/// which may cause some suprising results if used on single-shot named pipes:
///
/// -   [Calling GetFileAttributesW() removes a pipe](https://stackoverflow.com/questions/28769237/calling-getfileattributesw-removes-a-pipe) (stackoverflow.com)
/// -   [`dotnet/runtime#69604`: Using File.Exists to check the pipe created will make the NamedPipeClientStream connect fail](https://github.com/dotnet/runtime/issues/69604) (github.com)
///
/// To avoid these problems, as well as possible [TOC/TOU](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use) bugs, instead consider:
///
/// -   Calling [`CreateFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew) and handling the error (if you wish to connect to a pipe only if it exists.)
/// -   Calling [`pipe::named::create`] or similar with e.g. [`file::FLAG_FIRST_PIPE_INSTANCE`] (if you wish to avoid establishing a server pipe if it already exists.)
///
pub fn get_file_attributes_w(path: impl TryIntoAsCStr<u16>) -> firehazard::Result<u32> {
    let attributes = unsafe { winapi::um::fileapi::GetFileAttributesW(
        path.try_into()?.as_cstr()
    )};
    firehazard::Error::get_last_if(attributes == winapi::um::fileapi::INVALID_FILE_ATTRIBUTES)?; // -1
    Ok(attributes)
}

#[cfg(std)]
#[doc(alias = "GetFileAttributes")]
#[doc(alias = "GetFileAttributesW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesw)\]
/// GetFileAttributesW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use winapi::um::winnt::FILE_ATTRIBUTE_ARCHIVE;
/// let attributes = get_file_attributes("Readme.md").unwrap();
/// # return; // likely, but technically not guaranteed:
/// assert!(attributes & FILE_ATTRIBUTE_ARCHIVE != 0, "file is marked as 'ready for archiving'");
/// ```
///
/// ### Errata: Named Pipes
///
/// This (currently) works by (attempting to) open a handle to `path` under the hood,
/// which may cause some suprising results if used on single-shot named pipes:
///
/// -   [Calling GetFileAttributesW() removes a pipe](https://stackoverflow.com/questions/28769237/calling-getfileattributesw-removes-a-pipe) (stackoverflow.com)
/// -   [`dotnet/runtime#69604`: Using File.Exists to check the pipe created will make the NamedPipeClientStream connect fail](https://github.com/dotnet/runtime/issues/69604) (github.com)
///
/// To avoid these problems, as well as possible [TOC/TOU](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use) bugs, instead consider:
///
/// -   Calling [`CreateFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew) and handling the error (if you wish to connect to a pipe only if it exists.)
/// -   Calling [`pipe::named::create`] or similar with e.g. [`file::FLAG_FIRST_PIPE_INSTANCE`] (if you wish to avoid establishing a server pipe if it already exists.)
///
pub fn get_file_attributes(path: impl AsRef<std::path::Path>) -> firehazard::Result<u32> {
    get_file_attributes_w(osstr_to_wide0(path.as_ref().as_os_str(), &mut std::vec::Vec::new())?)
}
