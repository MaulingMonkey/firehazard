//! [child_process], [desktop_app_breakaway], [mitigation_policy]\[[2](mitigation_policy2)\]

pub mod child_process;
pub mod desktop_app_breakaway;
pub mod mitigation_policy;
pub mod mitigation_policy2;



#[doc(alias = "PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY Flags Mask
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ChildProcessPolicyFlagsMask(u32);

#[doc(alias = "PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY Flags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ChildProcessPolicyFlags(u32);

flags!(impl .. for ChildProcessPolicyFlags(u32) - ChildProcessPolicyFlagsMask);



#[doc(alias = "PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY Flags Mask
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DesktopAppPolicyFlagsMask(u32);

#[doc(alias = "PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY Flags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DesktopAppPolicyFlags(u32);

flags!(impl .. for DesktopAppPolicyFlags(u32) - DesktopAppPolicyFlagsMask);



#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags
///
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] pub struct MitigationPolicy(MitigationPolicyFlags1, MitigationPolicyFlags2);

impl MitigationPolicy {
    pub const fn new1(flags1: MitigationPolicyFlags1) -> Self { Self(flags1, MitigationPolicyFlags2(0)) }
    pub const fn new2(flags1: MitigationPolicyFlags1, flags2: MitigationPolicyFlags2) -> Self { Self(flags1, flags2) }
    pub const fn flags1(&self) -> MitigationPolicyFlags1 { self.0 }
    pub const fn flags2(&self) -> MitigationPolicyFlags2 { self.1 }
    #[doc(hidden)] #[deprecated = "renamed to flags1"] pub const fn flag1(&self) -> MitigationPolicyFlags1 { self.0 }
    #[doc(hidden)] #[deprecated = "renamed to flags2"] pub const fn flag2(&self) -> MitigationPolicyFlags2 { self.1 }
}

impl From<MitigationPolicyFlags1> for MitigationPolicy { fn from(flags: MitigationPolicyFlags1) -> Self { Self(flags, MitigationPolicyFlags2(0)) } }
impl From<MitigationPolicyFlags2> for MitigationPolicy { fn from(flags: MitigationPolicyFlags2) -> Self { Self(MitigationPolicyFlags1(0), flags) } }
impl From<(MitigationPolicyFlags1, MitigationPolicyFlags2)> for MitigationPolicy { fn from(flags: (MitigationPolicyFlags1, MitigationPolicyFlags2)) -> Self { Self(flags.0, flags.1) } }

#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flag Mask
///
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] pub struct MitigationPolicyMask(MitigationPolicyMask1, MitigationPolicyMask2);

impl MitigationPolicyMask {
    pub const fn new1(mask1: MitigationPolicyMask1) -> Self { Self(mask1, MitigationPolicyMask2(0)) }
    pub const fn new2(mask1: MitigationPolicyMask1, mask2: MitigationPolicyMask2) -> Self { Self(mask1, mask2) }
    pub const fn mask1(&self) -> MitigationPolicyMask1 { self.0 }
    pub const fn mask2(&self) -> MitigationPolicyMask2 { self.1 }
}

impl From<MitigationPolicyMask1> for MitigationPolicyMask { fn from(flags: MitigationPolicyMask1) -> Self { Self(flags, MitigationPolicyMask2(0)) } }
impl From<MitigationPolicyMask2> for MitigationPolicyMask { fn from(flags: MitigationPolicyMask2) -> Self { Self(MitigationPolicyMask1(0), flags) } }
impl From<(MitigationPolicyMask1, MitigationPolicyMask2)> for MitigationPolicyMask { fn from(flags: (MitigationPolicyMask1, MitigationPolicyMask2)) -> Self { Self(flags.0, flags.1) } }

impl From<()                               > for MitigationPolicy     { fn from(_: ()                               ) -> Self { Self::default() } }
impl From<()                               > for MitigationPolicyMask { fn from(_: ()                               ) -> Self { Self::default() } }
impl From<Option<core::convert::Infallible>> for MitigationPolicy     { fn from(_: Option<core::convert::Infallible>) -> Self { Self::default() } }
impl From<Option<core::convert::Infallible>> for MitigationPolicyMask { fn from(_: Option<core::convert::Infallible>) -> Self { Self::default() } }
impl From<MitigationPolicy                 > for MitigationPolicyMask { fn from(flags: MitigationPolicy             ) -> Self { Self(flags.0.into(), flags.1.into()) } }

impl core::ops::Not                                 for MitigationPolicy        { type Output = MitigationPolicyMask;  fn not(self) -> Self::Output { MitigationPolicyMask(!self.0, !self.1) } }
impl core::ops::Not                                 for MitigationPolicyMask    { type Output = MitigationPolicyMask;  fn not(self) -> Self::Output { MitigationPolicyMask(!self.0, !self.1) } }

