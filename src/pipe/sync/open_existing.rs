#[doc(alias = "CreateFile")]
#[doc(alias = "CreateFileW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)\]
/// CreateFileW(name, GENERIC_READ | GENERIC_WRITE, 0, nullptr, OPEN_EXISTING, 0, nullptr)
///
/// Opens or connects to an existing file or pipe as a client.
///
/// ### Alternatives
///
/// If `name` refers to...
/// *   something that might be a file, seriously reconsider using [`OwnedDuplex`] - read and write share seek state
/// *   a pipe you wish to *listen to for connections* (as a *server*), instead consider [`pipe::named::create_w`]
///
pub fn open_duplex_existing<'a, 'b: 'a>(
    name:                   impl string::InWide,
) -> firehazard::Result<OwnedDuplex> {
    Ok(OwnedDuplex(open_existing(name, access::GENERIC_READ | access::GENERIC_WRITE, file::Share::NONE)?.into_handle_nn()))
}



#[doc(alias = "CreateFile")]
#[doc(alias = "CreateFileW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)\]
/// CreateFileW(name, GENERIC_READ, FILE_SHARE_READ, nullptr, OPEN_EXISTING, 0, nullptr)
///
/// Opens or connects to an existing file or pipe as a client.
///
/// ### Alternatives
///
/// If `name` refers to...
/// *   a pipe you wish to *listen to for connections* (as a *server*), instead consider [`pipe::named::create_w`]
///
pub fn read_existing<'a, 'b: 'a>(
    name:                   impl string::InWide,
) -> firehazard::Result<OwnedReader> {
    Ok(OwnedReader(open_existing(name, access::GENERIC_READ, file::Share::READ)?.into_handle_nn()))
}



#[doc(alias = "CreateFile")]
#[doc(alias = "CreateFileW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)\]
/// CreateFileW(name, FILE_READ_ATTRIBUTES | GENERIC_WRITE, 0, nullptr, OPEN_EXISTING, 0, nullptr)
///
/// Opens or connects to an existing pipe as a client.
///
/// ### Alternatives
///
/// If `name` refers to...
/// *   a possibly non-existant *file* that you want to *create if missing*, instead consider [`create_file_w`]
/// *   a pipe you wish to *listen to for connections* (as a *server*), instead consider [`pipe::named::create_w`]
/// *   a console handle you wish to share with other processes, consider [`pipe::named::create_w`] with [`file::Share::WRITE`]
///
pub fn write_existing<'a, 'b: 'a>(
    name:                   impl string::InWide,
) -> firehazard::Result<OwnedWriter> {
    Ok(OwnedWriter(open_existing(
        name,
        unsafe { access::Mask::from_unchecked(winapi::um::winnt::FILE_READ_ATTRIBUTES) } | access::GENERIC_WRITE,
        file::Share::NONE
    )?.into_handle_nn()))
}



#[doc(alias = "CreateFile")]
#[doc(alias = "CreateFileW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)\]
/// CreateFileW(name, access, share, nullptr, OPEN_EXISTING, 0, nullptr)
///
/// Opens or connects to an existing file or pipe as a client.
///
pub(crate) fn open_existing<'a, 'b: 'a>(
    name:                   impl string::InWide,
    access:                 impl Into<access::Mask>,
    share:                  impl Into<file::Share>,
) -> firehazard::Result<handle::Owned> {
    string::convert_to_cstrnn::<{limit::stack::PIPE_NAME}, _, _>(name, |name| {
        // See also:
        //  • https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew#consoles
        //  • https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew#pipes
        unsafe { handle::Owned::from_raw(winapi::um::fileapi::CreateFileW(
            name.as_cstr(),

            // XXX: duplex will supposedly fail if trying to open `"CON"` per https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea#consoles
            access.into().into(), // GENERIC_READ | GENERIC_WRITE | FILE_READ_ATTRIBUTES

            // XXX: FILE_SHARE_WRITE should be considered for `"CONOUT$"`, but not most other files/pipes?
            share.into().into(), // NONE | READ

            // "CreateFile ignores the lpSecurityDescriptor member when opening an existing file or device, but continues to use the bInheritHandle member."
            null_mut(), // OTOH, maybe just call set_handle_information if you want to do that instead?  Or use a builder?
            //(&mut security::Attributes::from(inherit_handle)).into(),

            // "For devices other than files, this parameter is usually set to OPEN_EXISTING."
            // OwnedWriter::open_existing 's docs note that if `name` might be a nonexistant file to create, this should not be your preference
            winapi::um::fileapi::OPEN_EXISTING,

            // SECURITY_SQOS_PRESENT + related      could also be useful in some overloads?
            // FILE_FLAG_OVERLAPPED                 is a soundness hazard (see https://github.com/rust-lang/rust/issues/81357 , https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-open-modes#overlapped-mode )
            // FILE_FLAG_WRITE_THROUGH              might be useful if you want synchronous behavior over a network (see https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-open-modes#write-through-mode )
            0,

            // "When opening an existing file, CreateFile ignores this parameter." (hTemplateFile)
            null_mut(),

        ))}.map_err(|_| firehazard::Error::get_last())
    })?
}
