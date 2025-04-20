use crate::prelude::*;

use winapi::um::winnt::SID_MAX_SUB_AUTHORITIES;



#[doc(alias = "SID")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\]
/// Create a [`sid::Ptr`] at compile time via e.g. `sid!(S-1-0-0)`
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// let null_sid : sid::Ptr<'static>    = sid!(S-1-0-0);
/// let untrusted_integrity_level       = sid!(S-1-16-0);
/// let nt_administrators               = sid!(S-1-5-32-544); // stable?
///
/// // These SIDs almost certainly aren't stable/portable
/// let none = sid!(S-1-5-21-2440711095-4246273057-2830868914-513);
/// let docker_users = sid!(S-1-5-21-2440711095-4246273057-2830868914-1002);
/// ```
///
/// ### Compile Error
/// ```compile_fail
/// # use firehazard::*;
/// let too_many_subauthorities = sid!(S-1-1-1-2-3-4-5-6-7-8-9-10-11-12-13-14-15-16);
/// ```
///
#[macro_export] macro_rules! sid {
    (S-$rev:literal-$identifier_authority:literal$(-$sub_authority:literal)*) => {{
        const SUB_AUTHORITIES : [u32; {[0u32 $(,$sub_authority)*].len()-1}] = [$($sub_authority),*];
        assert!(SUB_AUTHORITIES.len() <= 15, "too many subauthorities (> SID_MAX_SUB_AUTHORITIES = 15)");
        const SID : $crate::sid::Static<{SUB_AUTHORITIES.len()}> = $crate::sid::Static::new($rev, $identifier_authority, SUB_AUTHORITIES);
        SID.as_sid_ptr()
    }};
}

#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)] pub struct Static<const NS: usize> {
    revision:               u8,
    sub_authority_count:    u8,
    identifier_authority:   [u8; 6],
    sub_authority:          [u32; NS],
}

impl<const NS: usize> Static<NS> {
    pub const fn new(revision: u8, identifier_authority: u8, sub_authority: [u32; NS]) -> Self {
        assert!(NS <= SID_MAX_SUB_AUTHORITIES as _, "too many subauthorities (> SID_MAX_SUB_AUTHORITIES = 15)");
        let identifier_authority = [0, 0, 0, 0, 0, identifier_authority];
        Self { revision, sub_authority_count: NS as u8, identifier_authority, sub_authority }
    }

    pub const fn as_sid_ptr<'s>(&'s self) -> sid::Ptr<'s> { unsafe { sid::Ptr::from_raw_unchecked(self as *const _ as *mut _) } }
}
