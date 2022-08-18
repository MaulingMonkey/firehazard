/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy
pub fn set_process_mitigation_policy<P: crate::policy::IntoPolicy>(policy: P) -> Result<(), crate::error::LastError> {
    use crate::error::LastError;
    use winapi::um::processthreadsapi::*;
    use std::mem::size_of_val;

    let (ty, value) = policy.into();
    LastError::get_if(0 == unsafe { SetProcessMitigationPolicy(ty, &value as *const P::Policy as *mut _, size_of_val(&value)) })
}
