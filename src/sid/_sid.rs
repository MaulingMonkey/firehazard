//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\]
//! [`SID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
//! related types and manipulation functions

#[path = "sid_integrity.rs"] pub mod integrity;

mod sid_and_attributes;                 pub use sid_and_attributes::*;
mod sid_attributes;                     pub use sid_attributes::*;
mod sid_constants;                      pub use sid_constants::*;
mod sid_ptr;                            pub use sid_ptr::*;
mod sid_box;                            pub use sid_box::*;
mod sid_static;                         pub use sid_static::*;
mod sid_value;                          pub use sid_value::*;



pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\convert_sid_to_string_sid.rs");
    include!(r"funcs\convert_string_sid_to_sid.rs");
    include!(r"funcs\equal_sid.rs");
}
