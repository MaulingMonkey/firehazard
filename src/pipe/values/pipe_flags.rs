#[allow(unused_imports)] use crate::prelude::*;
use winapi::shared::minwindef::DWORD;
use winapi::um::winbase::*;



// https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters

#[doc(alias = "PIPE_ACCESS_DUPLEX")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_ACCESS_DUPLEX
///
pub const ACCESS_DUPLEX : DWORD = PIPE_ACCESS_DUPLEX;

#[doc(alias = "PIPE_ACCESS_INBOUND")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_ACCESS_INBOUND
///
pub const ACCESS_INBOUND : DWORD = PIPE_ACCESS_INBOUND;

#[doc(alias = "PIPE_ACCESS_OUTBOUND")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_ACCESS_OUTBOUND
///
pub const ACCESS_OUTBOUND : DWORD = PIPE_ACCESS_OUTBOUND;



// TODO: WRITE_DAC
// TODO: WRITE_OWNER
// TODO: ACCESS_SYSTEM_SECURITY



#[doc(alias = "PIPE_TYPE_BYTE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_TYPE_BYTE
///
pub const TYPE_BYTE : DWORD = PIPE_TYPE_BYTE;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_TYPE_MESSAGE
///
#[doc(alias = "PIPE_TYPE_MESSAGE")]
pub const TYPE_MESSAGE : DWORD = PIPE_TYPE_MESSAGE;



#[doc(alias = "PIPE_WAIT")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_WAIT
///
pub const WAIT : DWORD = PIPE_WAIT;

#[deprecated = "Microsoft LAN Manager 2.0 compatability cruft that should not be used"]
#[doc(alias = "PIPE_NOWAIT")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_NOWAIT
///
/// Microsoft LAN Manager 2.0 compatability cruft that should not be used.
/// ReadFile, WriteFile, and [`pipe::named::connect`] will always return immediately.
///
pub const NOWAIT : DWORD = PIPE_NOWAIT;



#[doc(alias = "PIPE_ACCEPT_REMOTE_CLIENTS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_ACCEPT_REMOTE_CLIENTS
///
pub const ACCEPT_REMOTE_CLIENTS : DWORD = PIPE_ACCEPT_REMOTE_CLIENTS;

#[doc(alias = "PIPE_REJECT_REMOTE_CLIENTS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_REJECT_REMOTE_CLIENTS
///
pub const REJECT_REMOTE_CLIENTS : DWORD = PIPE_REJECT_REMOTE_CLIENTS;



#[doc(alias = "PIPE_READMODE_BYTE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_READMODE_BYTE
///
pub const READMODE_BYTE : DWORD = PIPE_READMODE_BYTE;

#[doc(alias = "PIPE_READMODE_MESSAGE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_READMODE_MESSAGE
///
pub const READMODE_MESSAGE : DWORD = PIPE_READMODE_MESSAGE;



#[doc(alias = "PIPE_CLIENT_END")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-getnamedpipeinfo)\]
/// PIPE_CLIENT_END
///
/// Pipe was created with `CreateFile*`, not `CreateNamedPipe*`.
///
/// This (currently) includes the "write" pipe created through [`pipe::create`] (and other `CreatePipe` wrappers), but not the "read" pipe.
///
pub const CLIENT_END : DWORD = PIPE_CLIENT_END;

#[doc(alias = "PIPE_SERVER_END")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-getnamedpipeinfo)\]
/// PIPE_SERVER_END
///
/// Pipe was created with `CreateNamedPipe*`, not `CreateFile*`.
///
/// This (currently) includes the "read" pipe created through [`pipe::create`] (and other `CreatePipe` wrappers), but not the "write" pipe.
///
pub const SERVER_END : DWORD = PIPE_SERVER_END;
