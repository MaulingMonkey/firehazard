use winapi::shared::ntdef::HANDLE;
use winapi::um::minwinbase::*;

use core::marker::PhantomData;
use core::ops::Deref;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-debug_event)\]
/// DEBUG_EVENT
#[repr(transparent)]
pub struct DebugEvent(DEBUG_EVENT);

impl DebugEvent {
    pub unsafe fn from_raw(de: DEBUG_EVENT) -> Self { Self(de) }

    pub fn u(self) -> DebugEventU {
        match self.dwDebugEventCode {
            EXCEPTION_DEBUG_EVENT       => DebugEventU::Exception       (unsafe{*self.0.u.Exception()}),
            CREATE_THREAD_DEBUG_EVENT   => DebugEventU::CreateThread    (unsafe{*self.0.u.CreateThread()}),
            CREATE_PROCESS_DEBUG_EVENT  => DebugEventU::CreateProcess   (unsafe{*self.0.u.CreateProcessInfo()}),
            EXIT_THREAD_DEBUG_EVENT     => DebugEventU::ExitThread      (unsafe{*self.0.u.ExitThread()}),
            EXIT_PROCESS_DEBUG_EVENT    => DebugEventU::ExitProcess     (unsafe{*self.0.u.ExitProcess()}),
            LOAD_DLL_DEBUG_EVENT        => DebugEventU::LoadDll         (unsafe{*self.0.u.LoadDll()}),
            UNLOAD_DLL_DEBUG_EVENT      => DebugEventU::UnloadDll       (unsafe{*self.0.u.UnloadDll()}),
            OUTPUT_DEBUG_STRING_EVENT   => DebugEventU::DebugString     (unsafe{*self.0.u.DebugString()}),
            RIP_EVENT                   => DebugEventU::Rip             (unsafe{*self.0.u.RipInfo()}),
            _                           => DebugEventU::_NotHandled     (PhantomData),
        }
    }
}

// DebugEvent -> DEBUG_EVENT
impl Deref for DebugEvent { type Target = DEBUG_EVENT; fn deref(&self) -> &Self::Target { &self.0 } }
impl AsRef<DEBUG_EVENT> for DebugEvent { fn as_ref(&self) -> &DEBUG_EVENT { &self.0 } }
impl From<DebugEvent> for DEBUG_EVENT { fn from(de: DebugEvent) -> Self { de.0 } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-debug_event)\]
/// ~ DEBUG_EVENT::u
pub enum DebugEventU {
    Exception(EXCEPTION_DEBUG_INFO),
    CreateThread(CREATE_THREAD_DEBUG_INFO),
    CreateProcess(CREATE_PROCESS_DEBUG_INFO),
    ExitThread(EXIT_THREAD_DEBUG_INFO),
    ExitProcess(EXIT_PROCESS_DEBUG_INFO),
    LoadDll(LOAD_DLL_DEBUG_INFO),
    UnloadDll(UNLOAD_DLL_DEBUG_INFO),
    DebugString(OUTPUT_DEBUG_STRING_INFO),
    Rip(RIP_INFO),

    #[doc(hidden)] _NotHandled(PhantomData<HANDLE>)
}
