//! Allocators, deallocators, etc.

#[cfg(alloc)] pub(crate) use alloc_::*;

mod allocator;                  pub use allocator::*;
mod cbox;                       pub use cbox::*;
mod cboxsized;                  pub use cboxsized::*;
mod cstring;                    pub use cstring::*;
mod narrow_or_wide;             pub use narrow_or_wide::*;

pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\heap_enable_termination_on_corruption.rs");
}
