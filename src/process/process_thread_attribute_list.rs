#![allow(dead_code)]

use crate::prelude::*;
use crate::alloc::CBox;

use winapi::shared::basetsd::DWORD64;
use winapi::shared::minwindef::{FALSE, WORD, DWORD, LPVOID};
use winapi::shared::ntdef::*;
use winapi::um::processthreadsapi::*;
use winapi::um::winnt::SECURITY_CAPABILITIES;

use core::fmt::{self, Debug, Formatter};



#[doc(alias =   "PROC_THREAD_ATTRIBUTE_LIST")]
#[doc(alias = "LPPROC_THREAD_ATTRIBUTE_LIST")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/procthread/process-and-thread-functions#process-and-thread-extended-attribute-functions)\]
/// Owned LPPROC_THREAD_ATTRIBUTE_LIST
///
#[repr(transparent)] pub struct ThreadAttributeList<'a>(CBox<PROC_THREAD_ATTRIBUTE_LIST>, PhantomData<&'a HANDLE>);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-deleteprocthreadattributelist)\]
/// DeleteProcThreadAttributeList + HeapFree
///
impl<'a> Drop for ThreadAttributeList<'a> {
    fn drop(&mut self) {
        // To my understanding: this frees what's referenced by PROC_THREAD_ATTRIBUTE_LIST, but not the buffer containing the PROC_THREAD_ATTRIBUTE_LIST itself.
        unsafe { DeleteProcThreadAttributeList(self.0.as_mut_ptr()) }
        // The PROC_THREAD_ATTRIBUTE_LIST buffer will be dropped implicitly by `self.0`'s implicit [`CBox::drop`] as it goes out of scope.
    }
}

impl Debug for ThreadAttributeList<'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "process::ThreadAttributeList {{ .. }}") } }

impl<'a> ThreadAttributeList<'a> {
    #[doc(alias = "InitializeProcThreadAttributeList")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-initializeprocthreadattributelist)\]
    /// InitializeProcThreadAttributeList
    ///
    pub fn new() -> Self { Self::with_attribute_capacity(27).unwrap() }

    #[doc(alias = "InitializeProcThreadAttributeList")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-initializeprocthreadattributelist)\]
    /// InitializeProcThreadAttributeList
    ///
    /// ### Errors
    /// *   [`ERROR::INVALID_PARAMETER`]    &mdash; if `attributes` > `27` as of Windows 10.0.19043.1889
    ///
    pub fn with_attribute_capacity(attributes: u32) -> firehazard::Result<Self> {
        let mut bytes = 0;
        let _ = unsafe { InitializeProcThreadAttributeList(null_mut(), attributes, 0, &mut bytes) }; // fails w/ ERROR_INSUFFICIENT_BUFFER
        let mut cb = CBox::<PROC_THREAD_ATTRIBUTE_LIST>::new_oversized(Default::default(), bytes);
        firehazard::Error::get_last_if(FALSE == unsafe { InitializeProcThreadAttributeList(cb.as_mut_ptr(), attributes, 0, &mut bytes) })?;
        Ok(Self(cb, PhantomData))
    }

    #[doc(alias = "UpdateProcThreadAttribute")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
    /// UpdateProcThreadAttribute
    ///
    /// ### Errors
    /// -   [`ERROR::NOT_SUPPORTED`]    &mdash; if e.g. `PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER` isn't supported.
    /// -   [`ERROR::BAD_LENGTH`]       &mdash; if e.g. `PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER`'s value wasn't exactly 32 bits.
    /// -   `ERROR::???`                &mdash; the above errors are not an exhaustive list
    ///
    pub fn update<'s>(&'s mut self, ThreadAttributeRef(attribute, value, size, _): ThreadAttributeRef<'a>) -> firehazard::Result<&'s mut Self> where 'a : 's {
        firehazard::Error::get_last_if(FALSE == unsafe { UpdateProcThreadAttribute(
            self.0.as_mut_ptr(),
            0,              // flags            (reserved: must be 0)
            attribute,
            value,
            size,
            null_mut(),     // previous value   (reserved: must be null)
            null_mut(),     // return size      (reserved: must be null)
        )})?;
        Ok(self)
    }
}

