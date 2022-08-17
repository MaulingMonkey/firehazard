/// Debugger APIs from [debugapi.h](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/), ~~dbgeng.h~~ (not yet)

mod debug_event;                    pub use debug_event::*;

pub use funcs::*;
pub(crate) mod funcs {
    mod debugapi_h;                 pub use debugapi_h::*;
}
