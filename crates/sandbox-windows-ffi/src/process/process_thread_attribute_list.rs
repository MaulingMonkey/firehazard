#![allow(dead_code)]

use crate::*;
use crate::alloc::CBox;

use winapi::shared::basetsd::DWORD64;
use winapi::shared::minwindef::{FALSE, WORD, DWORD};
use winapi::shared::ntdef::*;
use winapi::um::processthreadsapi::*;
use winapi::um::winnt::SECURITY_CAPABILITIES;

use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::mem::{zeroed, size_of_val};
use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/process-and-thread-functions#process-and-thread-extended-attribute-functions)\]
/// Owned LPPROC_THREAD_ATTRIBUTE_LIST
#[repr(transparent)] pub struct ThreadAttributeList<'a>(CBox<PROC_THREAD_ATTRIBUTE_LIST>, PhantomData<&'a HANDLE>);

impl<'a> Drop for ThreadAttributeList<'a> { fn drop(&mut self) { unsafe { DeleteProcThreadAttributeList(self.0.as_mut_ptr()) } } }
impl Debug for ThreadAttributeList<'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "process::ThreadAttributeList {{ .. }}") } }

impl<'a> ThreadAttributeList<'a> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-initializeprocthreadattributelist)\]
    /// InitializeProcThreadAttributeList
    pub fn new() -> Self { Self::with_attribute_capacity(27).unwrap() }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-initializeprocthreadattributelist)\]
    /// InitializeProcThreadAttributeList
    ///
    /// ### Errors
    /// *   `ERROR_INVALID_PARAMETER` if `attributes` > `27` as of Windows 10.0.19043.1889
    pub fn with_attribute_capacity(attributes: u32) -> Result<Self, Error> {
        let mut bytes = 0;
        let _ = unsafe { InitializeProcThreadAttributeList(null_mut(), attributes, 0, &mut bytes) }; // fails w/ ERROR_INSUFFICIENT_BUFFER
        let mut cb = CBox::<PROC_THREAD_ATTRIBUTE_LIST>::new_oversized(unsafe{zeroed()}, bytes);
        Error::get_last_if(FALSE == unsafe { InitializeProcThreadAttributeList(cb.as_mut_ptr(), attributes, 0, &mut bytes) })?;
        Ok(Self(cb, PhantomData))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\] UpdateProcThreadAttribute
    pub fn update<'s, T: ?Sized>(&'s mut self, ThreadAttributeRef(attribute, value): ThreadAttributeRef<'a, T>) -> Result<&'s mut Self, Error> where 'a : 's {
        unsafe { self.update_attribute_impl(attribute, value)?; }
        Ok(self)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\] UpdateProcThreadAttribute
    unsafe fn update_attribute_impl<T: ?Sized>(&mut self, attribute: usize, value: &'a T) -> Result<(), Error> {
        Error::get_last_if(FALSE == unsafe { UpdateProcThreadAttribute(
            self.0.as_mut_ptr(),
            0,                          // flags            (reserved: must be 0)
            attribute,
            value as *const T as *mut T as _,
            size_of_val(value),
            null_mut(),                 // previous value   (reserved: must be null)
            null_mut(),                 // return size      (reserved: must be null)
        )})
    }
}

// TODO: type-erase ThreadAttributeRef s
// TODO: from slice of type-erased ThreadAttributeRef s



// pub unsafe trait AsThreadAttribute { ... }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\] UpdateProcThreadAttribute (attribute, &value) pair
pub struct ThreadAttributeRef<'a, T: ?Sized>(usize, &'a T);

impl<'a, T: ?Sized> ThreadAttributeRef<'a, T> {
    pub unsafe fn from_raw(attribute: usize, value: &'a T) -> Self { Self(attribute, value) }
}

impl<'a> ThreadAttributeRef<'a, ()> {
    /// (PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY, [GROUP_AFFINITY](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-group_affinity))
    pub fn group_affinity(value: &'a GROUP_AFFINITY) -> ThreadAttributeRef<'a, GROUP_AFFINITY> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY, value) }

