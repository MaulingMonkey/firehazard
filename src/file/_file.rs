//! \[<strike>microsoft.com</strike>\]
//! APIs and constants related to file management



mod file_constants;             pub use file_constants::*;

pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\create_file.rs");
    include!(r"funcs\flush_file_buffers.rs");
    include!(r"funcs\get_file_attributes.rs");
    include!(r"funcs\get_file_type.rs");
    include!(r"funcs\get_final_path_name_by_handle.rs");
    include!(r"funcs\set_file_pointer.rs");
}
