// NOTE: Everything here is at most `pub(crate)`
//  • Not part of the Windows SDK, this doesn't appear to be part of Microsoft's intended public API.
//  • Signatures etc. on documentation, rather than an actual header.  Sketchy, as docs lie.



#[doc(alias = "NtQueryInformationFile")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntqueryinformationfile)\]
/// NtQueryInformationFile(handle, ...)
///
/// ### Errors
///
/// | error                         | condition |
/// | ------------------------------| ----------|
/// | STATUS_NOT_IMPLEMENTED        | `ntdll.dll` or `NtQueryObject` cannot be loaded
/// | STATUS_ACCESS_DENIED          | Insufficient permissions for query?
/// | STATUS_INVALID_HANDLE         | Invalid `file_handle`?
/// | ...                           | ...
///
pub(crate) fn nt_query_information_file<'h, Info: NtFileInformation>(
    file_handle:    impl Into<handle::Pseudo<'h>>,
) -> firehazard::Result<alloc::CBoxSized<Info>> {
    #[allow(non_snake_case)] let NtQueryInformationFile = *ntdll::NtQueryInformationFile;

    let mut stack = Info::zeroed();
    let stack_size = u32::try_from(core::mem::size_of_val(&stack)).unwrap();

    let mut io_status_block = bytemuck::Zeroable::zeroed();
    let status = unsafe { NtQueryInformationFile(
        file_handle.into().as_handle(),
        &mut io_status_block,
        core::ptr::from_mut(&mut stack).cast(),
        stack_size,
        Info::CLASS,
    )};
    if status == STATUS::SUCCESS { return Ok(alloc::CBoxSized::new(stack)) }

    // TODO: var-sized fallback?

    Err(status.into())
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntqueryinformationfile)\]
/// NtQueryInformationFile parameters 3 ..= 5
///
/// *   parameter 3 (FileInformation)       will be passed `&mut self`
/// *   parameter 4 (Length)                will be passed `size_of_val(&self)`, at least when first called.  Oversized allocs may also be attempted.
/// *   parameter 5 (FileInformationClass)  will be passed `Self::CLASS`
///
/// ### Safety
///
/// The above arguments must be safe to pass to `NtQueryInformationFile`.
/// Possible safety issues include `Self` being too small for `Self::CLASS`, or `NtQueryInformationFile` writing invalid bit patterns for `Self`.
///
pub(crate) unsafe trait NtFileInformation : bytemuck::Zeroable {
    /// Will be passed to `NtQueryInformationFile`'s 5th parameter, `FileInformationClass`.
    const CLASS : file::InformationClass;
}
