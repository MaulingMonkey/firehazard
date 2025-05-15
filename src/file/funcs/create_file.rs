#[doc(no_inline)] pub use create_file_w as create_file;



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
///     c"Readme.md",
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
    name:                   impl string::InNarrow,
    desired_access:         impl Into<access::Mask>,
    share_mode:             impl Into<file::Share>,
    security_attributes:    impl Into<Option<&'a security::Attributes<'b>>>,
    creation_disposition:   u32, // XXX
    flags_and_attributes:   u32, // XXX
    template_file:          impl Into<Option<io::sync::BorrowedFile<'t>>>,
) -> firehazard::Result<handle::Owned> {
    let desired_access      = desired_access.into().into();
    let share_mode          = share_mode.into().into();
    let security_attributes = security_attributes.into().map_or(null(), |a| a) as *mut _;
    let template_file       = template_file.into().map_or(null_mut(), |h| h.as_handle().cast());

    string::convert_to_cstrnn::<{limit::stack::PATH}, _, _>(name, |name| {
        let handle = NonNull::new(unsafe { winapi::um::fileapi::CreateFileA(
            name.as_cstr(), desired_access, share_mode, security_attributes,
            creation_disposition, flags_and_attributes, template_file,
        )}).ok_or_else(|| firehazard::Error::get_last())?;
        Ok(unsafe { handle::Owned::from_raw_nn(handle.cast()) })
    })?
}



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
    name:                   impl string::InWide,
    desired_access:         impl Into<access::Mask>,
    share_mode:             impl Into<file::Share>,
    security_attributes:    impl Into<Option<&'a security::Attributes<'b>>>,
    creation_disposition:   u32, // XXX
    flags_and_attributes:   u32, // XXX
    template_file:          impl Into<Option<io::sync::BorrowedFile<'t>>>,
) -> firehazard::Result<handle::Owned> {
    let desired_access      = desired_access.into().into();
    let share_mode          = share_mode.into().into();
    let security_attributes = security_attributes.into().map_or(null(), |a| a) as *mut _;
    let template_file       = template_file.into().map_or(null_mut(), |h| h.as_handle().cast());

    string::convert_to_cstrnn::<{limit::stack::PATH}, _, _>(name, |name| {
        let handle = NonNull::new(unsafe { winapi::um::fileapi::CreateFileW(
            name.as_cstr(), desired_access, share_mode, security_attributes,
            creation_disposition, flags_and_attributes, template_file,
        )}).ok_or_else(|| firehazard::Error::get_last())?;
        Ok(unsafe { handle::Owned::from_raw_nn(handle.cast()) })
    })?
}
