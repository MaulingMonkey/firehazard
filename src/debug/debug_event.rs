use crate::prelude::*;

use winapi::shared::ntdef::HANDLE;
use winapi::um::minwinbase::*;

use core::ops::Deref;



#[deprecated = "use `firehazard::debug::Event` instead"     ] #[doc(hidden)] pub use Event  as DebugEvent;
#[deprecated = "use `firehazard::debug::EventU` instead"    ] #[doc(hidden)] pub use EventU as DebugEventU;

#[doc(alias = "DEBUG_EVENT")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-debug_event)\]
/// DEBUG_EVENT
///
#[repr(transparent)]
pub struct Event(DEBUG_EVENT);

impl Event {
    pub unsafe fn from_raw(de: DEBUG_EVENT) -> Self { Self(de) }

    pub fn u(self) -> EventU {
        match self.dwDebugEventCode {
            EXCEPTION_DEBUG_EVENT       => EventU::Exception       (unsafe{*self.0.u.Exception()}),
            CREATE_THREAD_DEBUG_EVENT   => EventU::CreateThread    (unsafe{*self.0.u.CreateThread()}),
            CREATE_PROCESS_DEBUG_EVENT  => EventU::CreateProcess   (unsafe{*self.0.u.CreateProcessInfo()}),
            EXIT_THREAD_DEBUG_EVENT     => EventU::ExitThread      (unsafe{*self.0.u.ExitThread()}),
            EXIT_PROCESS_DEBUG_EVENT    => EventU::ExitProcess     (unsafe{*self.0.u.ExitProcess()}),
            LOAD_DLL_DEBUG_EVENT        => EventU::LoadDll         (unsafe{*self.0.u.LoadDll()}),
            UNLOAD_DLL_DEBUG_EVENT      => EventU::UnloadDll       (unsafe{*self.0.u.UnloadDll()}),
            OUTPUT_DEBUG_STRING_EVENT   => EventU::DebugString     (unsafe{*self.0.u.DebugString()}),
            RIP_EVENT                   => EventU::Rip             (unsafe{*self.0.u.RipInfo()}),
            _                           => EventU::_NotHandled     (PhantomData),
        }
    }
}

// debug::Event -> DEBUG_EVENT
impl Deref for Event { type Target = DEBUG_EVENT; fn deref(&self) -> &Self::Target { &self.0 } }
impl AsRef<DEBUG_EVENT> for Event { fn as_ref(&self) -> &DEBUG_EVENT { &self.0 } }
impl From<Event> for DEBUG_EVENT { fn from(de: Event) -> Self { de.0 } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-debug_event)\]
/// ≈ DEBUG_EVENT::u
pub enum EventU {
    #[doc(alias = "EXCEPTION_DEBUG_EVENT")]
    /// EXCEPTION_DEBUG_EVENT → [EXCEPTION_DEBUG_INFO]
    ///
    Exception(EXCEPTION_DEBUG_INFO),

    #[doc(alias = "CREATE_THREAD_DEBUG_EVENT")]
    /// CREATE_THREAD_DEBUG_EVENT → [CREATE_THREAD_DEBUG_INFO]
    ///
    CreateThread(CREATE_THREAD_DEBUG_INFO),

    #[doc(alias = "CREATE_PROCESS_DEBUG_EVENT")]
    /// CREATE_PROCESS_DEBUG_EVENT → [CREATE_PROCESS_DEBUG_INFO]
    ///
    CreateProcess(CREATE_PROCESS_DEBUG_INFO),

    #[doc(alias = "EXIT_THREAD_DEBUG_EVENT")]
    /// EXIT_THREAD_DEBUG_EVENT → [EXIT_THREAD_DEBUG_INFO]
    ///
    ExitThread(EXIT_THREAD_DEBUG_INFO),

    #[doc(alias = "EXIT_PROCESS_DEBUG_EVENT")]
    /// EXIT_PROCESS_DEBUG_EVENT → [EXIT_PROCESS_DEBUG_INFO]
    ///
    ExitProcess(EXIT_PROCESS_DEBUG_INFO),

    #[doc(alias = "LOAD_DLL_DEBUG_EVENT")]
    /// LOAD_DLL_DEBUG_EVENT → LOAD_DLL_DEBUG_INFO
    ///
    LoadDll(LOAD_DLL_DEBUG_INFO),

    #[doc(alias = "UNLOAD_DLL_DEBUG_EVENT")]
    /// UNLOAD_DLL_DEBUG_EVENT → [UNLOAD_DLL_DEBUG_INFO]
    ///
    UnloadDll(UNLOAD_DLL_DEBUG_INFO),

    #[doc(alias = "OUTPUT_DEBUG_STRING_EVENT")]
    /// OUTPUT_DEBUG_STRING_EVENT → [OUTPUT_DEBUG_STRING_INFO]
    ///
    DebugString(OUTPUT_DEBUG_STRING_INFO),

    #[doc(alias = "RIP_EVENT")]
    /// RIP_EVENT → [RIP_INFO]
    ///
    Rip(RIP_INFO),

    #[doc(hidden)] _NotHandled(PhantomData<HANDLE>)
}
