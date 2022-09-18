//! [child_process], [desktop_app_breakaway], [mitigation_policy]\[[2](mitigation_policy2)\]

pub mod child_process;
pub mod desktop_app_breakaway;
pub mod mitigation_policy;
pub mod mitigation_policy2;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY Flags Mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ChildProcessPolicyFlagsMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY Flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ChildProcessPolicyFlags(u32);

flags!(impl .. for ChildProcessPolicyFlags(u32) - ChildProcessPolicyFlagsMask);



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY Flags Mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DesktopAppPolicyFlagsMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY Flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DesktopAppPolicyFlags(u32);

flags!(impl .. for DesktopAppPolicyFlags(u32) - DesktopAppPolicyFlagsMask);



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicy([u64; 2]);

impl From<MitigationPolicyFlags1> for MitigationPolicy { fn from(flags: MitigationPolicyFlags1) -> Self { Self([flags.0, 0]) } }
impl From<(MitigationPolicyFlags1, MitigationPolicyFlags2)> for MitigationPolicy { fn from(flags: (MitigationPolicyFlags1, MitigationPolicyFlags2)) -> Self { Self([flags.0.0, flags.1.0]) } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags Mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags1Mask(u64);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags1(u64);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags Mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags2Mask(u64);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY Flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MitigationPolicyFlags2(u64);

flags!(impl .. for MitigationPolicyFlags1(u64) - MitigationPolicyFlags1Mask);
flags!(impl .. for MitigationPolicyFlags2(u64) - MitigationPolicyFlags2Mask);
