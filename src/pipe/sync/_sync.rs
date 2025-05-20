//! `HANDLE`s to [pipe]s (created without [`file::FLAG_OVERLAPPED`])

use crate::prelude::*;
#[cfg(doc)] use crate::io::{Read, Write};

include!(r"conversion.rs");
include!(r"handles.rs");
include!(r"io.rs");
include!(r"open_existing.rs");
