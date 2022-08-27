mod bits32;                     pub(crate) use bits32::*;
pub(crate) mod partition;

pub(crate) fn none2null<T>(_: Option<core::convert::Infallible>) -> *mut T { core::ptr::null_mut() }