impl core::ops::BitAnd<MitigationPolicy             > for MitigationPolicy        { type Output = MitigationPolicy; fn bitand(self, rhs: MitigationPolicy               ) -> MitigationPolicy       { MitigationPolicy(self.0 & rhs.0, self.1 & rhs.1) } }
impl core::ops::BitAnd<MitigationPolicyMask         > for MitigationPolicy        { type Output = MitigationPolicy; fn bitand(self, rhs: MitigationPolicyMask           ) -> MitigationPolicy       { MitigationPolicy(self.0 & rhs.0, self.1 & rhs.1) } }
impl core::ops::BitAnd<MitigationPolicy             > for MitigationPolicyMask    { type Output = MitigationPolicy; fn bitand(self, rhs: MitigationPolicy               ) -> MitigationPolicy       { MitigationPolicy(self.0 & rhs.0, self.1 & rhs.1) } }
impl core::ops::BitAnd<MitigationPolicyFlags1       > for MitigationPolicy        { type Output = MitigationPolicyFlags1; fn bitand(self, rhs: MitigationPolicyFlags1   ) -> MitigationPolicyFlags1 { self.flags1() & rhs } }
impl core::ops::BitAnd<MitigationPolicyFlags2       > for MitigationPolicy        { type Output = MitigationPolicyFlags2; fn bitand(self, rhs: MitigationPolicyFlags2   ) -> MitigationPolicyFlags2 { self.flags2() & rhs } }
impl core::ops::BitAnd<MitigationPolicy             > for MitigationPolicyFlags1  { type Output = MitigationPolicyFlags1; fn bitand(self, rhs: MitigationPolicy         ) -> MitigationPolicyFlags1 { self & rhs.flags1() } }
impl core::ops::BitAnd<MitigationPolicy             > for MitigationPolicyFlags2  { type Output = MitigationPolicyFlags2; fn bitand(self, rhs: MitigationPolicy         ) -> MitigationPolicyFlags2 { self & rhs.flags2() } }
impl core::ops::BitAnd<MitigationPolicyMask1        > for MitigationPolicy        { type Output = MitigationPolicyFlags1; fn bitand(self, rhs: MitigationPolicyMask1    ) -> MitigationPolicyFlags1 { self.flags1() & rhs } }
impl core::ops::BitAnd<MitigationPolicyMask2        > for MitigationPolicy        { type Output = MitigationPolicyFlags2; fn bitand(self, rhs: MitigationPolicyMask2    ) -> MitigationPolicyFlags2 { self.flags2() & rhs } }
impl core::ops::BitAnd<MitigationPolicy             > for MitigationPolicyMask1   { type Output = MitigationPolicyFlags1; fn bitand(self, rhs: MitigationPolicy         ) -> MitigationPolicyFlags1 { self & rhs.flags1() } }
impl core::ops::BitAnd<MitigationPolicy             > for MitigationPolicyMask2   { type Output = MitigationPolicyFlags2; fn bitand(self, rhs: MitigationPolicy         ) -> MitigationPolicyFlags2 { self & rhs.flags2() } }

impl core::ops::BitOr<MitigationPolicy              > for MitigationPolicy        { type Output = MitigationPolicy; fn bitor (self, rhs: MitigationPolicy         ) -> MitigationPolicy { MitigationPolicy(self.0 | rhs.0, self.1 | rhs.1) } }
impl core::ops::BitOr<MitigationPolicyFlags1        > for MitigationPolicy        { type Output = MitigationPolicy; fn bitor (self, rhs: MitigationPolicyFlags1   ) -> MitigationPolicy { MitigationPolicy(self.flags1() | rhs, self.flags2()) } }
impl core::ops::BitOr<MitigationPolicyFlags2        > for MitigationPolicy        { type Output = MitigationPolicy; fn bitor (self, rhs: MitigationPolicyFlags2   ) -> MitigationPolicy { MitigationPolicy(self.flags1(), rhs | self.flags2()) } }
impl core::ops::BitOr<MitigationPolicy              > for MitigationPolicyFlags1  { type Output = MitigationPolicy; fn bitor (self, rhs: MitigationPolicy         ) -> MitigationPolicy { MitigationPolicy(rhs.flags1() | self, rhs.flags2()) } }
impl core::ops::BitOr<MitigationPolicy              > for MitigationPolicyFlags2  { type Output = MitigationPolicy; fn bitor (self, rhs: MitigationPolicy         ) -> MitigationPolicy { MitigationPolicy(rhs.flags1(), self | rhs.flags2()) } }

