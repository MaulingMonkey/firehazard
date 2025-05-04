//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/)\]
//! Error handling types and functions

use crate::prelude::*;

include!(r"error.rs");
include!(r"fast_fail.rs");
include!(r"handle_conversion_error.rs");
include!(r"preserve_error_scope.rs");
