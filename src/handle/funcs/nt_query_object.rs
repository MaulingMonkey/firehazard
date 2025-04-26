#[cfg(std)] // currently required for minidl
#[doc(alias = "NtQueryObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntqueryobject)\]
/// NtQueryObject(handle, ...)
///
/// ### Errors
///
/// | error                         | condition |
/// | ------------------------------| ----------|
/// | ERROR_API_UNAVAILABLE         | `ntdll.dll` or `NtQueryObject` cannot be loaded
/// | STATUS_ACCESS_DENIED          | Insufficient permissions for query?
/// | STATUS_INVALID_HANDLE         | Invalid handle
/// | STATUS_INFO_LENGTH_MISMATCH   | For fixed sized `Info` types, unless the requested size match *exactly*?
/// | STATUS_BUFFER_OVERFLOW        | Insufficiently large buffer for info? (documented)
/// | STATUS_BUFFER_TOO_SMALL       | Insufficiently large buffer for info? (documented)
///
pub(crate) fn nt_query_object<'h, Info: ntdll::OBJECT_INFORMATION>(
    handle:     impl Into<handle::Pseudo<'h>>,
 ) -> firehazard::Result<alloc::CBoxSized<Info>> {
    use winapi::shared::ntstatus::*;

    let handle = handle.into();
    let handle = handle.as_handle().cast();
    #[allow(non_snake_case)] let NtQueryObject = (*ntdll::NtQueryObject)?;

    // Errata for ObjectBasicInformation / PUBLIC_OBJECT_BASIC_INFORMATION, as experienced on Windows 10.0.19045.5737:
    //  *   STATUS_INFO_LENGTH_MISMATCH is returned unless size == size_of::<Info>() *exactly*?
    //  *   Contrary to docs, `ReturnLength` isn't set on the above error - it remains 0.
    //
    // I suspect each case is manually handled, and fixed-size information classes will often only check the size, rather than write it.
    // As such, for every class, our first attempt is to request *exactly* size_of::<Info>().
    // If we succeed, or fail - and aren't told a larger buffer size to attempt - we can bail early after that.
    //
    let mut stack = MaybeUninit::<Info>::uninit();
    let stack_size = u32::try_from(size_of_val(&stack)).unwrap();
    let mut size = 0;
    let status = unsafe { NtQueryObject(handle, Info::CLASS, NonNull::new(stack.as_mut_ptr().cast()), stack_size, Some(&mut size)) };
    if status == STATUS_SUCCESS { return Ok(alloc::CBoxSized::new(unsafe { stack.assume_init() })) }
    if size <= stack_size       { return Err(firehazard::Error(status as _)) }

    let info = alloc::CBoxSized::new_oversized(Info::default(), usize::from32(size));
    let status = unsafe { NtQueryObject(handle, Info::CLASS, Some(info.as_non_null().cast()), size, None) };

    match status {
        STATUS_SUCCESS  => Ok(info),
        _               => Err(firehazard::Error::from(status)),
    }
}

#[cfg(std)] #[test] fn nt_query_object_file_object_basic_information_maybe_overlapped() {
    let overlapped  = create_file_w(cstr16!("Readme.md"), access::GENERIC_READ, file::Share::READ, None, winapi::um::fileapi::OPEN_EXISTING, file::FLAG_OVERLAPPED, None).unwrap();
    let overlapped  = nt_query_object::<ntdll::PUBLIC_OBJECT_BASIC_INFORMATION>(&overlapped ).unwrap();

    let synchronous = create_file_w(cstr16!("Readme.md"), access::GENERIC_READ, file::Share::READ, None, winapi::um::fileapi::OPEN_EXISTING, 0,                     None).unwrap();
    let synchronous = nt_query_object::<ntdll::PUBLIC_OBJECT_BASIC_INFORMATION>(&synchronous).unwrap();

    // XXX: Sadly, these appear identical: PUBLIC_OBJECT_BASIC_INFORMATION {
    //     Attributes: 0,
    //     GrantedAccess: READ_CONTROL | SYNCHRONIZE | 0x0089,
    //     HandleCount: 1,
    //     PointerCount: 32769,
    //     ..
    // }
    std::dbg!((&*overlapped, &*synchronous));
    // panic!();
}



