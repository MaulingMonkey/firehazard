#[cfg(std)] pub fn osstr_to_wide0<'b>(os: &std::ffi::OsStr, buf: &'b mut std::vec::Vec<u16>) -> Result<abistr::CStrNonNull<'b, u16>, crate::Error> {
    use std::os::windows::ffi::OsStrExt;
    buf.clear();
    buf.extend(os.encode_wide().chain([0]));
    if buf[..buf.len()-1].contains(&0) { return Err(crate::Error(winapi::shared::winerror::ERROR_ILLEGAL_CHARACTER)) }
    Ok(unsafe { abistr::CStrNonNull::<u16>::from_units_with_nul_unchecked(&buf[..]) })
}
