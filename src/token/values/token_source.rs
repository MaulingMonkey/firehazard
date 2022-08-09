use crate::*;
use abistr::*;
use winapi::um::winnt::{TOKEN_SOURCE_LENGTH, TOKEN_SOURCE};
use std::mem::{align_of, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_source)\]
/// TOKEN_SOURCE
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Source {
    pub source_name:        CStrBuf<u8, TOKEN_SOURCE_LENGTH>,
    pub source_identifier:  Luid,
}

impl Source {
    const _ALIGN : () = assert!(align_of::<Source>() == align_of::<TOKEN_SOURCE>());
    const _SIZE  : () = assert!(size_of ::<Source>() == size_of ::<TOKEN_SOURCE>());
}

impl AsRef<TOKEN_SOURCE> for Source { fn as_ref(&    self) -> &    TOKEN_SOURCE { unsafe { std::mem::transmute(self) } } }
impl AsMut<TOKEN_SOURCE> for Source { fn as_mut(&mut self) -> &mut TOKEN_SOURCE { unsafe { std::mem::transmute(self) } } }
impl AsRef<Source> for TOKEN_SOURCE { fn as_ref(&    self) -> &    Source  { unsafe { std::mem::transmute(self) } } }
impl AsMut<Source> for TOKEN_SOURCE { fn as_mut(&mut self) -> &mut Source  { unsafe { std::mem::transmute(self) } } }
impl From<Source> for TOKEN_SOURCE { fn from(e: Source ) -> Self { *e.as_ref() } }
impl From<TOKEN_SOURCE> for Source { fn from(e: TOKEN_SOURCE) -> Self { *e.as_ref() } }
