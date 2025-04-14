use crate::*;
use abistr::*;
use winapi::um::winnt::{TOKEN_SOURCE_LENGTH, TOKEN_SOURCE};



#[doc(alias = "TOKEN_SOURCE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_source)\]
/// TOKEN_SOURCE
///
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)] pub struct Source {
    pub source_name:        CStrBuf<u8, TOKEN_SOURCE_LENGTH>,
    pub source_identifier:  Luid,
}



structure!(@assert layout token::Source => TOKEN_SOURCE {
    source_name         == SourceName,
    source_identifier   == SourceIdentifier,
});

impl AsRef<TOKEN_SOURCE> for Source { fn as_ref(&    self) -> &    TOKEN_SOURCE { unsafe { core::mem::transmute(self) } } }
impl AsMut<TOKEN_SOURCE> for Source { fn as_mut(&mut self) -> &mut TOKEN_SOURCE { unsafe { core::mem::transmute(self) } } }
impl AsRef<Source> for TOKEN_SOURCE { fn as_ref(&    self) -> &    Source  { unsafe { core::mem::transmute(self) } } }
impl AsMut<Source> for TOKEN_SOURCE { fn as_mut(&mut self) -> &mut Source  { unsafe { core::mem::transmute(self) } } }
impl From<Source> for TOKEN_SOURCE { fn from(e: Source ) -> Self { *e.as_ref() } }
impl From<TOKEN_SOURCE> for Source { fn from(e: TOKEN_SOURCE) -> Self { *e.as_ref() } }
