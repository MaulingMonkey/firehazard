#[doc(alias = "CompareObjectHandles")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-compareobjecthandles)\]
/// <strike>CompareObjectHandles</strike>
///
#[cfg(doc)] /// NYI (windows SDK too early to link this API?)
pub fn compare_object_handles(first: &firehazard::handle::Owned, second: &firehazard::handle::Owned) -> bool {
    use firehazard::*;

    // #[link(name = "kernelbase")] extern {} // unable to link against kernelbase?
    0 != unsafe { winapi::um::handleapi::CompareObjectHandles(first.as_handle(), second.as_handle()) }
}
