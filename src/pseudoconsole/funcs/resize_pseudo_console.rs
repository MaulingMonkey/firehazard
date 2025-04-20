#[doc(alias = "ResizePseudoConsole")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/resizepseudoconsole?source=recommendations)\]
/// ResizePseudoConsole
///
pub fn resize_pseudo_console(
    pcon:   &pseudoconsole::Owned,
    size:   impl pseudoconsole::IntoSize,
) -> firehazard::Result<()> {
    let hr = unsafe { winapi::um::consoleapi::ResizePseudoConsole(
        pcon.as_handle(),
        size.into(),
    )};

    if !SUCCEEDED(hr) { return Err(firehazard::Error(hr as _)) }
    Ok(())
}
