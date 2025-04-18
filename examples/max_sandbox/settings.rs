use firehazard::*;

use std::collections::*;
use std::path::PathBuf;



#[derive(Debug)]
pub struct Token {
    pub integrity:      sid::integrity::Level,
    pub privileges:     HashSet<privilege::Luid>,
    pub enabled:        Vec<sid::Ptr<'static>>,
    pub restricted:     Option<Vec<sid::Ptr<'static>>>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            integrity:  Default::default(),
            privileges: Default::default(),
            enabled:    Default::default(),
            restricted: Some(vec![sid::NULL]),
        }
    }
}



#[derive(Debug)]
pub struct Allow {
    pub same_desktop:   bool,
    pub dynamic_code:   bool,
    pub missing_cet:    bool,
}

impl Default for Allow {
    fn default() -> Self {
        Self {
            same_desktop:   false,
            dynamic_code:   false,
            missing_cet:    ! cfg!(cet),
        }
    }
}



#[derive(Debug)]
pub struct Target {
    pub exe:            PathBuf,
    pub allow:          Allow,
    pub spawn:          Token, // Used to initialize non-delayed DLLs, pre-main code
    pub lockdown:       Token, // Eventual lockdown settings
}

impl Target {
    #[allow(unused_variables)]
    pub fn list() -> impl Iterator<Item = Self> {
        let sandbox_process_token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
        let all         = Box::leak(Box::new(sandbox_process_token.groups_and_privileges().unwrap())).sids().iter().map(|s| s.sid).collect::<Vec<_>>();
        let user        = Box::leak(Box::new(sandbox_process_token.user().unwrap())).user().sid;
        let session     = Box::leak(Box::new(sandbox_process_token.logon_sid().unwrap())).groups().iter().next().unwrap().sid;

        let self_exe = std::path::PathBuf::from(std::env::args_os().next().expect("args[0] / exe"));
        let dir = self_exe.parent().unwrap();
        let manifest_dir = dir.ancestors().nth(2).unwrap(); // target/debug
        let build = if cfg!(debug_assertions) { "debug" } else { "release" };
        let arch = "x86_64-pc-windows-msvc"; // XXX

        let se_change_notify_privilege = privilege::name::CHANGE_NOTIFY.lookup_luid().unwrap();
        let mut targets = vec![
            Target {
                exe: manifest_dir.join(format!(r"crates\no-std\target\{build}\examples\trivial.exe")),
                allow: Allow{
                    missing_cet: true, // no-std doesn't have my fancy build.rs script enabling cet builds
                    .. Default::default()
                },
                spawn: Token {
                    // Minimal permissions to access e.g. `/KnownDlls` / `kernel32.dll` for init
                    privileges: [se_change_notify_privilege].into_iter().collect(),
                    enabled:    vec![sid::WORLD],
                    restricted: Some(vec![sid::WORLD]),
                    .. Default::default()
                },
                lockdown: Token {
                    .. Default::default()
                },
            },
            Target {
                exe: manifest_dir.join(format!(r"crates\no-std\target\{arch}\{build}\examples\trivial.exe")),
                allow: Allow{
                    missing_cet: true, // no-std doesn't have my fancy build.rs script enabling cet builds
                    .. Default::default()
                },
                spawn: Token {
                    // Minimal permissions to access e.g. `/KnownDlls` / `kernel32.dll` for init
                    privileges: [se_change_notify_privilege].into_iter().collect(),
                    enabled:    vec![sid::WORLD],
                    restricted: Some(vec![sid::WORLD]),
                    .. Default::default()
                },
                lockdown: Token {
                    .. Default::default()
                },
            },
            Target {
                exe: dir.join("less_trivial.exe"),
                allow: Allow::default(),
                spawn: Token {
                    integrity:  sid::integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, sid::builtin::alias::USERS, sid::WORLD, session],
                    restricted: Some(vec![user, sid::builtin::alias::USERS, sid::WORLD, session]),
                    .. Default::default()
                },
                lockdown: Token {
                    // `BCryptGenRandom` is unreliable unless the process token has `user` or `session` available.
                    // Specifically, the second call to it can fail with GetLastError()==ERROR_ACCESS_DENIED, even with
                    // identical params - due to `RegisterReopenWait` / `ReopenSystemPreferredRngCallback` failing.
                    // ```
                    // thread 'main' panicked at 'couldn't generate random bytes with preferred RNG: Os { code: 5, kind: PermissionDenied, message: "Access is denied." }'
                    // ```
                    // In rust 1.61.0, this is kinda fine, since `std::sys::windows::rand::hashmap_random_keys` only calls it once... per thread, which is less fine.
                    // In rust 1.63.0, this is broken, since `hashmap_random_keys` calls it once to check if it should use it, then again to actually use it.
                    // In rust ≈nightly, this is fine, since `RtlGenRandom` will be used as a fallback whenever it fails thanks to this simplification PR:
                    // <https://github.com/rust-lang/rust/commit/46673bb08ffa22f21287349d966d875038e41b37>
                    // <https://github.com/rust-lang/rust/blob/1.61.0/library/std/src/sys/windows/rand.rs#L18>
                    // <https://github.com/rust-lang/rust/blob/1.63.0/library/std/src/sys/windows/rand.rs#L16>
                    // <https://github.com/rust-lang/rust/blob/2fbc08e2ce64dee45a29cb6133da6b32366268aa/library/std/src/sys/windows/rand.rs#L16>
                    enabled:    vec![session],
                    restricted: Some(vec![session]),
                    .. Default::default()
                },
            },
            #[cfg(target_pointer_width = "64")] // wasmer has issues on 32-bit
            Target {
                exe: dir.join("run-wasmer.exe"),
                allow: Allow {
                    dynamic_code:   true,
                    missing_cet:    true, // `wasm` doesn't have my fancy build.rs script enabling cet builds
                    .. Allow::default()
                },
                spawn: Token {
                    integrity:  sid::integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, sid::builtin::alias::USERS, sid::WORLD, session],
                    restricted: Some(vec![user, sid::builtin::alias::USERS, sid::WORLD, session]),
                    .. Default::default()
                },
                lockdown: Token {
                    enabled:    vec![session],
                    restricted: Some(vec![session]),
                    .. Default::default()
                },
            },
            #[cfg(target_pointer_width = "64")] // wasmtime has issues on 32-bit
            Target {
                exe: dir.join("run-wasmtime.exe"),
                allow: Allow {
                    dynamic_code:   true,
                    missing_cet:    true, // `wasm` doesn't have my fancy build.rs script enabling cet builds
                    .. Allow::default()
                },
                spawn: Token {
                    integrity:  sid::integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, sid::builtin::alias::USERS, sid::WORLD, session],
                    restricted: Some(vec![user, sid::builtin::alias::USERS, sid::WORLD, session]),
                    .. Default::default()
                },
                lockdown: Token {
                    enabled:    vec![session],
                    restricted: Some(vec![session]),
                    .. Default::default()
                },
            },
            Target {
                exe: dir.join("ui_basic_window.exe"),
                allow: Allow {
                    same_desktop: true,
                    .. Allow::default()
                },
                spawn: Token {
                    integrity:  sid::integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, sid::builtin::alias::USERS, sid::WORLD, session],
                    restricted: Some(vec![user, sid::builtin::alias::USERS, sid::WORLD, session]),
                    .. Default::default()
                },
                lockdown: Token {
                    enabled:    vec![session],
                    restricted: Some(vec![session]),
                    .. Default::default()
                },
            },
            Target {
                exe: dir.join("ui_d3d9_window.exe"),
                allow: Allow {
                    same_desktop:   true,
                    missing_cet:    true, // D3DERR::NOTAVAILABLE (0x8876086A) unless allowed
                    .. Allow::default()
                },
                spawn: Token {
                    // "User" is semi-optional here, but lots of database access denied errors occur without it
                    integrity:  sid::integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, sid::builtin::alias::USERS, sid::WORLD, session],
                    restricted: Some(vec![user, sid::builtin::alias::USERS, sid::WORLD, session]),
                    .. Default::default()
                },

                // lockdown for ui_d3d9_window happens after Direct3DCreate9 but before CreateDevice
                //
                lockdown: Token {
                    // CreateDevice fails w/ D3DERR::NOTAVAILABLE (0x8876086A) unless we have: se_change_notify_privilege + USERS + WORLD
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![sid::builtin::alias::USERS, sid::WORLD, session],
                    restricted: Some(vec![sid::builtin::alias::USERS, sid::WORLD, session]),
                    .. Default::default()
                },
            },
            Target {
                exe: dir.join("ui_d3d11_window.exe"),
                allow: Allow {
                    same_desktop: true,
                    .. Allow::default()
                },
                spawn: Token {
                    integrity:  sid::integrity::Low,// else STATUS_DLL_INIT_FAILED (thrown from multiple threads threads + process)
                    privileges: [
                        se_change_notify_privilege, // else misc. scary nonfatal exceptions + debugger can't list dlls (would `revert_to_self` help?)
                    ].into_iter().collect(),
                    enabled: vec![
                        // required to avoid these nonfatal dxgi access denied errors (compat / user settings?):
                        // onecore\windows\directx\database\helperlibrary\lib\perappusersettingsqueryimpl.cpp(148)\dxgi.dll!00007FFB9C493F03: (caller: 00007FFB9C46DD72) ReturnHr(1) tid(3bb8) 80070005 Access is denied.
                        // onecore\windows\directx\database\helperlibrary\lib\perappusersettingsqueryimpl.cpp(108)\dxgi.dll!00007FFB9C493FCC: (caller: 00007FFB9C46DD72) ReturnHr(2) tid(3bb8) 80070005 Access is denied.
                        // onecore\windows\directx\database\helperlibrary\lib\perappusersettingsqueryimpl.cpp(98)\dxgi.dll!00007FFB9C493C9D: (caller: 00007FFB9C45DE61) ReturnHr(3) tid(3bb8) 80070005 Access is denied.
                        // onecore\windows\directx\database\helperlibrary\lib\directxdatabasehelper.cpp(1410)\dxgi.dll!00007FFB9C493DBD: (caller: 00007FFB9C45DE61) ReturnHr(4) tid(3bb8) 8000FFFF Catastrophic failure
                        //user,
                        sid::builtin::alias::USERS, // else STATUS_DLL_NOT_FOUND   (thrown from multiple threads threads + process)
                        sid::WORLD,                 // else STATUS_ACCESS_DENIED   (thrown from process only?)
                        session,                    // else STATUS_DLL_INIT_FAILED (thrown from multiple threads threads + process)
                    ],
                    restricted: Some(vec![
                        //user,
                        sid::builtin::alias::USERS, // else STATUS_DLL_NOT_FOUND   (thrown from multiple threads threads + process)
                        sid::WORLD,                 // else STATUS_ACCESS_DENIED   (thrown from process only?)
                        session,                    // else STATUS_DLL_INIT_FAILED (thrown from multiple threads threads + process)
                    ]),
                    .. Default::default()
                },

                // lockdown for ui_d3d11_window happens after D3D11CreateDeviceAndSwapChain.
                //
                // While the program won't crash at that point if absolutely no permissions are granted,
                // a few settings are required for rendering to succeed - without them, the window will
                // remain blank and white, as if e.g. `swap_chain.Present(...)` was never called.
                //
                // It might be possible to further restrict permissions after the first present,
                // I have not experimented with that.
                //
                lockdown: Token {
                    privileges: [
                        // I would say "DLL access", but lockdown happens after all relevant DLLs are loaded.
                        se_change_notify_privilege, // required to present?
                    ].into_iter().collect(),
                    enabled:    vec![
                        //user,                     // else nonfatal 1702 exceptions (presumably not "RPC_S_INVALID_BINDING"?)
                        sid::builtin::alias::USERS, // required to present?
                    ],
                    restricted: Some(vec![
                        //user,                     // else nonfatal 1702 exceptions (presumably not "RPC_S_INVALID_BINDING"?)
                        sid::builtin::alias::USERS, // required to present?
                    ]),
                    .. Default::default()
                },
            },
        ];
        targets.retain(|t| t.exe.exists() || !t.exe.ends_with("trivial.exe"));
        targets.into_iter()
    }
}
