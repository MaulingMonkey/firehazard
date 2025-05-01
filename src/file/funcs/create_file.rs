#[doc(alias = "CreateFileA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\]
/// CreateFileA
///
/// Create a file handle (possibly by opening an existing file!)
///
/// ### Alternatives
/// *   <code>std::[fs](std::fs)::[File](std::fs::File)::{[create](std::fs::File::create), [create_new](std::fs::File::create_new), [open](std::fs::File::open)}</code>
///     &mdash; cross platform, requires [`std`]
/// *   <code>std::[fs](std::fs)::[OpenOptions](std::fs::OpenOptions)::[open](std::fs::OpenOptions::open)</code>
///     &mdash; cross platform, requires [`std`]
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use winapi::um::fileapi::OPEN_EXISTING;
/// #
/// let readme = create_file_a(
///     abistr::cstr!("Readme.md"),
///     access::GENERIC_READ,
///     file::Share::READ,
///     None,
///     OPEN_EXISTING,
///     0,
///     None,
/// ).unwrap();
/// ```
///
pub fn create_file_a<'a, 'b: 'a, 't>(
    name:                   impl TryIntoAsCStr,
    desired_access:         impl Into<access::Mask>,
    share_mode:             impl Into<file::Share>,
    security_attributes:    impl Into<Option<&'a security::Attributes<'b>>>,
    creation_disposition:   u32, // XXX
    flags_and_attributes:   u32, // XXX
    template_file:          impl Into<Option<io::sync::BorrowedFile<'t>>>,
) -> firehazard::Result<handle::Owned> {
    let handle = NonNull::new(unsafe { winapi::um::fileapi::CreateFileA(
        name.try_into()?.as_cstr(),
        desired_access.into().into(),
        share_mode.into().into(),
        security_attributes.into().map_or(null(), |a| a) as *mut _,
        creation_disposition,
        flags_and_attributes,
        template_file.into().map_or(null_mut(), |h| h.as_handle().cast()),
    )}).ok_or_else(|| firehazard::Error::get_last())?;

    Ok(unsafe { handle::Owned::from_raw_nn(handle.cast()) })
}

#[doc(alias = "CreateFileW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)\]
/// CreateFileW
///
/// Create a file handle (possibly by opening an existing file!)
///
/// ### Alternatives
/// *   <code>std::[fs](std::fs)::[File](std::fs::File)::{[create](std::fs::File::create), [create_new](std::fs::File::create_new), [open](std::fs::File::open)}</code>
///     &mdash; cross platform, requires [`std`]
/// *   <code>std::[fs](std::fs)::[OpenOptions](std::fs::OpenOptions)::[open](std::fs::OpenOptions::open)</code>
///     &mdash; cross platform, requires [`std`]
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use winapi::um::fileapi::OPEN_EXISTING;
/// #
/// let readme = create_file_w(
///     abistr::cstr16!("Readme.md"),
///     access::GENERIC_READ,
///     file::Share::READ,
///     None,
///     OPEN_EXISTING,
///     0,
///     None,
/// ).unwrap();
/// ```
///
///
pub fn create_file_w<'a, 'b: 'a, 't>(
    name:                   impl TryIntoAsCStr<u16>,
    desired_access:         impl Into<access::Mask>,
    share_mode:             impl Into<file::Share>,
    security_attributes:    impl Into<Option<&'a security::Attributes<'b>>>,
    creation_disposition:   u32, // XXX
    flags_and_attributes:   u32, // XXX
    template_file:          impl Into<Option<io::sync::BorrowedFile<'t>>>,
) -> firehazard::Result<handle::Owned> {
    let handle = NonNull::new(unsafe { winapi::um::fileapi::CreateFileW(
        name.try_into()?.as_cstr(),
        desired_access.into().into(),
        share_mode.into().into(),
        security_attributes.into().map_or(null(), |a| a) as *mut _,
        creation_disposition,
        flags_and_attributes,
        template_file.into().map_or(null_mut(), |h| h.as_handle().cast()),
    )}).ok_or_else(|| firehazard::Error::get_last())?;

    Ok(unsafe { handle::Owned::from_raw_nn(handle.cast()) })
}

#[cfg(std)]
#[doc(alias = "CreateFile")]
#[doc(alias = "CreateFileW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)\]
/// CreateFileW
///
/// Create a file handle (possibly by opening an existing file!)
///
/// ### Alternatives
/// *   <code>std::[fs](std::fs)::[File](std::fs::File)::{[create](std::fs::File::create), [create_new](std::fs::File::create_new), [open](std::fs::File::open)}</code>
///     &mdash; cross platform, requires [`std`]
/// *   <code>std::[fs](std::fs)::[OpenOptions](std::fs::OpenOptions)::[open](std::fs::OpenOptions::open)</code>
///     &mdash; cross platform, requires [`std`]
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use winapi::um::fileapi::OPEN_EXISTING;
/// #
/// let readme = create_file(
///     "Readme.md",
///     access::GENERIC_READ,
///     file::Share::READ,
///     None,
///     OPEN_EXISTING,
///     0,
///     None,
/// ).unwrap();
/// ```
///
pub fn create_file<'a, 'b: 'a, 't>(
    name:                   impl AsRef<std::path::Path>,
    desired_access:         impl Into<access::Mask>,
    share_mode:             impl Into<file::Share>,
    security_attributes:    impl Into<Option<&'a security::Attributes<'b>>>,
    creation_disposition:   u32, // XXX
    flags_and_attributes:   u32, // XXX
    template_file:          impl Into<Option<io::sync::BorrowedFile<'t>>>,
) -> firehazard::Result<handle::Owned> {
    create_file_w(
        osstr_to_wide0(name.as_ref().as_os_str(), &mut std::vec::Vec::new())?,
        desired_access,
        share_mode,
        security_attributes,
        creation_disposition,
        flags_and_attributes,
        template_file,
    )
}
