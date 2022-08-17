use abistr::*;
use win32_security_playground::output_debug_string_a;

fn main() {
    // Pulls in for DoS resistant hash seeding:
    //  BCryptGenRandom                     (bcrypt.dll)
    //  SystemFunction036 aka RtlGenRandom  (cryptbase.dll) as a fallback
    // Which can cause permissions problems if permissions lock the process down too much:
    //  1. loading dlls         (can fail to even read the DLL into memory)
    //  2. initializing dlls    (can fail to finish DllMain)
    //  3. executing dlls       (can fail on bellow line due to internal access denied panic)
    dbg!(std::collections::HashMap::<u32, u32>::new());
    sandbox();
    dbg!(std::collections::HashMap::<u32, u32>::new());
}

fn sandbox() {
    output_debug_string_a(cstr!("sandbox"));
    std::thread::sleep(std::time::Duration::from_secs(1));
}
