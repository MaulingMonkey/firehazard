//! Allocators, deallocators, etc.

mod allocator;                  pub use allocator::*;
mod cbox;                       pub use cbox::*;
mod cboxsized;                  pub use cboxsized::*;
mod cstring;                    pub use cstring::*;

pub use funcs::*;
pub(crate) mod funcs {
    include!("funcs/heap_enable_termination_on_corruption.rs");
}
