use crate::*;
use abistr::*;
use winapi::um::winnt::{TOKEN_SOURCE_LENGTH, TOKEN_SOURCE};
use std::mem::{align_of, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_source)\]
/// TOKEN_SOURCE
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TokenSource {
    pub source_name:        CStrBuf<u8, TOKEN_SOURCE_LENGTH>,
    pub source_identifier:  Luid,
}

impl TokenSource {
    const _ALIGN : () = assert!(align_of::<TokenSource>() == align_of::<TOKEN_SOURCE>());
    const _SIZE  : () = assert!(size_of ::<TokenSource>() == size_of ::<TOKEN_SOURCE>());
}

impl AsRef<TOKEN_SOURCE> for TokenSource { fn as_ref(&    self) -> &    TOKEN_SOURCE { unsafe { std::mem::transmute(self) } } }
impl AsMut<TOKEN_SOURCE> for TokenSource { fn as_mut(&mut self) -> &mut TOKEN_SOURCE { unsafe { std::mem::transmute(self) } } }
impl AsRef<TokenSource> for TOKEN_SOURCE { fn as_ref(&    self) -> &    TokenSource  { unsafe { std::mem::transmute(self) } } }
impl AsMut<TokenSource> for TOKEN_SOURCE { fn as_mut(&mut self) -> &mut TokenSource  { unsafe { std::mem::transmute(self) } } }
impl From<TokenSource> for TOKEN_SOURCE { fn from(ts: TokenSource ) -> Self { *ts.as_ref() } }
impl From<TOKEN_SOURCE> for TokenSource { fn from(ts: TOKEN_SOURCE) -> Self { *ts.as_ref() } }
