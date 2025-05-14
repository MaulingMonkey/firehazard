//! \[~~microsoft.com~~\] Intended usage: use firehazard::prelude::\*;

#![allow(unused_imports)]

#[doc(no_inline)] pub use crate::*;
#[doc(no_inline)] pub use core::result::Result;

// re-inherit Error ambiguously
#[allow(ambiguous_glob_reexports)] pub use a::*;
#[allow(ambiguous_glob_reexports)] pub use b::*;
#[allow(ambiguous_glob_reexports)] pub use c::*;
mod a { #[doc(no_inline)] pub use core::error::Error; }
mod b { #[cfg(std)] #[doc(no_inline)] pub use std::io::Error; }
mod c { #[doc(no_inline)] pub use crate::Error; }



#[doc(no_inline)] pub(crate) use crate as firehazard;

#[doc(no_inline)] pub(crate) use core::{
    convert::Infallible,

    ptr::NonNull,
    ptr::null,
    ptr::null_mut,

    marker::PhantomData,

    mem::MaybeUninit,
    mem::align_of,
    mem::size_of,
    mem::size_of_val,
    mem::forget,
    mem::transmute,
};

#[doc(no_inline)] pub(crate) use abistr::{
    cstr8,
    cstr16,
    AsCStr,
    AsOptCStr,
    CStrBuf,
    CStrNonNull,
    CStrPtr,
    TryIntoAsCStr,
    TryIntoAsOptCStr,
    InteriorNulError,
    //NotNulTerminatedError,
};

#[doc(no_inline)] pub(crate) use winapi::{
    ctypes::c_void,
    shared::minwindef::{BOOL, FALSE, TRUE},
    shared::winerror::{SUCCEEDED, *},
};

#[doc(no_inline)] pub(crate) use winresult::{
    NtStatus,
    ERROR,
    STATUS,
};

#[doc(no_inline)] pub(crate) use util::{
    From32,
    none2null,
    slice_assume_init_mut,
    slice_assume_init_ref,
};
