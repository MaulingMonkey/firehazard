//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\]
//! [`SID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
//! related types and manipulation functions

mod sid_and_attributes;                 pub use sid_and_attributes::*;
mod sid_ptr;                            pub use sid_ptr::*;
mod sid_box;                            pub use sid_box::*;



/// Re-exported into the global/root module
pub(crate) mod funcs {
    mod convert_string_sid_to_sid;  pub use convert_string_sid_to_sid::*;
}