impl<'a> TryFrom<&'_ [ThreadAttributeRef<'a>]> for ThreadAttributeList<'a> {
    type Error = firehazard::Error;
    fn try_from(refs: &'_ [ThreadAttributeRef<'a>]) -> Result<Self, Self::Error> {
        let len = refs.len().try_into().unwrap_or(!0u32);
        let mut list = Self::with_attribute_capacity(len)?;
        for r in refs.iter().copied() { list.update(r)?; }
        Ok(list)
    }
}



// pub unsafe trait AsThreadAttribute { ... }



#[doc(alias = "UpdateProcThreadAttribute")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\]
/// UpdateProcThreadAttribute (attribute, &value, size) tuple
///
#[derive(Clone, Copy)] pub struct ThreadAttributeRef<'a>(usize, LPVOID, usize, PhantomData<&'a usize>);

impl<'a> ThreadAttributeRef<'a> {
    #[doc(alias = "PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY")]
    /// (PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY, [GROUP_AFFINITY](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-group_affinity))
    ///
    pub fn group_affinity(value: &'a GROUP_AFFINITY) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_HANDLE_LIST")]
    /// (PROC_THREAD_ATTRIBUTE_HANDLE_LIST, &\[[handle::Borrowed]\])
    ///
    /// Note that [`create_process_as_user_a`] must specify `true` for `inherit_handles`,
    /// and the handles must also be marked as inheritable,
    /// or you'll get ERROR_INVALID_PARAMETER from [`create_process_as_user_a`].
    ///
    /// In other words: using this attribute strictly *narrows* what handles the child process inherits.
    ///
    pub fn handle_list(value: &'a [handle::Borrowed<'a>]) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_HANDLE_LIST, value) } }



    // TODO: ditch winapi versions in favor of custom struct?

    #[doc(alias = "PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR")]
    /// (PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR, [PROCESSOR_NUMBER](https://learn.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-processor_number))
    ///
    pub fn ideal_processor_ntdef(value: &'a winapi::shared::ntdef::PROCESSOR_NUMBER) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR, value) } }

    #[doc(alias = "PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR")]
    /// (PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR, [PROCESSOR_NUMBER](https://learn.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-processor_number))
    pub fn ideal_processor_winnt(value: &'a winapi::um::winnt::PROCESSOR_NUMBER) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_MACHINE_TYPE")]
    /// (PROC_THREAD_ATTRIBUTE_MACHINE_TYPE, [IMAGE_FILE_MACHINE_*](https://learn.microsoft.com/en-us/windows/win32/sysinfo/image-file-machine-constants))
    ///
    pub fn machine_type(value: &'a WORD) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_MACHINE_TYPE, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY")]
    /// (PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, [process::creation::MitigationPolicy]) - see also <code>[process::creation::mitigation_policy]::\*</code>,  <code>[process::creation::mitigation_policy2]::\*</code>
    ///
    pub fn mitigation_policy(value: &'a process::creation::MitigationPolicy) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_PARENT_PROCESS")]
    /// (PROC_THREAD_ATTRIBUTE_PARENT_PROCESS, [process::OwnedHandle])
    ///
    pub fn parent_process(value: &'a process::OwnedHandle) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_PARENT_PROCESS, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_PREFERRED_NODE")]
    /// (PROC_THREAD_ATTRIBUTE_PREFERRED_NODE, u8)
    ///
    pub fn preferred_node(value: &'a u8) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_PREFERRED_NODE, value) } }



    // XXX: UMS_CREATE_THREAD_ATTRIBUTES not defined
    // #[doc(alias = "UMS_CREATE_THREAD_ATTRIBUTES")]
    // /// (PROC_THREAD_ATTRIBUTE_UMS_THREAD, UMS_CREATE_THREAD_ATTRIBUTES)
    // ///
    // pub unsafe fn ums_thread(value: &'a UMS_CREATE_THREAD_ATTRIBUTES) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_UMS_THREAD, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES")]
    /// (PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, [SECURITY_CAPABILITIES](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_capabilities))
    ///
    pub unsafe fn security_capabilities_raw(value: &'a SECURITY_CAPABILITIES) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL")]
    /// (PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL, [PROTECTION_LEVEL_*](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_protection_level_information#members))
    ///
    pub fn protection_level(value: &'a DWORD) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY")]
    /// (PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY, DWORD) - see also <code>[process::creation::child_process]::\*</code>
    ///
    pub fn child_process_policy(value: &'a process::creation::ChildProcessPolicyFlags) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY")]
    /// (PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY, DWORD) - see also <code>[process::creation::desktop_app_breakaway]::\*</code>
    ///
    pub fn desktop_app_policy(value: &'a process::creation::DesktopAppPolicyFlags) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE")]
    /// (PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE, HPCON)
    ///
    pub fn pseudoconsole(pcon: &'a pseudoconsole::Owned) -> Self { unsafe { Self::from_raw_value(PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE, pcon.as_handle().cast(), size_of::<winapi::um::wincontypes::HPCON>()) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_JOB_LIST")]
    /// (PROC_THREAD_ATTRIBUTE_JOB_LIST, \[[job::OwnedHandle]\])
    ///
    pub fn job_list(value: &'a [job::Handle<'a>]) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_JOB_LIST, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES")]
    /// (PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES, [XSTATE_*](https://learn.microsoft.com/en-us/windows/win32/debug/working-with-xstate-context))
    ///
    pub fn enable_optional_xstate_features(value: &'a DWORD64) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES, value) } }



    #[doc(alias = "PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER")]
    /// (PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER, COMPONENT_* flags)
    ///
    /// ### 0
    /// Protect against nothing.
    ///
    /// ### COMPONENT_KTM (1 << 0)
    /// Blocks access to [Kernel Transaction Manager](https://learn.microsoft.com/en-us/windows/win32/ktm/kernel-transaction-manager-portal) APIs to mitigate
    /// [CVE-2018-8611](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-8611).
    ///
    /// References:
    /// *   <https://big5-sec.github.io/posts/component-filter-mitigation/>
    /// *   [https://research.nccgroup.com/2020/04/27/cve-2018-8611-exploiting-windows-ktm-part-1-5-introduction/](https://web.archive.org/web/20200427131306/https://research.nccgroup.com/2020/04/27/cve-2018-8611-exploiting-windows-ktm-part-1-5-introduction/)
    /// *   [https://research.nccgroup.com/2020/05/04/cve-2018-8611-exploiting-windows-ktm-part-2-5-patch-analysis-and-basic-triggering/](https://web.archive.org/web/20200510224136/https://research.nccgroup.com/2020/05/04/cve-2018-8611-exploiting-windows-ktm-part-2-5-patch-analysis-and-basic-triggering/)
    /// *   [https://research.nccgroup.com/2020/05/11/cve-2018-8611-exploiting-windows-ktm-part-3-5-triggering-the-race-condition-and-debugging-tricks/](https://web.archive.org/web/20200516173733/https://research.nccgroup.com/2020/05/11/cve-2018-8611-exploiting-windows-ktm-part-3-5-triggering-the-race-condition-and-debugging-tricks/)
    /// *   [https://research.nccgroup.com/2020/05/18/cve-2018-8611-exploiting-windows-ktm-part-4-5-from-race-win-to-kernel-read-and-write-primitive/](https://web.archive.org/web/20200525021855/https://research.nccgroup.com/2020/05/18/cve-2018-8611-exploiting-windows-ktm-part-4-5-from-race-win-to-kernel-read-and-write-primitive/)
    /// *   [https://research.nccgroup.com/2020/05/25/cve-2018-8611-exploiting-windows-ktm-part-5-5-vulnerability-detection-and-a-better-read-write-primitive/](https://web.archive.org/web/20211017104658/https://research.nccgroup.com/2020/05/25/cve-2018-8611-exploiting-windows-ktm-part-5-5-vulnerability-detection-and-a-better-read-write-primitive/)
    ///
    /// ### COMPONENT_??? (1 << 1)
    /// New components will presumably use new bits.
    /// Please [file an issue](https://github.com/MaulingMonkey/firehazard/issues) if you become aware of new components that aren't documented here!
    ///
    /// ### Platforms
    /// |     | SKU                                              | `cmd /C ver`                              |
    /// | --- | ------------------------------------------------ | ----------------------------------------- |
    /// | ✔️ | Windows Server 2022                              | 10.0.20348.3561                           |
    /// | ✔️ | Windows 10 Professional                          | 10.0.19045.5854 <br> 10.0.19043.2251      |
    /// | ❌ | Windows Server 2025 <br> Windows Server 2019     | 10.0.17763.3534                           |
    ///
    pub fn component_filter_flags(component_flags: &'a u32) -> Self { unsafe { Self::from_raw(PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER, component_flags) } }
    // Windows 10 (10.0.19043.2251) will report ERROR_BAD_LENGTH if you attempt to pass a u64 instead.
    // OTOH it would not suprise me if this is expanded to support larger sets of component flags in future versions of windows?



    // ==============================================  raw constructors  ==============================================



    /// Used to implement the vast majority of [`ThreadAttributeRef`] constructors.
    /// Generally, you should prefer one of the many *safe* static `fn`s that individually handle a specific attribute.
    ///
    /// Given: `ThreadAttributeRef::from_raw(attribute, &value)`
    ///
    /// Then: `UpdateProcThreadAttribute(..., attribute, &value, sizeof(value), ...)` will be called when this is converted into part of a thread attribute list.
    ///
    /// ### Errata
    ///
    /// Per [`microsoft/terminal#6705`](https://github.com/microsoft/terminal/issues/6705) as
    /// [brought to my attention by chrisd](https://discord.com/channels/273534239310479360/583054410670669833/1362882021860577380),
    /// `PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE` breaks from the typical pattern, and is kept broken for backwards compatability concerns:
    ///
    /// ```ignore
    /// // sane API design:
    /// UpdateProcThreadAttribute(.., PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE, &hpc, sizeof(hpc), ..)
    /// // *actually* expected (note the missing `&`):
    /// UpdateProcThreadAttribute(.., PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE, hpc, sizeof(hpc), ..)
    /// ```
    ///
    /// As such, do *not* call [`ThreadAttributeRef::from_raw`] for `PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE`.
    /// Instead, use [`ThreadAttributeRef::from_raw_value`], or better yet, use [`ThreadAttributeRef::pseudoconsole`].
    ///
    /// ### Safety
    /// -   `attribute` should be a valid `PROC_THREAD_ATTRIBUTE_*`, without any errata such as `PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE`.
    /// -   `value` must be of an appropriate type and value for `attribute`.
    ///
    pub unsafe fn from_raw<T: ?Sized + 'a>(attribute: usize, value: &'a T) -> Self {
        debug_assert!(
            attribute != PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE,
            "per https://github.com/microsoft/terminal/issues/6705 , PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE is quirky and passes a handle directly to UpdateProcThreadAttribute, rather than a pointer to the handle.  Use `from_raw_value` instead of `from_raw_ref`."
        );
        Self(attribute, value as *const _ as *mut _, size_of_val(value), PhantomData)
    }

    /// Generally, you should prefer one of the many *safe* static `fn`s that individually handle a specific attribute.
    ///
    /// Given: `ThreadAttributeRef::from_raw_value(attribute, value, size)`
    ///
    /// Then: `UpdateProcThreadAttribute(..., attribute, value, size, ...)` will be called when this is converted into part of a thread attribute list.
    ///
    pub unsafe fn from_raw_value(attribute: usize, value: LPVOID, size: usize) -> Self { Self(attribute, value, size, PhantomData) }
}



// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\WinBase.h

const PROC_THREAD_ATTRIBUTE_NUMBER    : usize = 0x0000FFFF;
const PROC_THREAD_ATTRIBUTE_THREAD    : usize = 0x00010000; // Attribute may be used with thread creation
const PROC_THREAD_ATTRIBUTE_INPUT     : usize = 0x00020000; // Attribute is input only
const PROC_THREAD_ATTRIBUTE_ADDITIVE  : usize = 0x00040000; // Attribute may be "accumulated," e.g. bitmasks, counters, etc.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(usize)] enum ProcThreadAttribute { // _PROC_THREAD_ATTRIBUTE_NUM
    ParentProcess                = 0,
    HandleList                   = 2,
    GroupAffinity                = 3,
    PreferredNode                = 4,
    IdealProcessor               = 5,
    UmsThread                    = 6,
    MitigationPolicy             = 7,
    SecurityCapabilities         = 9,
    ProtectionLevel              = 11,
    JobList                      = 13,
    ChildProcessPolicy           = 14,
    AllApplicationPackagesPolicy = 15,
    Win32kFilter                 = 16,
    SafeOpenPromptOriginClaim    = 17,
    DesktopAppPolicy             = 18,
    PseudoConsole                = 22,
    MitigationAuditPolicy        = 24,
    MachineType                  = 25,
    ComponentFilter              = 26,
    EnableOptionalXStateFeatures = 27,
    TrustedApp                   = 29,
}

#[allow(non_snake_case)] const fn ProcThreadAttributeValue(number: ProcThreadAttribute, thread: bool, input: bool, additive: bool) -> usize {
    0   | (number as usize & PROC_THREAD_ATTRIBUTE_NUMBER)
        | if thread   { PROC_THREAD_ATTRIBUTE_THREAD      } else { 0 }
        | if input    { PROC_THREAD_ATTRIBUTE_INPUT       } else { 0 }
        | if additive { PROC_THREAD_ATTRIBUTE_ADDITIVE    } else { 0 }
}

//                                                                                                                                                  thread input additive
const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS                  : usize = ProcThreadAttributeValue(ProcThreadAttribute::ParentProcess,                  false, true, false);
const PROC_THREAD_ATTRIBUTE_HANDLE_LIST                     : usize = ProcThreadAttributeValue(ProcThreadAttribute::HandleList,                     false, true, false);
const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY                  : usize = ProcThreadAttributeValue(ProcThreadAttribute::GroupAffinity,                  true,  true, false);
const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE                  : usize = ProcThreadAttributeValue(ProcThreadAttribute::PreferredNode,                  false, true, false);
const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR                 : usize = ProcThreadAttributeValue(ProcThreadAttribute::IdealProcessor,                 true,  true, false);
const PROC_THREAD_ATTRIBUTE_UMS_THREAD                      : usize = ProcThreadAttributeValue(ProcThreadAttribute::UmsThread,                      true,  true, false);
const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY               : usize = ProcThreadAttributeValue(ProcThreadAttribute::MitigationPolicy,               false, true, false);
const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES           : usize = ProcThreadAttributeValue(ProcThreadAttribute::SecurityCapabilities,           false, true, false);
const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL                : usize = ProcThreadAttributeValue(ProcThreadAttribute::ProtectionLevel,                false, true, false);
const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE                   : usize = ProcThreadAttributeValue(ProcThreadAttribute::PseudoConsole,                  false, true, false);
const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE                    : usize = ProcThreadAttributeValue(ProcThreadAttribute::MachineType,                    false, true, false);
const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES : usize = ProcThreadAttributeValue(ProcThreadAttribute::EnableOptionalXStateFeatures,   true,  true, false);
const PROC_THREAD_ATTRIBUTE_JOB_LIST                        : usize = ProcThreadAttributeValue(ProcThreadAttribute::JobList,                        false, true, false);
const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY            : usize = ProcThreadAttributeValue(ProcThreadAttribute::ChildProcessPolicy,             false, true, false);
const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY : usize = ProcThreadAttributeValue(ProcThreadAttribute::AllApplicationPackagesPolicy,   false, true, false);
const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER                   : usize = ProcThreadAttributeValue(ProcThreadAttribute::Win32kFilter,                   false, true, false);
const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY              : usize = ProcThreadAttributeValue(ProcThreadAttribute::DesktopAppPolicy,               false, true, false);
const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY         : usize = ProcThreadAttributeValue(ProcThreadAttribute::MitigationAuditPolicy,          false, true, false);
const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER                : usize = ProcThreadAttributeValue(ProcThreadAttribute::ComponentFilter,                false, true, false);
const PROC_THREAD_ATTRIBUTE_TRUSTED_APP                     : usize = ProcThreadAttributeValue(ProcThreadAttribute::TrustedApp,                     false, true, false);
