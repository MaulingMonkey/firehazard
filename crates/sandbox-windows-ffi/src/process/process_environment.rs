use crate::*;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::winerror::*;
use core::ptr::null_mut;



/// Inherit the parent process's environment
pub struct Inherit;

/// Clear the parent process's environment
///
/// N.B. this means even things like `PATH` are cleared, which may affect DLL loading
pub struct Clear;

/// [process::environment::Inherit] | [process::environment::Clear] | &\[[u8]\] | &\[[u16]\]
pub unsafe trait TryIntoEnvironment { fn as_env_ptr(&self, expect_unicode: bool) -> Result<LPVOID, Error>; }

unsafe impl TryIntoEnvironment for Inherit { fn as_env_ptr(&self, _expect_unicode: bool) -> Result<LPVOID, Error> { Ok(null_mut()) } }
unsafe impl TryIntoEnvironment for Clear   { fn as_env_ptr(&self, _expect_unicode: bool) -> Result<LPVOID, Error> { Ok(&0u32 as *const _ as *mut _) } }

unsafe impl<'a> TryIntoEnvironment for &'a [u8] {
    fn as_env_ptr(&self, expect_unicode: bool) -> Result<LPVOID, Error> {
        if expect_unicode           { return Err(Error(ERROR_BAD_ENVIRONMENT)) }
        if !self.ends_with(&[0, 0]) { return Err(Error(E_STRING_NOT_NULL_TERMINATED as _)) }
        Ok(self.as_ptr() as _)
    }
}

unsafe impl<'a> TryIntoEnvironment for &'a [u16] {
    fn as_env_ptr(&self, expect_unicode: bool) -> Result<LPVOID, Error> {
        if !expect_unicode          { return Err(Error(ERROR_BAD_ENVIRONMENT)) }
        if !self.ends_with(&[0, 0]) { return Err(Error(E_STRING_NOT_NULL_TERMINATED as _)) }
        Ok(self.as_ptr() as _)
    }
}
