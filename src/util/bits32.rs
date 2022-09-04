use core::mem::size_of;

const _ : () = assert!(size_of::<u32>() <= size_of::<usize>(), "these traits should be removed if trying to support 16-bit");

pub(crate) trait From32<T> { fn from32(value: T) -> Self; }
impl From32<u32> for usize { #[inline(always)] fn from32(value: u32) -> usize { value as _ } }
