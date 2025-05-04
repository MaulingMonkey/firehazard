//! [Inherit], [Clear], [TryIntoEnvironment]

use crate::prelude::*;
use winapi::shared::minwindef::LPVOID;



/// Inherit the parent process's environment
pub struct Inherit;

/// Clear the parent process's environment
///
/// N.B. this means even things like `PATH` are cleared, which may affect DLL loading
pub struct Clear;

/// [process::environment::Inherit] | [process::environment::Clear] | &\[[u8]\] | &\[[u16]\]
pub unsafe trait TryIntoEnvironment { fn as_env_ptr(&self, expect_unicode: bool) -> firehazard::Result<LPVOID>; }

unsafe impl TryIntoEnvironment for Inherit { fn as_env_ptr(&self, _expect_unicode: bool) -> firehazard::Result<LPVOID> { Ok(null_mut()) } }
unsafe impl TryIntoEnvironment for Clear   { fn as_env_ptr(&self, _expect_unicode: bool) -> firehazard::Result<LPVOID> { Ok(&0u32 as *const _ as *mut _) } }

unsafe impl TryIntoEnvironment for &'_ [u8] {
    fn as_env_ptr(&self, expect_unicode: bool) -> firehazard::Result<LPVOID> {
        if expect_unicode           { Err(ERROR_BAD_ENVIRONMENT)?; }
        if !self.ends_with(&[0, 0]) { Err(E_STRING_NOT_NULL_TERMINATED)?; }
        Ok(self.as_ptr() as _)
    }
}

// Encoding is arguably confused here... but I'm pretty sure I'm okay with that?
            unsafe impl TryIntoEnvironment for &'_ str                  { fn as_env_ptr(&self, expect_unicode: bool) -> firehazard::Result<LPVOID> { TryIntoEnvironment::as_env_ptr(&self.as_bytes(), expect_unicode) } }
#[cfg(std)] unsafe impl TryIntoEnvironment for &'_ std::string::String  { fn as_env_ptr(&self, expect_unicode: bool) -> firehazard::Result<LPVOID> { TryIntoEnvironment::as_env_ptr(&self.as_bytes(), expect_unicode) } }
#[cfg(std)] unsafe impl TryIntoEnvironment for std::string::String      { fn as_env_ptr(&self, expect_unicode: bool) -> firehazard::Result<LPVOID> { TryIntoEnvironment::as_env_ptr(&self.as_bytes(), expect_unicode) } }

unsafe impl TryIntoEnvironment for &'_ [u16] {
    fn as_env_ptr(&self, expect_unicode: bool) -> firehazard::Result<LPVOID> {
        if !expect_unicode          { Err(ERROR_BAD_ENVIRONMENT)?; }
        if !self.ends_with(&[0, 0]) { Err(E_STRING_NOT_NULL_TERMINATED)?; }
        Ok(self.as_ptr() as _)
    }
}
