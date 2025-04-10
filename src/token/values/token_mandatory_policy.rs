use core::fmt::{self, Debug, Formatter};
use core::ops::*;



/// \[[microsoft.com](https://learn.microsoft.com/et-ee/windows/win32/api/winnt/ns-winnt-token_mandatory_policy)\]
/// ≈ TOKEN_MANDATORY_POLICY
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] pub struct MandatoryPolicyMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/et-ee/windows/win32/api/winnt/ns-winnt-token_mandatory_policy)\]
/// ≈ TOKEN_MANDATORY_POLICY
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] pub struct MandatoryPolicy { policy: u32 }
// What's this? MandatoryPolicy straddles the line of policy and flags struct?
// Well, the win32 API defines TOKEN_MANDATORY_POLICY as a single field struct, so I mimic that.
// OTOH, TOKEN_MANDATORY_POLICY_* constants are meant to apply to policy as DWORDs, so I mimic that too.

impl MandatoryPolicy {
    /// ### Safety
    /// *   Some APIs might theoretically assume the policy is valid?
    pub const unsafe fn from_unchecked(policy: u32) -> Self { Self { policy } }

    pub fn policy(self) -> u32 { self.policy }
    pub fn set_policy(&mut self, value: Self) { *self = value; }

    /// TOKEN_MANDATORY_POLICY_OFF
    pub const OFF               : MandatoryPolicy = MandatoryPolicy { policy: winapi::um::winnt::TOKEN_MANDATORY_POLICY_OFF             };

    /// TOKEN_MANDATORY_POLICY_NO_WRITE_UP
    pub const NO_WRITE_UP       : MandatoryPolicy = MandatoryPolicy { policy: winapi::um::winnt::TOKEN_MANDATORY_POLICY_NO_WRITE_UP     };

    /// TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN
    pub const NEW_PROCESS_MIN   : MandatoryPolicy = MandatoryPolicy { policy: winapi::um::winnt::TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN };

    /// TOKEN_MANDATORY_POLICY_VALID_MASK
    pub const VALID_MASK        : MandatoryPolicyMask = MandatoryPolicyMask(winapi::um::winnt::TOKEN_MANDATORY_POLICY_VALID_MASK);
}

impl Debug for MandatoryPolicy {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;

        let mut v = self.policy;
        if v == 0 { return write!(fmt, "TOKEN_MANDATORY_POLICY_OFF") }

        macro_rules! v { ($e:expr) => {{
            const E : u32 = $e;
            if v & E != 0 {
                write!(fmt, "{}", stringify!($e))?;
                v &= !E;
                if v != 0 { write!(fmt, " | ")?; }
            }
        }}}

        v!(TOKEN_MANDATORY_POLICY_NO_WRITE_UP);
        v!(TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN);

        if v != 0 { write!(fmt, "0x{:04x}", v)? }

        Ok(())
    }
}

impl From<()> for MandatoryPolicy { fn from(_: ()) -> Self { Self { policy: 0 } } }
impl From<MandatoryPolicy> for u32 { fn from(mp: MandatoryPolicy) -> Self { mp.policy } }

impl BitAnd         for MandatoryPolicy { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self { policy: self.policy & rhs.policy } } }
impl BitXor         for MandatoryPolicy { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self { policy: self.policy ^ rhs.policy } } }
impl BitOr          for MandatoryPolicy { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self { policy: self.policy | rhs.policy } } }
impl BitAndAssign   for MandatoryPolicy { fn bitand_assign(&mut self, rhs: Self) { self.policy &= rhs.policy } }
impl BitXorAssign   for MandatoryPolicy { fn bitxor_assign(&mut self, rhs: Self) { self.policy ^= rhs.policy } }
impl BitOrAssign    for MandatoryPolicy { fn bitor_assign (&mut self, rhs: Self) { self.policy |= rhs.policy } }

impl Not                                for MandatoryPolicy     { type Output = MandatoryPolicyMask; fn not(self) -> Self::Output { MandatoryPolicyMask(!self.policy) } }
impl BitAnd<MandatoryPolicyMask>        for MandatoryPolicy     { type Output = MandatoryPolicy; fn bitand(self, rhs: MandatoryPolicyMask) -> Self::Output { MandatoryPolicy { policy: self.policy & rhs.0 } } }
impl BitAnd<MandatoryPolicy    >        for MandatoryPolicyMask { type Output = MandatoryPolicy; fn bitand(self, rhs: MandatoryPolicy    ) -> Self::Output { MandatoryPolicy { policy: self.0 & rhs.policy } } }
impl BitAndAssign<MandatoryPolicyMask>  for MandatoryPolicy     { fn bitand_assign(&mut self, rhs: MandatoryPolicyMask) { self.policy &= rhs.0 } }