#[cfg(std)] // currently required for minidl
#[doc(alias = "NtQueryObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntqueryobject)\]
/// NtQueryObject(handle, ObjectTypeInformation, ...)
///
/// | Known Values      | Includes  |
/// | ------------------| ----------|
/// | `Desktop`         | [Desktops](desktop)
/// | `File`            | [Files](crate::file), [Pipes](pipe)
/// | `Job`             | [Jobs](job)
/// | `Process`         | [Process](process) (including [`get_current_process`]), some dangling / never-valid handles
/// | `Thread`          | [Threads](thread) (including [`get_current_thread`])
/// | `Token`           | [Access Tokens](token)
/// | `WindowStation`   | [Window Stations](winsta)
/// | ???               | (Pseudo) Console (Buffer)s, Events, Event Logs, GDI Objects, Heaps, Mail slots, Modules, Mutexes, Semaphores, Sockets, Timers, Volumes, ...
///
///
///
/// ### Errors
///
/// | error                                                             | condition |
/// | ------------------------------------------------------------------| ----------|
/// | ERROR_API_UNAVAILABLE                                             | `ntdll.dll` or `NtQueryObject` cannot be loaded
/// | STATUS_ACCESS_DENIED                                              | Insufficient permissions to query `handle` (e.g. no `*_QUERY_[LIMITED_]INFORMATION` access?)
/// | <span style="color: red">`Ok("Process")`</span>                   | `handle` is dangling or never valid on Windows 10.0.19045.5737?
/// | STATUS_INVALID_HANDLE                                             | `handle` is dangling or never valid
/// | <span style="opacity: 50%">STATUS_INFO_LENGTH_MISMATCH?</span>    | If the [`nt_query_object_type_name`]-internal buffer is too small?
/// | <span style="opacity: 50%">STATUS_BUFFER_OVERFLOW?</span>         | If the [`nt_query_object_type_name`]-internal buffer is too small?
/// | <span style="opacity: 50%">STATUS_BUFFER_TOO_SMALL?</span>        | If the [`nt_query_object_type_name`]-internal buffer is too small?
///
///
///
/// ### Examples
///
/// ```
/// # #[cfg(std)] {
/// # use firehazard::*;
/// # use abistr::cstr16;
/// #
/// if let Ok(desktop) = open_input_desktop(None, false, access::GENERIC_ALL) {
///     assert_eq!("Desktop", nt_query_object_type_name(&desktop).unwrap());
/// }
///
///
///
/// if let Ok(file) = std::fs::File::open("Readme.md") {
///     assert_eq!("File", nt_query_object_type_name(&file).unwrap());
/// }
///
/// if let Ok((r, w)) = pipe::create(None, 0) {
///     assert_eq!("File", nt_query_object_type_name(&r).unwrap());
///     assert_eq!("File", nt_query_object_type_name(&w).unwrap());
/// }
///
///
///
/// if let Ok(job) = create_job_object_w(None, ()) {
///     assert_eq!("Job", nt_query_object_type_name(&job).unwrap());
/// }
///
///
///
/// assert_eq!("Process", nt_query_object_type_name(get_current_process()).unwrap());
/// if let Ok(process) = std::process::Command::new("cmd.exe").args(["/C", "ver"]).spawn() {
///     let process = process::OwnedHandle::from(process);
///     assert_eq!("Process", nt_query_object_type_name(&process).unwrap());
/// }
///
///
///
/// assert_eq!("Thread", nt_query_object_type_name(get_current_thread()).unwrap());
/// let thread = thread::OwnedHandle::from(std::thread::spawn(||{}));
/// assert_eq!("Thread", nt_query_object_type_name(&thread).unwrap());
///
///
///
/// // XXX: token::PseudoHandle → handle::Pseudo is banned
/// // assert_eq!("Token", nt_query_object_type_name(get_current_process_token()).unwrap());
///
/// if let Ok(token) = open_process_token(get_current_process(), token::ALL_ACCESS) {
///     assert_eq!("Token", nt_query_object_type_name(&token).unwrap());
///     if let Ok(token) = duplicate_token_ex(
///         &token, token::ALL_ACCESS, None, security::Delegation, token::Impersonation,
///     ) {
///         assert_eq!("Token", nt_query_object_type_name(&token).unwrap());
///     }
/// }
///
///
///
/// if let Ok(winsta0) = open_window_station_w(cstr16!("WinSta0"), false, winsta::ALL_ACCESS) {
///     assert_eq!("WindowStation", nt_query_object_type_name(&winsta0).unwrap());
/// }
/// #
/// # } // #[cfg(std)]
/// ```
///
pub fn nt_query_object_type_name<'h>(
    handle:     impl Into<handle::Pseudo<'h>>,
) -> firehazard::Result<std::ffi::OsString> {
    let info = nt_query_object::<ntdll::PUBLIC_OBJECT_TYPE_INFORMATION>(handle)?;
    Ok(std::os::windows::ffi::OsStringExt::from_wide(info.type_name()))
}

tests! {
    #[test] fn nt_query_object_type_name_invalid_handle_value() {
        // XXX: this is actually interpreted as get_process_handle()
        let r = nt_query_object_type_name(handle::invalid::invalid_value());
        assert_eq!(Ok(std::ffi::OsStr::new("Process")), r.as_deref());
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0] // no exception
    fn nt_query_object_type_name_never_valid() {
        let r = nt_query_object_type_name(handle::invalid::never_valid());
        if false { // XXX: this seems unreliable:
            assert_eq!(Ok(std::ffi::OsStr::new("Process")), r.as_deref());
        }
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0] // no exception
    fn nt_query_object_type_name_dangling() {
        let r = nt_query_object_type_name(handle::invalid::dangling());
        if false { // XXX: this seems unreliable:
            assert_eq!(Ok(std::ffi::OsStr::new("Process")), r.as_deref());
        }
    }
}
