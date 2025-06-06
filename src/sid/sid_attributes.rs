#[allow(unused_imports)] use crate::prelude::*;
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
/// DWORD/[u32]: SE_GROUP_*: SID (group) attributes flags mask
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AttributesMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
/// DWORD/[u32]: SE_GROUP_*: SID (group) attributes flags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Attributes(u32);

flags!(impl .. for Attributes(u32) - AttributesMask);

impl Attributes {
    /// ### Safety
    /// *   Some APIs might theoretically assume flags are a valid?
    pub const unsafe fn from_unchecked(flags: u32) -> Self { Self(flags) }
}

impl Debug for Attributes {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;
        flags!(self.0, fmt, "0x{:X}", [
            SE_GROUP_ENABLED,
            SE_GROUP_ENABLED_BY_DEFAULT,
            SE_GROUP_INTEGRITY,
            SE_GROUP_INTEGRITY_ENABLED,
            SE_GROUP_LOGON_ID,
            SE_GROUP_MANDATORY,
            SE_GROUP_OWNER,
            SE_GROUP_RESOURCE,
            SE_GROUP_USE_FOR_DENY_ONLY,
        ])
    }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
/// SE_GROUP_*
///
pub mod group {
    use super::*;



    #[doc(alias = "SE_GROUP_ENABLED")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
    /// SE_GROUP_ENABLED
    /// <br>
    /// The SID is enabled for access checks. When the system performs an access check, it checks for access-allowed and access-denied access control entries (ACEs) that apply to the SID.
    /// A SID without this attribute is ignored during an access check unless the [`group::USE_FOR_DENY_ONLY`] attribute is set.
    /// <br>
    /// &nbsp;
    ///
    pub const ENABLED : Attributes = Attributes(winapi::um::winnt::SE_GROUP_ENABLED);



    #[doc(alias = "SE_GROUP_ENABLED_BY_DEFAULT")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
    /// SE_GROUP_ENABLED_BY_DEFAULT<br>
    /// The SID is enabled by default.<br>
    /// &nbsp;<br>
    ///
    pub const ENABLED_BY_DEFAULT : Attributes = Attributes(winapi::um::winnt::SE_GROUP_ENABLED_BY_DEFAULT);



    #[doc(alias = "SE_GROUP_INTEGRITY")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
    /// SE_GROUP_INTEGRITY<br>
    /// The SID is a mandatory integrity SID.<br>
    /// &nbsp;
    ///
    pub const INTEGRITY : Attributes = Attributes(winapi::um::winnt::SE_GROUP_INTEGRITY);



    #[doc(alias = "SE_GROUP_INTEGRITY_ENABLED")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
    /// SE_GROUP_INTEGRITY_ENABLED<br>
    /// The SID is enabled for mandatory integrity checks.<br>
    /// &nbsp;
    ///
    pub const INTEGRITY_ENABLED : Attributes = Attributes(winapi::um::winnt::SE_GROUP_INTEGRITY_ENABLED);



    #[doc(alias = "SE_GROUP_LOGON_ID")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
    /// SE_GROUP_LOGON_ID<br>
    /// The SID is a logon SID that identifies the logon session associated with an access token.<br>
    /// &nbsp;
    ///
    pub const LOGON_ID : Attributes = Attributes(winapi::um::winnt::SE_GROUP_LOGON_ID);



    #[doc(alias = "SE_GROUP_MANDATORY")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
    /// SE_GROUP_MANDATORY
    /// <br>
    /// The SID cannot have the [`group::ENABLED`] attribute cleared by a call to the [`AdjustTokenGroups`] function.
    /// However, you can use the <code>[`create_restricted_token`]\[[`_filter`](create_restricted_token_filter)\]</code> functions to convert a mandatory SID to a deny-only SID.
    /// <br>
    /// &nbsp;
    ///
    /// [`AdjustTokenGroups`]:  https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokengroups
    ///
    pub const MANDATORY : Attributes = Attributes(winapi::um::winnt::SE_GROUP_MANDATORY);



    #[doc(alias = "SE_GROUP_OWNER")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
    /// SE_GROUP_OWNER<br>
    /// The SID identifies a group account for which the user of the token is the owner of the group, or the SID can be assigned as the owner of the token or objects.<br>
    /// &nbsp;
    ///
    pub const OWNER : Attributes = Attributes(winapi::um::winnt::SE_GROUP_OWNER);



    #[doc(alias = "SE_GROUP_RESOURCE")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
    /// SE_GROUP_RESOURCE<br>
    /// The SID identifies a domain-local group.<br>
    /// &nbsp;
    ///
    pub const RESOURCE : Attributes = Attributes(winapi::um::winnt::SE_GROUP_RESOURCE);



    #[doc(alias = "SE_GROUP_USE_FOR_DENY_ONLY")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups#members)\]
    /// SE_GROUP_USE_FOR_DENY_ONLY
    /// <br>
    /// The SID is a deny-only SID in a restricted token. When the system performs an access check, it checks for access-denied ACEs that apply to the SID; it ignores access-allowed ACEs for the SID.
    /// If this attribute is set, [`group::ENABLED`] is not set, and the SID cannot be reenabled.
    /// <br>
    /// &nbsp;
    ///
    pub const USE_FOR_DENY_ONLY : Attributes = Attributes(winapi::um::winnt::SE_GROUP_USE_FOR_DENY_ONLY);
}
