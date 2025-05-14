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
    pub fn flag1(&self) -> MitigationPolicyFlags1 { self.0 }
    pub fn flag2(&self) -> MitigationPolicyFlags2 { self.1 }
}

impl From<MitigationPolicyFlags1> for MitigationPolicy { fn from(flags: MitigationPolicyFlags1) -> Self { Self(flags, MitigationPolicyFlags2(0)) } }
impl From<MitigationPolicyFlags2> for MitigationPolicy { fn from(flags: MitigationPolicyFlags2) -> Self { Self(MitigationPolicyFlags1(0), flags) } }
impl From<(MitigationPolicyFlags1, MitigationPolicyFlags2)> for MitigationPolicy { fn from(flags: (MitigationPolicyFlags1, MitigationPolicyFlags2)) -> Self { Self(flags.0, flags.1) } }

#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flag Mask
///
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] pub struct MitigationPolicyMask(MitigationPolicyFlags1Mask, MitigationPolicyFlags2Mask);

impl MitigationPolicyMask {
    pub fn mask1(&self) -> MitigationPolicyFlags1Mask { self.0 }
    pub fn mask2(&self) -> MitigationPolicyFlags2Mask { self.1 }
}

impl From<MitigationPolicyFlags1Mask> for MitigationPolicyMask { fn from(flags: MitigationPolicyFlags1Mask) -> Self { Self(flags, MitigationPolicyFlags2Mask(0)) } }
impl From<MitigationPolicyFlags2Mask> for MitigationPolicyMask { fn from(flags: MitigationPolicyFlags2Mask) -> Self { Self(MitigationPolicyFlags1Mask(0), flags) } }
impl From<(MitigationPolicyFlags1Mask, MitigationPolicyFlags2Mask)> for MitigationPolicyMask { fn from(flags: (MitigationPolicyFlags1Mask, MitigationPolicyFlags2Mask)) -> Self { Self(flags.0, flags.1) } }



#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags Mask
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags1Mask(u64);

#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags1(u64);

flags!(impl .. for MitigationPolicyFlags1(u64) - MitigationPolicyFlags1Mask);



#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags Mask
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags2Mask(u64);

#[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags2(u64);

flags!(impl .. for MitigationPolicyFlags2(u64) - MitigationPolicyFlags2Mask);
