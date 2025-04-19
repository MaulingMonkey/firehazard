#[doc(alias = "ResizePseudoConsole")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/resizepseudoconsole?source=recommendations)\]
/// ResizePseudoConsole
///
pub fn resize_pseudo_console(
    pcon:   &firehazard::pseudoconsole::Owned,
    size:   impl firehazard::pseudoconsole::IntoSize,
) -> Result<(), firehazard::Error> {
    use firehazard::AsLocalHandle;

    let hr = unsafe { winapi::um::consoleapi::ResizePseudoConsole(
        pcon.as_handle(),
        size.into(),
    )};

    if !winapi::shared::winerror::SUCCEEDED(hr) { return Err(firehazard::Error(hr as _)) }
    Ok(())
}
