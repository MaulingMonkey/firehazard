#[doc(alias = "CreatePseudoConsole")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/createpseudoconsole)\]
/// CreatePseudoConsole
///
/// ### Arguments
/// -   `size` &mdash; the initial width and height of the pseudo-console
/// -   `input` &mdash; a pipe the pseudo-console will read input from
/// -   `output` &mdash; a pipe the pseudo-console will write output to
/// -   `flags` &mdash; `0` | `pseudoconsole::INHERIT_CURSOR`
///
/// ### Safety
/// -   `size` being 0, negative, or overflowing has not been tested
/// -   `input` must not have been created with `FILE_FLAG_OVERLAPPED`
/// -   `output` must not have been created with `FILE_FLAG_OVERLAPPED`
/// -   `flags` being invalid has not been tested
///
pub unsafe fn create_pseudo_console<'i, 'o>(
    size:   impl pseudoconsole::IntoSize,
    input:  impl Into<io::ReadHandle<'i>>,
    output: impl Into<io::WriteHandle<'o>>,
    flags:  u32, // TODO: replace with a better type?
) -> firehazard::Result<pseudoconsole::Owned> {
    let mut pcon = null_mut();
    let hr = unsafe { winapi::um::consoleapi::CreatePseudoConsole(
        size.into(),
        input.into().as_handle().cast(),
        output.into().as_handle().cast(),
        flags,
        &mut pcon,
    )};

    if !SUCCEEDED(hr) { return Err(firehazard::Error(hr as _)) }
    let pcon = NonNull::new(pcon).ok_or(ERROR_INVALID_HANDLE)?;
    Ok(pseudoconsole::Owned(pcon))
}
