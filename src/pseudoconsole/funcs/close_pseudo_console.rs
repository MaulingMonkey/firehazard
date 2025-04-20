#[doc(alias = "ClosePseudoConsole")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/closepseudoconsole)\]
/// ClosePseudoConsole
///
pub fn close_pseudo_console(pcon: impl Into<pseudoconsole::Owned>) {
    // NOTE: ClosePseudoConsole returns no errors
    unsafe { winapi::um::consoleapi::ClosePseudoConsole(
        core::mem::ManuallyDrop::new(pcon.into()).as_handle()
    )}
}
