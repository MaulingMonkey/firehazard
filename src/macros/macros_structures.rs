/// ### Example
///
/// ```
/// # #[cfg(nope)] { // macro not exported from library
/// # use firehazard::*;
/// # use winapi::um::winnt::*;
/// structure!(@assert layout io::Counters => IO_COUNTERS {
///     read_operation_count    == ReadOperationCount,
///     write_operation_count   == WriteOperationCount,
///     other_operation_count   == OtherOperationCount,
///     read_transfer_count     == ReadTransferCount,
///     write_transfer_count    == WriteTransferCount,
///     other_transfer_count    == OtherTransferCount,
/// });
/// # }
/// ```
macro_rules! structure {
    ( @assert layout $a:ty => $b:ty { $( $af:ident == $bf:ident ),* $(,)? } ) => {
        const _ : () = assert!(core::mem::align_of::<$a>() == core::mem::align_of::<$b>(), "struct alignment mismatch");
        structure!( @assert layout -align $a => $b { $( $af == $bf ),* } );
    };
    ( @assert layout -align $a:ty => $b:ty { $( $af:ident == $bf:ident ),* $(,)? } ) => {
        const _ : () = assert!(core::mem::size_of ::<$a>() == core::mem::size_of ::<$b>(), "struct size mismatch");
        const _ : () = {
            let a = core::mem::MaybeUninit::<$a>::uninit();
            let b = core::mem::MaybeUninit::<$b>::uninit();
            let a = a.as_ptr();
            let b = b.as_ptr();
            $(
                let af = unsafe { core::ptr::addr_of!((*a).$af) };
                let bf = unsafe { core::ptr::addr_of!((*b).$bf) };
                assert!($crate::size_of_val_raw_sized(af) == $crate::size_of_val_raw_sized(bf), "field size mismatch");
            )*
            let _ = (a,b);
        };
        // XXX: #[test] to compare field offsets?
    };
}
