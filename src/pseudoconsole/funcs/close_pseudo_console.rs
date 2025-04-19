#[doc(alias = "ClosePseudoConsole")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/closepseudoconsole)\]
/// ClosePseudoConsole
///
pub fn close_psuedo_console(pcon: impl Into<firehazard::pseudoconsole::Owned>) {
    // NOTE: ClosePsuedoConsole returns no errors
    use firehazard::AsLocalHandle;
    unsafe { winapi::um::consoleapi::ClosePseudoConsole(
        core::mem::ManuallyDrop::new(pcon.into()).as_handle()
    )}
}
