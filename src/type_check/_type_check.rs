use crate::prelude::*;
use core::sync::atomic::{AtomicU8, Ordering::*};
use winapi::um::processenv::GetEnvironmentVariableW;

const UNINIT : u8 = 2;

macro_rules! environment {
    ( $env:tt => always | debug | never ) => {{
        const COMPILE_TIME : Option<bool> = match option_env!($env) {
            None            => None,
            Some(env) => match env.as_bytes() {
                b"always"   => Some(true),
                b"debug"    => Some(cfg!(debug_assertions)),
                b"never"    => Some(false),
                _invalid    => panic!(concat!("invalid value for `%", $env, "%`: expected `always`, `debug`, or `never`.")),
            }
        };

        match COMPILE_TIME {
            Some(v) => v,
            None => {
                static RUN_TIME : AtomicU8 = AtomicU8::new(UNINIT);
                match RUN_TIME.load(Relaxed) {
                    0 => false,
                    1 => true,
                    _ => always_debug_never(&RUN_TIME, ::abistr::cstr16!($env)),
                }
            },
        }
    }};
}

fn always_debug_never(run_time: &AtomicU8, env: CStrNonNull<u16>) -> bool {
    let _dont_touch_last_error = crate::error::PreserveErrorScope::new();

    // no_std
    let mut buffer = [0u16; 16];
    let len32 = u32::try_from(buffer.len()).unwrap();
    let len32 = unsafe { GetEnvironmentVariableW(env.as_cstr(), buffer.as_mut_ptr(), len32) };
    let value = &buffer[..usize::from32(len32)];

    let value = if  value == cstr16!("always").to_units()   { true }
    else if         value == cstr16!("debug").to_units()    { cfg!(debug_assertions) }
    else if         value == cstr16!("never").to_units()    { false }
    else                                                    { true }; // XXX: bugchecks or warnings?

    match run_time.compare_exchange(UNINIT, value as u8, SeqCst, SeqCst) {
        Ok(_) => value,
        Err(prev) => match prev {
            0 => false,
            1 => true,
            _ => {
                // XXX: impossible?
                assert!(cfg!(not(debug_assertions)), "impossible previous run_time value for always_debug_never: {prev}");
                value
            }
        }
    }
}

// keep in sync with: doc\environment.md
#[allow(dead_code)] // XXX: casting handles not yet implemented
pub(crate) fn cast_handle()         -> bool { environment!("FIREHAZARD_TYPE_CHECK_CAST_HANDLE"          => always | debug | never) }
pub(crate) fn owned_from_raw()      -> bool { environment!("FIREHAZARD_TYPE_CHECK_OWNED_FROM_RAW"       => always | debug | never) }
pub(crate) fn borrowed_from_raw()   -> bool { environment!("FIREHAZARD_TYPE_CHECK_BORROWED_FROM_RAW"    => always | debug | never) }