impl core::ops::BitAndAssign<MitigationPolicy       > for MitigationPolicy        { fn bitand_assign(&mut self, rhs: MitigationPolicy       ) { *self = *self & rhs } }
impl core::ops::BitAndAssign<MitigationPolicyMask   > for MitigationPolicy        { fn bitand_assign(&mut self, rhs: MitigationPolicyMask   ) { *self = *self & rhs } }
impl core::ops::BitAndAssign<MitigationPolicyFlags1 > for MitigationPolicy        { fn bitand_assign(&mut self, rhs: MitigationPolicyFlags1 ) { *self = (*self & rhs).into() } }
impl core::ops::BitAndAssign<MitigationPolicyFlags2 > for MitigationPolicy        { fn bitand_assign(&mut self, rhs: MitigationPolicyFlags2 ) { *self = (*self & rhs).into() } }
impl core::ops::BitAndAssign<MitigationPolicy       > for MitigationPolicyFlags1  { fn bitand_assign(&mut self, rhs: MitigationPolicy       ) { *self = *self & rhs } }
impl core::ops::BitAndAssign<MitigationPolicy       > for MitigationPolicyFlags2  { fn bitand_assign(&mut self, rhs: MitigationPolicy       ) { *self = *self & rhs } }
impl core::ops::BitAndAssign<MitigationPolicyMask1  > for MitigationPolicy        { fn bitand_assign(&mut self, rhs: MitigationPolicyMask1  ) { *self = (*self & rhs).into() } }
impl core::ops::BitAndAssign<MitigationPolicyMask2  > for MitigationPolicy        { fn bitand_assign(&mut self, rhs: MitigationPolicyMask2  ) { *self = (*self & rhs).into() } }
impl core::ops::BitAndAssign<MitigationPolicy       > for MitigationPolicyMask1   { fn bitand_assign(&mut self, rhs: MitigationPolicy       ) { *self = (*self & rhs).into() } }
impl core::ops::BitAndAssign<MitigationPolicy       > for MitigationPolicyMask2   { fn bitand_assign(&mut self, rhs: MitigationPolicy       ) { *self = (*self & rhs).into() } }

impl core::ops::BitOrAssign<MitigationPolicy        > for MitigationPolicy        { fn bitor_assign (&mut self, rhs: MitigationPolicy       ) { *self = *self | rhs } }
impl core::ops::BitOrAssign<MitigationPolicyFlags1  > for MitigationPolicy        { fn bitor_assign (&mut self, rhs: MitigationPolicyFlags1 ) { *self = *self | rhs } }
impl core::ops::BitOrAssign<MitigationPolicyFlags2  > for MitigationPolicy        { fn bitor_assign (&mut self, rhs: MitigationPolicyFlags2 ) { *self = *self | rhs } }

impl core::ops::BitOr<()> for MitigationPolicy                                  { type Output = MitigationPolicy; fn bitor (self, rhs: ()                                 ) -> Self::Output { MitigationPolicy::from(self) | MitigationPolicy::from(rhs) } }
impl core::ops::BitOr<MitigationPolicy> for ()                                  { type Output = MitigationPolicy; fn bitor (self, rhs: MitigationPolicy                   ) -> Self::Output { MitigationPolicy::from(self) | MitigationPolicy::from(rhs) } }
impl core::ops::BitOr<Option<core::convert::Infallible>> for MitigationPolicy   { type Output = MitigationPolicy; fn bitor (self, rhs: Option<core::convert::Infallible>  ) -> Self::Output { MitigationPolicy::from(self) | MitigationPolicy::from(rhs) } }
impl core::ops::BitOr<MitigationPolicy> for Option<core::convert::Infallible>   { type Output = MitigationPolicy; fn bitor (self, rhs: MitigationPolicy                   ) -> Self::Output { MitigationPolicy::from(self) | MitigationPolicy::from(rhs) } }

impl core::ops::Mul<bool> for MitigationPolicy                                  { type Output = MitigationPolicy; fn mul(self, rhs: bool            ) -> Self::Output { if rhs  { self } else { MitigationPolicy::default() } } }
impl core::ops::Mul<MitigationPolicy> for bool                                  { type Output = MitigationPolicy; fn mul(self, rhs: MitigationPolicy) -> Self::Output { if self { rhs  } else { MitigationPolicy::default() } } }



#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags Mask
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyMask1(u64);
#[doc(hidden)] #[deprecated = "renamed to MitigationPolicyMask1"] pub type MitigationPolicyFlags1Mask = MitigationPolicyMask1;

#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags1(u64);

flags!(impl .. for MitigationPolicyFlags1(u64) - MitigationPolicyMask1);



#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags Mask
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyMask2(u64);
#[doc(hidden)] #[deprecated = "renamed to MitigationPolicyMask2"] pub type MitigationPolicyFlags2Mask = MitigationPolicyMask2;

#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags2(u64);

flags!(impl .. for MitigationPolicyFlags2(u64) - MitigationPolicyMask2);
