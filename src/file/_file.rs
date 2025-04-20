//! \[<strike>microsoft.com</strike>\]
//! APIs and constants related to file management



mod file_constants;             pub use file_constants::*;

pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\get_file_type.rs");
}
