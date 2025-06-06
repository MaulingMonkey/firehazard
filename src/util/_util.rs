mod assert_sendable;            pub(crate) use assert_sendable::*;
mod bits32;                     pub(crate) use bits32::*;
pub(crate) mod partition;
mod provenance;                 pub(crate) use provenance::*;
pub(crate) mod slice;
pub(crate) mod test;

#[allow(dead_code)] // XXX: Unlike the pending nightly fn, this acquires safety by sacrificing `?Sized` support.
pub(crate) const fn size_of_val_raw_sized<T>(_: *const T) -> usize { core::mem::size_of::<T>() }
pub(crate) fn none2null<T>(_: Option<core::convert::Infallible>) -> *mut T { core::ptr::null_mut() }
