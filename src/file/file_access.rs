use crate::prelude::*;



#[doc(alias = "FILE_READ_DATA")]
/// FILE_READ_DATA
pub const READ_DATA             : access::Mask = unsafe { access::Mask::from_unchecked(0x0001) }; // file & pipe
#[doc(alias = "FILE_LIST_DIRECTORY")]
/// FILE_LIST_DIRECTORY
pub const LIST_DIRECTORY        : access::Mask = unsafe { access::Mask::from_unchecked(0x0001) }; // directory

#[doc(alias = "FILE_WRITE_DATA")]
/// FILE_WRITE_DATA
pub const WRITE_DATA            : access::Mask = unsafe { access::Mask::from_unchecked(0x0002) }; // file & pipe
#[doc(alias = "FILE_ADD_FILE")]
/// FILE_ADD_FILE
pub const ADD_FILE              : access::Mask = unsafe { access::Mask::from_unchecked(0x0002) }; // directory

#[doc(alias = "FILE_APPEND_DATA")]
/// FILE_APPEND_DATA
pub const APPEND_DATA           : access::Mask = unsafe { access::Mask::from_unchecked(0x0004) }; // file
#[doc(alias = "FILE_ADD_SUBDIRECTORY")]
/// FILE_ADD_SUBDIRECTORY
pub const ADD_SUBDIRECTORY      : access::Mask = unsafe { access::Mask::from_unchecked(0x0004) }; // directory
#[doc(alias = "FILE_CREATE_PIPE_INSTANCE")]
/// FILE_CREATE_PIPE_INSTANCE
pub const CREATE_PIPE_INSTANCE  : access::Mask = unsafe { access::Mask::from_unchecked(0x0004) }; // named pipe

#[doc(alias = "FILE_READ_EA")]
/// FILE_READ_EA
pub const READ_EA               : access::Mask = unsafe { access::Mask::from_unchecked(0x0008) }; // file & directory

#[doc(alias = "FILE_WRITE_EA")]
/// FILE_WRITE_EA
pub const WRITE_EA              : access::Mask = unsafe { access::Mask::from_unchecked(0x0010) }; // file & directory

#[doc(alias = "FILE_EXECUTE")]
/// FILE_EXECUTE
pub const EXECUTE               : access::Mask = unsafe { access::Mask::from_unchecked(0x0020) }; // file
#[doc(alias = "FILE_TRAVERSE")]
/// FILE_TRAVERSE
pub const TRAVERSE              : access::Mask = unsafe { access::Mask::from_unchecked(0x0020) }; // directory

#[doc(alias = "FILE_DELETE_CHILD")]
/// FILE_DELETE_CHILD
pub const DELETE_CHILD          : access::Mask = unsafe { access::Mask::from_unchecked(0x0040) }; // directory

#[doc(alias = "FILE_READ_ATTRIBUTES")]
/// FILE_READ_ATTRIBUTES
pub const READ_ATTRIBUTES       : access::Mask = unsafe { access::Mask::from_unchecked(0x0080) }; // all

#[doc(alias = "FILE_WRITE_ATTRIBUTES")]
/// FILE_WRITE_ATTRIBUTES
pub const WRITE_ATTRIBUTES      : access::Mask = unsafe { access::Mask::from_unchecked(0x0100) }; // all

#[doc(alias = "FILE_ALL_ACCESS")]
/// FILE_ALL_ACCESS = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0x1FF
pub const ALL_ACCESS            : access::Mask = unsafe { access::Mask::from_unchecked(winapi::um::winnt::FILE_ALL_ACCESS) };

#[doc(alias = "FILE_GENERIC_READ")]
/// FILE_GENERIC_READ = STANDARD_RIGHTS_READ | FILE_READ_DATA | FILE_READ_ATTRIBUTES | FILE_READ_EA | SYNCHRONIZE
pub const GENERIC_READ          : access::Mask = unsafe { access::Mask::from_unchecked(winapi::um::winnt::FILE_GENERIC_READ) };

#[doc(alias = "FILE_GENERIC_WRITE")]
/// FILE_GENERIC_WRITE = STANDARD_RIGHTS_WRITE | FILE_WRITE_DATA | FILE_WRITE_ATTRIBUTES | FILE_WRITE_EA | FILE_APPEND_DATA | SYNCHRONIZE
pub const GENERIC_WRITE         : access::Mask = unsafe { access::Mask::from_unchecked(winapi::um::winnt::FILE_GENERIC_WRITE) };

#[doc(alias = "FILE_GENERIC_EXECUTE")]
/// FILE_GENERIC_EXECUTE = STANDARD_RIGHTS_EXECUTE | FILE_READ_ATTRIBUTES | FILE_EXECUTE | SYNCHRONIZE
pub const GENERIC_EXECUTE       : access::Mask = unsafe { access::Mask::from_unchecked(winapi::um::winnt::FILE_GENERIC_EXECUTE) };
