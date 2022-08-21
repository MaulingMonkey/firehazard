use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// ACE_HEADER::AceType
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Type(u8);

impl Type {
    pub const ACCESS_ALLOWED                    : Type = Type(ACCESS_ALLOWED_ACE_TYPE);
    pub const ACCESS_DENIED                     : Type = Type(ACCESS_DENIED_ACE_TYPE);
    pub const ACCESS_ALLOWED_OBJECT             : Type = Type(ACCESS_ALLOWED_OBJECT_ACE_TYPE);
    pub const ACCESS_DENIED_OBJECT              : Type = Type(ACCESS_DENIED_OBJECT_ACE_TYPE);
    pub const SYSTEM_AUDIT                      : Type = Type(SYSTEM_AUDIT_ACE_TYPE);
    pub const SYSTEM_ALARM                      : Type = Type(SYSTEM_ALARM_ACE_TYPE);
    pub const SYSTEM_AUDIT_OBJECT               : Type = Type(SYSTEM_AUDIT_OBJECT_ACE_TYPE);
    pub const SYSTEM_ALARM_OBJECT               : Type = Type(SYSTEM_ALARM_OBJECT_ACE_TYPE);
    pub const SYSTEM_MANDATORY_LABEL            : Type = Type(SYSTEM_MANDATORY_LABEL_ACE_TYPE);
    pub const ACCESS_ALLOWED_CALLBACK           : Type = Type(ACCESS_ALLOWED_CALLBACK_ACE_TYPE);
    pub const ACCESS_DENIED_CALLBACK            : Type = Type(ACCESS_DENIED_CALLBACK_ACE_TYPE);
    pub const SYSTEM_RESOURCE_ATTRIBUTE         : Type = Type(SYSTEM_RESOURCE_ATTRIBUTE_ACE_TYPE);
    pub const SYSTEM_SCOPED_POLICY_ID           : Type = Type(SYSTEM_SCOPED_POLICY_ID_ACE_TYPE);
    pub const SYSTEM_AUDIT_CALLBACK             : Type = Type(SYSTEM_AUDIT_CALLBACK_ACE_TYPE);
    pub const SYSTEM_AUDIT_CALLBACK_OBJECT      : Type = Type(SYSTEM_AUDIT_CALLBACK_OBJECT_ACE_TYPE);
    pub const ACCESS_ALLOWED_CALLBACK_OBJECT    : Type = Type(ACCESS_ALLOWED_CALLBACK_OBJECT_ACE_TYPE);
    pub const ACCESS_DENIED_CALLBACK_OBJECT     : Type = Type(ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE);
    pub const SYSTEM_PROCESS_TRUST_LABEL        : Type = Type(SYSTEM_PROCESS_TRUST_LABEL_ACE_TYPE);
    pub const SYSTEM_ACCESS_FILTER              : Type = Type(SYSTEM_ACCESS_FILTER_ACE_TYPE);
}

impl Type {
    pub fn short_type(&self) -> Option<&'static str> {
        Some(match *self {
            Self::ACCESS_ALLOWED                    => "A",
            Self::ACCESS_DENIED                     => "D",
            Self::ACCESS_ALLOWED_OBJECT             => "OA",
            Self::ACCESS_DENIED_OBJECT              => "OD",
            Self::SYSTEM_AUDIT                      => "AU",
            Self::SYSTEM_ALARM                      => "AL",
            Self::SYSTEM_AUDIT_OBJECT               => "OU",
            Self::SYSTEM_ALARM_OBJECT               => "OL",
            Self::SYSTEM_MANDATORY_LABEL            => "ML",
            Self::ACCESS_ALLOWED_CALLBACK           => "XA",
            Self::ACCESS_DENIED_CALLBACK            => "XD",
            Self::SYSTEM_RESOURCE_ATTRIBUTE         => "RA",
            Self::SYSTEM_SCOPED_POLICY_ID           => "SP",
            Self::SYSTEM_AUDIT_CALLBACK             => "XU",
            Self::SYSTEM_AUDIT_CALLBACK_OBJECT      => return None, // XXX
            Self::ACCESS_ALLOWED_CALLBACK_OBJECT    => "ZA",
            Self::ACCESS_DENIED_CALLBACK_OBJECT     => return None, // XXX
            Self::SYSTEM_PROCESS_TRUST_LABEL        => "TL",
            Self::SYSTEM_ACCESS_FILTER              => "FL",
            _                                       => return None
        })
    }

    pub fn long_type(&self) -> Option<&'static str> {
        Some(match *self {
            Self::ACCESS_ALLOWED                    => "ACCESS_ALLOWED",
            Self::ACCESS_DENIED                     => "ACCESS_DENIED",
            Self::ACCESS_ALLOWED_OBJECT             => "ACCESS_ALLOWED_OBJECT",
            Self::ACCESS_DENIED_OBJECT              => "ACCESS_DENIED_OBJECT",
            Self::SYSTEM_AUDIT                      => "SYSTEM_AUDIT",
            Self::SYSTEM_ALARM                      => "SYSTEM_ALARM",
            Self::SYSTEM_AUDIT_OBJECT               => "SYSTEM_AUDIT_OBJECT",
            Self::SYSTEM_ALARM_OBJECT               => "SYSTEM_ALARM_OBJECT",
            Self::SYSTEM_MANDATORY_LABEL            => "SYSTEM_MANDATORY_LABEL",
            Self::ACCESS_ALLOWED_CALLBACK           => "ACCESS_ALLOWED_CALLBACK",
            Self::ACCESS_DENIED_CALLBACK            => "ACCESS_DENIED_CALLBACK",
            Self::SYSTEM_RESOURCE_ATTRIBUTE         => "SYSTEM_RESOURCE_ATTRIBUTE",
            Self::SYSTEM_SCOPED_POLICY_ID           => "SYSTEM_SCOPED_POLICY_ID",
            Self::SYSTEM_AUDIT_CALLBACK             => "SYSTEM_AUDIT_CALLBACK",
            Self::SYSTEM_AUDIT_CALLBACK_OBJECT      => "SYSTEM_AUDIT_CALLBACK_OBJECT",
            Self::ACCESS_ALLOWED_CALLBACK_OBJECT    => "ACCESS_ALLOWED_CALLBACK_OBJECT",
            Self::ACCESS_DENIED_CALLBACK_OBJECT     => "ACCESS_DENIED_CALLBACK_OBJECT",
            Self::SYSTEM_PROCESS_TRUST_LABEL        => "SYSTEM_PROCESS_TRUST_LABEL",
            Self::SYSTEM_ACCESS_FILTER              => "SYSTEM_ACCESS_FILTER",
            _                                       => return None
        })
    }
}

impl Debug for Type {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}_ACE_TYPE", self.long_type().unwrap_or("???")) }
}
