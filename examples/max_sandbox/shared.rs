use firehazard::*;
use firehazard::process::creation::ChildProcessPolicyFlags;
use abistr::cstr16;
use winresult::ERROR;



/// Operating System supported features and flags.
#[derive(Clone, Debug)] pub struct OsSupported {
    /// Mask of supported [`component_filter_flags`](process::ThreadAttributeRef::component_filter_flags) such as `COMPONENT_KTM` which appear to be supported by the operating system.
    ///
    /// Observed values:
    /// *   [`None`]
    /// *   [`Some`]`(0xFFFFFFFF)`
    pub components: Option<u32>,

    // XXX: This is likely to be obnoxiously complicated to do 100% right.
    //pub mitigation_policy: Option<process::creation::MitigationPolicy>,

    child_process_policy: [bool; 3],
}

impl OsSupported {
    /// Test the currently running operating system for supported features and flags.
    pub fn query() -> Self {
        Self {
            components:             query_components(),
            child_process_policy:   query_child_process_policy(),
        }
    }

    pub fn is_child_process_policy_supported(&self, flags: ChildProcessPolicyFlags) -> bool {
        self.child_process_policy.get(usize::try_from(u32::from(flags)).unwrap_or(!0)).copied().unwrap_or(false)
    }
}

fn query_components() -> Option<u32> {
    if process::ThreadAttributeList::try_from(&[process::ThreadAttributeRef::component_filter_flags(&0)][..]).is_err() { return None }

    let mut supported_component_flags = 0;
    for component_index in 0 .. 32 {
        let component_flag : u32 = 1 << component_index;
        let Ok(tal) = process::ThreadAttributeList::try_from(&[process::ThreadAttributeRef::component_filter_flags(&component_flag)][..]) else { continue };
        let si = process::StartupInfoExW { attribute_list: Some(tal), .. Default::default() };
        match create_process_w(cstr16!(""), None, None, None, false, process::EXTENDED_STARTUPINFO_PRESENT, process::environment::Clear, (), &si) {
            Err(err) if err == ERROR::PATH_NOT_FOUND    => supported_component_flags |= component_flag,
            Err(err) if err == ERROR::INVALID_PARAMETER => {}, // component_flag not supported?
            Err(_rr)                                    => {}, // ???
            Ok(process)                                 => panic!("created process {pid} for \"\" while testing OS support for component filters", pid = process.process_id),
        }
    }
    Some(supported_component_flags)
}

fn query_child_process_policy() -> [bool; 3] {
    [
        (0, ChildProcessPolicyFlags::default()),
        (1, process::creation::child_process::RESTRICTED),
        (2, process::creation::child_process::OVERRIDE),
    ].map(|(i,policy)| {
        debug_assert_eq!(i, u32::from(policy));
        let Ok(tal) = process::ThreadAttributeList::try_from(&[process::ThreadAttributeRef::child_process_policy(&policy)][..]) else { return false };
        let si = process::StartupInfoExW { attribute_list: Some(tal), .. Default::default() };
        match create_process_w(cstr16!(""), None, None, None, false, process::EXTENDED_STARTUPINFO_PRESENT, process::environment::Clear, (), &si) {
            Err(err) if err == ERROR::PATH_NOT_FOUND    => true,
            Err(err) if err == ERROR::INVALID_PARAMETER => false, // ChildProcessPolicyFlags not supported?
            Err(_rr)                                    => false,
            Ok(process)                                 => panic!("created process {pid} for \"\" while testing OS support for ChildProcessPolicyFlags", pid = process.process_id),
        }
    })
}
