/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy
pub fn set_process_mitigation_policy<P: crate::policy::IntoPolicy>(policy: P) -> Result<(), crate::Error> {
    use crate::Error;
    use winapi::um::processthreadsapi::*;
    use core::mem::size_of_val;

    let (ty, value) = policy.into();
    Error::get_last_if(0 == unsafe { SetProcessMitigationPolicy(ty as u32, &value as *const P::Policy as *mut _, size_of_val(&value)) })
}
