use sandbox_windows_ffi::*;

use abistr::*;

use std::collections::*;
use std::path::PathBuf;

#[allow(dead_code)] // individual trust levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)] pub enum Integrity { Untrusted, Low, Medium, High, System }

impl Default for Integrity { fn default() -> Self { Integrity::Untrusted } }

impl Integrity {
    pub fn sid(self) -> sid::Ptr<'static> { match self {
        Self::Untrusted => sid!(S-1-16-0x0000),
        Self::Low       => sid!(S-1-16-0x1000),
        Self::Medium    => sid!(S-1-16-0x2000),
        Self::High      => sid!(S-1-16-0x3000),
        Self::System    => sid!(S-1-16-0x4000),
    }}
}



pub struct Token {
    pub integrity:      Integrity,
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
            restricted: Some(vec![sid!(S-1-0-0)]),
        }
    }
}



#[derive(Default)]
pub struct Allow {
    pub same_desktop:   bool,
    pub dynamic_code:   bool,
}



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
        let users       = sid!(S-1-5-32-545);
        let everyone    = sid!(S-1-1-0);
        let null        = sid!(S-1-0-0);

        let self_exe = std::path::PathBuf::from(std::env::args_os().next().expect("args[0] / exe"));
        let dir = self_exe.parent().unwrap();
        let manifest_dir = dir.ancestors().nth(2).unwrap(); // target/debug
        let build = if cfg!(debug_assertions) { "debug" } else { "release" };
        let arch = "x86_64-pc-windows-msvc"; // XXX

        let se_change_notify_privilege = lookup_privilege_value_a(cstr!("SeChangeNotifyPrivilege")).unwrap();
        let mut targets = vec![
            Target {
                exe: manifest_dir.join(format!(r"crates\no-std\target\{build}\trivial.exe")),
                allow: Allow::default(),
                spawn: Token {
                    // Minimal permissions to access e.g. `/KnownDlls` / `kernel32.dll` for init
                    privileges: [se_change_notify_privilege].into_iter().collect(),
                    enabled:    vec![everyone],
                    restricted: Some(vec![everyone]),
                    .. Default::default()
                },
                lockdown: Token {
                    .. Default::default()
                },
            },
            Target {
                exe: manifest_dir.join(format!(r"crates\no-std\target\{arch}\{build}\trivial.exe")),
                allow: Allow::default(),
                spawn: Token {
                    // Minimal permissions to access e.g. `/KnownDlls` / `kernel32.dll` for init
                    privileges: [se_change_notify_privilege].into_iter().collect(),
                    enabled:    vec![everyone],
                    restricted: Some(vec![everyone]),
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
                    integrity:  Integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, users, everyone, session],
                    restricted: Some(vec![user, users, everyone, session]),
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
                    // In rust ~nightly, this is fine, since `RtlGenRandom` will be used as a fallback whenever it fails thanks to this simplification PR:
                    // <https://github.com/rust-lang/rust/commit/46673bb08ffa22f21287349d966d875038e41b37>
                    // <https://github.com/rust-lang/rust/blob/1.61.0/library/std/src/sys/windows/rand.rs#L18>
                    // <https://github.com/rust-lang/rust/blob/1.63.0/library/std/src/sys/windows/rand.rs#L16>
                    // <https://github.com/rust-lang/rust/blob/2fbc08e2ce64dee45a29cb6133da6b32366268aa/library/std/src/sys/windows/rand.rs#L16>
                    enabled:    vec![session],
                    restricted: Some(vec![session]),
                    .. Default::default()
                },
            },
            Target {
                exe: dir.join("run-wasmer.exe"),
                allow: Allow {
                    dynamic_code: true,
                    .. Allow::default()
                },
                spawn: Token {
                    integrity:  Integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, users, everyone, session],
                    restricted: Some(vec![user, users, everyone, session]),
                    .. Default::default()
                },
                lockdown: Token {
                    enabled:    vec![session],
                    restricted: Some(vec![session]),
                    .. Default::default()
                },
            },
            Target {
                exe: dir.join("run-wasmtime.exe"),
                allow: Allow {
                    dynamic_code: true,
                    .. Allow::default()
                },
                spawn: Token {
                    integrity:  Integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, users, everyone, session],
                    restricted: Some(vec![user, users, everyone, session]),
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
                    integrity:  Integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, users, everyone, session],
                    restricted: Some(vec![user, users, everyone, session]),
                    .. Default::default()
                },
                lockdown: Token {
                    enabled:    vec![session],
                    restricted: Some(vec![session]),
                    .. Default::default()
                },
            },
        ];
        targets.retain(|t| t.exe.exists() || !t.exe.ends_with("trivial.exe"));
        targets.into_iter()
    }
}
