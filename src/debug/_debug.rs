//! Debugger APIs from [debugapi.h](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/), <strike>dbgeng.h</strike> (not yet)

mod debug_event;                    pub use debug_event::*;

pub use funcs::*;
pub(crate) mod funcs {
    mod debugapi_h;                 pub use debugapi_h::*;
    mod memoryapi_h;                pub use memoryapi_h::*;
}