    /// (PROC_THREAD_ATTRIBUTE_HANDLE_LIST, \[[handle::Owned]\])
    pub fn handle_list(value: &'a [handle::Owned]) -> ThreadAttributeRef<'a, [handle::Owned]> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_HANDLE_LIST, value) }

    /// (PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR, [PROCESSOR_NUMBER](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-processor_number))
    pub fn ideal_processor_ntdef(value: &'a winapi::shared::ntdef::PROCESSOR_NUMBER) -> ThreadAttributeRef<'a, winapi::shared::ntdef::PROCESSOR_NUMBER> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR, value) }
    /// (PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR, [PROCESSOR_NUMBER](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-processor_number))
    pub fn ideal_processor_winnt(value: &'a winapi::um::winnt::PROCESSOR_NUMBER) -> ThreadAttributeRef<'a, winapi::um::winnt::PROCESSOR_NUMBER> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR, value) }

    /// (PROC_THREAD_ATTRIBUTE_MACHINE_TYPE, [IMAGE_FILE_MACHINE_*](https://docs.microsoft.com/en-us/windows/win32/sysinfo/image-file-machine-constants))
    pub fn machine_type(value: &'a WORD) -> ThreadAttributeRef<'a, WORD> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_MACHINE_TYPE, value) }

    /// (PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, DWORD) - see also <code>[process::creation::mitigation_policy]::\*</code>
    pub fn mitigation_policy_dword(value: &'a DWORD) -> ThreadAttributeRef<'a, DWORD> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, value) }
    /// (PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, DWORD64) - see also <code>[process::creation::mitigation_policy]::\*</code>
    pub fn mitigation_policy_dword64(value: &'a DWORD64) -> ThreadAttributeRef<'a, DWORD64> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, value) }
    /// (PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, [DWORD64; 2]) - see also <code>[process::creation::mitigation_policy]\[[2](process::creation::mitigation_policy2)\]::\*</code>
    pub fn mitigation_policy_dword64_2(value: &'a [DWORD64; 2]) -> ThreadAttributeRef<'a, [DWORD64; 2]> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, value) }

    /// (PROC_THREAD_ATTRIBUTE_PARENT_PROCESS, [process::OwnedHandle])
    pub fn parent_process(value: &'a process::OwnedHandle) -> ThreadAttributeRef<'a, process::OwnedHandle> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_PARENT_PROCESS, value) }

    /// (PROC_THREAD_ATTRIBUTE_PREFERRED_NODE, u8)
    pub fn preferred_node(value: &'a u8) -> ThreadAttributeRef<'a, u8> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_PREFERRED_NODE, value) }

    // XXX: UMS_CREATE_THREAD_ATTRIBUTES not defined
    // /// (PROC_THREAD_ATTRIBUTE_UMS_THREAD, UMS_CREATE_THREAD_ATTRIBUTES)
    // pub unsafe fn ums_thread(value: &'a UMS_CREATE_THREAD_ATTRIBUTES) -> ThreadAttributeRef<'a, UMS_CREATE_THREAD_ATTRIBUTES> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_UMS_THREAD, value) }

    /// (PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, [SECURITY_CAPABILITIES](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_capabilities))
    pub unsafe fn security_capabilities_raw(value: &'a SECURITY_CAPABILITIES) -> ThreadAttributeRef<'a, SECURITY_CAPABILITIES> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, value) }

    /// (PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL, [PROTECTION_LEVEL_*](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_protection_level_information#members))
    pub fn protection_level(value: &'a DWORD) -> ThreadAttributeRef<'a, DWORD> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL, value) }

    /// (PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY, DWORD) - see also <code>[process::creation::child_process]::\*</code>
    pub fn child_process_policy(value: &'a DWORD) -> ThreadAttributeRef<'a, DWORD> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY, value) }

    /// (PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY, DWORD) - see also <code>[process::creation::desktop_app_breakaway]::\*</code>
    pub fn desktop_app_policy(value: &'a DWORD) -> ThreadAttributeRef<'a, DWORD> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY, value) }

    /// (PROC_THREAD_ATTRIBUTE_JOB_LIST, \[[job::OwnedHandle]\])
    pub fn job_list(value: &'a [job::OwnedHandle]) -> ThreadAttributeRef<'a, [job::OwnedHandle]> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_JOB_LIST, value) }

    /// (PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES, [XSTATE_*](https://docs.microsoft.com/en-us/windows/win32/debug/working-with-xstate-context))
    pub fn enable_optional_xstate_features(value: &'a DWORD64) -> ThreadAttributeRef<'a, DWORD64> { ThreadAttributeRef(PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES, value) }
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
