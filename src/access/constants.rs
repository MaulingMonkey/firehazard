use crate::access::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// DELETE
///
pub const DELETE                    : Mask      = Mask    (winapi::um::winnt::DELETE                    ); // 0x00010000

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// READ_CONTROL
///
pub const READ_CONTROL              : Mask      = Mask    (winapi::um::winnt::READ_CONTROL              ); // 0x00020000

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// WRITE_DAC
///
pub const WRITE_DAC                 : Mask      = Mask    (winapi::um::winnt::WRITE_DAC                 ); // 0x00040000

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// WRITE_OWNER
///
pub const WRITE_OWNER               : Mask      = Mask    (winapi::um::winnt::WRITE_OWNER               ); // 0x00080000

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// SYNCHRONIZE
///
pub const SYNCHRONIZE               : Mask      = Mask    (winapi::um::winnt::SYNCHRONIZE               ); // 0x00100000



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// STANDARD_RIGHTS_REQUIRED
///
pub const STANDARD_RIGHTS_REQUIRED  : MaskMask  = MaskMask(winapi::um::winnt::STANDARD_RIGHTS_REQUIRED  ); // 0x000F0000

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// STANDARD_RIGHTS_READ
///
pub const STANDARD_RIGHTS_READ      : Mask      = Mask    (winapi::um::winnt::STANDARD_RIGHTS_READ      ); // READ_CONTROL - this seems... buggy? on the windows sdk side of things?

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// STANDARD_RIGHTS_WRITE
///
pub const STANDARD_RIGHTS_WRITE     : Mask      = Mask    (winapi::um::winnt::STANDARD_RIGHTS_WRITE     ); // READ_CONTROL - this seems... buggy? on the windows sdk side of things?

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// STANDARD_RIGHTS_EXECUTE
///
pub const STANDARD_RIGHTS_EXECUTE   : Mask      = Mask    (winapi::um::winnt::STANDARD_RIGHTS_EXECUTE   ); // READ_CONTROL - this seems... buggy? on the windows sdk side of things?

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)\]
/// STANDARD_RIGHTS_ALL
///
pub const STANDARD_RIGHTS_ALL       : MaskMask  = MaskMask(winapi::um::winnt::STANDARD_RIGHTS_ALL       ); // 0x001F0000



// TODO: better docs link?
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-mask)\]
/// SPECIFIC_RIGHTS_ALL
///
pub const SPECIFIC_RIGHTS_ALL       : MaskMask  = MaskMask(winapi::um::winnt::SPECIFIC_RIGHTS_ALL       ); // 0x0000FFFF



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/sacl-access-right)\]
/// ACCESS_SYSTEM_SECURITY
///
pub const ACCESS_SYSTEM_SECURITY    : Mask      = Mask    (winapi::um::winnt::ACCESS_SYSTEM_SECURITY    ); // 0x01000000

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/requesting-access-rights-to-an-object)\]
/// MAXIMUM_ALLOWED
///
pub const MAXIMUM_ALLOWED           : Mask      = Mask    (winapi::um::winnt::MAXIMUM_ALLOWED           ); // 0x02000000



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/generic-access-rights)\]
/// GENERIC_READ
///
pub const GENERIC_READ              : Mask      = Mask    (winapi::um::winnt::GENERIC_READ              ); // 0x80000000

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/generic-access-rights)\]
/// GENERIC_WRITE
///
pub const GENERIC_WRITE             : Mask      = Mask    (winapi::um::winnt::GENERIC_WRITE             ); // 0x40000000

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/generic-access-rights)\]
/// GENERIC_EXECUTE
///
pub const GENERIC_EXECUTE           : Mask      = Mask    (winapi::um::winnt::GENERIC_EXECUTE           ); // 0x20000000

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/generic-access-rights)\]
/// GENERIC_ALL
///
pub const GENERIC_ALL               : Mask      = Mask    (winapi::um::winnt::GENERIC_ALL               ); // 0x10000000
