mod bits32;                     pub(crate) use bits32::*;
pub(crate) mod partition;
mod provenance;                 pub(crate) use provenance::*;

pub(crate) fn none2null<T>(_: Option<core::convert::Infallible>) -> *mut T { core::ptr::null_mut() }
