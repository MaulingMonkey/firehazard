#[allow(unused_imports)] use winapi::um::winnt::*; // mostly for docs

macro_rules! well_known_sids {
    ($(
        $( #[desc = $desc:literal] )*
        #[values($rev:ident, $auth:ident $(, $subauth:ident)* $(,)?)]
        $( #[$($attr:meta),+] )*
        pub const $name:ident = sid!($($sid:tt)*);
    )*) => {
        #[allow(unused_imports)] use winapi::um::winnt::*;

        $(
            #[doc = concat!("`",$(stringify!($sid)),*,"`")]
            $( #[doc = concat!(" aka `", $desc, "`")] )*
            ///
            /// SID Components:
            #[doc = concat!("* [", stringify!($rev), "]")]
            #[doc = concat!("* [", stringify!($auth), "]")]
        $(  #[doc = concat!("* [", stringify!($subauth), "]")] )*
            $( #[$($attr),+] )*
            pub const $name : crate::sid::Ptr<'static> = crate::sid!($($sid)*);
        )*

        #[cfg(std)] #[test] fn validate_sid_values() {
            #[allow(unused_mut)] let mut errors = false;

            $(
                let name = stringify!($name);

                if $name.revision() != $rev {
                    std::println!("\u{001B}[31;1merror\u{001B}[0m: {name} revision is incorrect");
                    errors = true;
                }

                if $name.authority() != $auth {
                    std::println!("\u{001B}[31;1merror\u{001B}[0m: {name} authority is incorrect");
                    errors = true;
                }

                if $name.subauthorities() != &[$($subauth),*][..] {
                    std::println!("\u{001B}[31;1merror\u{001B}[0m: {name} subauthorities are incorrect");
                    errors = true;
                }

                let descs : &'static [&'static str] = &[ $($desc),* ];
                if let Ok(actual_desc) = $name.lsa_lookup_sids2() {
                    if !descs.contains(&&*actual_desc) {
                        std::println!("\u{001B}[31;1merror\u{001B}[0m: {name} documentation is missing actual description\n  documented: {descs:?}\n  actual:      {actual_desc:?}\n");
                        errors = true;
                    }
                }
            )*
            assert!(!errors);
        }
    };
}



// https://superuser.com/questions/884988/what-is-nt-authority-and-nt-service

well_known_sids! {
    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
    // "Universal well-known SIDs"
    #[desc = "NULL SID"]                        #[values(SID_REVISION, SECURITY_NULL_SID_AUTHORITY,    SECURITY_NULL_RID)]                  pub const NULL                  = sid!(S-1-0-0);
    #[desc = "Everyone"]                        #[values(SID_REVISION, SECURITY_WORLD_SID_AUTHORITY,   SECURITY_WORLD_RID)]                 pub const WORLD                 = sid!(S-1-1-0);
    #[desc = "LOCAL"]                           #[values(SID_REVISION, SECURITY_LOCAL_SID_AUTHORITY,   SECURITY_LOCAL_RID)]                 pub const LOCAL                 = sid!(S-1-2-0);
    #[desc = "CONSOLE LOGON"]                   #[values(SID_REVISION, SECURITY_LOCAL_SID_AUTHORITY,   SECURITY_LOCAL_LOGON_RID)]           pub const LOCAL_LOGON           = sid!(S-1-2-1); // ? aka "Console Logon"
    #[desc = "CREATOR OWNER"]                   #[values(SID_REVISION, SECURITY_CREATOR_SID_AUTHORITY, SECURITY_CREATOR_OWNER_RID)]         pub const CREATOR_ONWER         = sid!(S-1-3-0);
    #[desc = "CREATOR GROUP"]                   #[values(SID_REVISION, SECURITY_CREATOR_SID_AUTHORITY, SECURITY_CREATOR_GROUP_RID)]         pub const CREATOR_GROUP         = sid!(S-1-3-1);
    #[desc = "CREATOR OWNER SERVER"]            #[values(SID_REVISION, SECURITY_CREATOR_SID_AUTHORITY, SECURITY_CREATOR_OWNER_SERVER_RID)]  pub const CREATOR_OWNER_SERVER  = sid!(S-1-3-2);
    #[desc = "CREATOR GROUP SERVER"]            #[values(SID_REVISION, SECURITY_CREATOR_SID_AUTHORITY, SECURITY_CREATOR_GROUP_SERVER_RID)]  pub const CREATOR_GROUP_SERVER  = sid!(S-1-3-3);
    #[desc = "OWNER RIGHTS"]                    #[values(SID_REVISION, SECURITY_CREATOR_SID_AUTHORITY, SECURITY_CREATOR_OWNER_RIGHTS_RID)]  pub const CREATOR_OWNER_RIGHTS  = sid!(S-1-3-4);
}

/// `S-1-5-*` aka `NT AUTHORITY\*`
///
/// |  C    | Desc |
/// | ----- | ---- |
/// | 1     | [`SID_REVISION`]
/// | 5     | [`SECURITY_NT_AUTHORITY`]
/// | \*    | Various
pub mod nt_authority { well_known_sids! {
    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
    // "NT well-known SIDs"
    #[desc = "DIALUP"]                          #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_DIALUP_RID)]                         pub const DIALUP                    = sid!(S-1-5-1);
    #[desc = "NETWORK"]                         #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_NETWORK_RID)]                        pub const NETWORK                   = sid!(S-1-5-2);
    #[desc = "BATCH"]                           #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BATCH_RID)]                          pub const BATCH                     = sid!(S-1-5-3);
    #[desc = "INTERACTIVE"]                     #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_INTERACTIVE_RID)]                    pub const INTERACTIVE               = sid!(S-1-5-4);
    //#[desc = "NETWORK SERVICE"]               #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_LOGON_IDS_RID)]                      // Logon IDs                        = sid!(S-1-5-5-x-y); // aka LogonSessionId_x_yyyyyyyyy
    #[desc = "SERVICE"]                         #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_SERVICE_RID)]                        pub const SERVICE                   = sid!(S-1-5-6);
    #[desc = "ANONYMOUS LOGON"]                 #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_ANONYMOUS_LOGON_RID)]                pub const ANONYMOUS_LOGON           = sid!(S-1-5-7);
    #[desc = "PROXY"]                           #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_PROXY_RID)]                          pub const PROXY                     = sid!(S-1-5-8);
    #[desc = "ENTERPRISE DOMAIN CONTROLLERS"]   #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_ENTERPRISE_CONTROLLERS_RID)]         pub const ENTERPRISE_CONTROLLERS    = sid!(S-1-5-9);
    #[desc = "ENTERPRISE DOMAIN CONTROLLERS"]   #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_SERVER_LOGON_RID)]                   pub const SERVER_LOGON              = sid!(S-1-5-9); // n.b. same value as SECURITY_ENTERPRISE_CONTROLLERS_RID
    #[desc = "SELF"]                            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_PRINCIPAL_SELF_RID)]                 pub const PRINCIPAL_SELF            = sid!(S-1-5-10);
    #[desc = "Authenticated Users"]             #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_AUTHENTICATED_USER_RID)]             pub const AUTHENTICATED_USER        = sid!(S-1-5-11);
    #[desc = "RESTRICTED"]                      #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_RESTRICTED_CODE_RID)]                pub const RESTRICTED_CODE           = sid!(S-1-5-12);
    #[desc = "TERMINAL SERVER USER"]            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_TERMINAL_SERVER_RID)]                pub const TERMINAL_SERVER           = sid!(S-1-5-13);
    #[desc = "REMOTE INTERACTIVE LOGON"]        #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_REMOTE_LOGON_RID)]                   pub const REMOTE_LOGON              = sid!(S-1-5-14);
    #[desc = "This Organization"]               #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_THIS_ORGANIZATION_RID)]              pub const THIS_ORGANIZATION         = sid!(S-1-5-15);

    #[desc = "IUSR"]                            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_IUSER_RID)]                          pub const IUSER                     = sid!(S-1-5-17);
    #[desc = "SYSTEM"]                          #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_LOCAL_SYSTEM_RID)]                   pub const LOCAL_SYSTEM              = sid!(S-1-5-18);
    #[desc = "LOCAL SERVICE"]                   #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_LOCAL_SERVICE_RID)]                  pub const LOCAL_SERVICE             = sid!(S-1-5-19);
    #[desc = "NETWORK SERVICE"]                 #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_NETWORK_SERVICE_RID)]                pub const NETWORK_SERVICE           = sid!(S-1-5-20);

    //  const local users and groups        = sid!(S-1-5-21-x-y);       // SECURITY_NT_AUTHORITY    SECURITY_NT_NON_UNIQUE  x y     includes local named accounts, groups, like "MaulingMonkey" and "docker-users"
    //  const NtNonUnique                   = sid!(S-1-5-0x15-x-y);
    //  const EnterpriseReadOnlyControllers = sid!(S-1-5-0x16);         // SECURITY_NT_AUTHORITY    SECURITY_ENTERPRISE_READONLY_CONTROLLERS_RID
    //  const InstallerGroupCapability      = sid!(S-1-5-0x20);         // same as BuiltinDomain?
}}

/// `S-1-5-32-*` aka `BUILTIN\*`
///
/// |  C    | Desc |
/// | ----- | ---- |
/// | 1     | [`SID_REVISION`]
/// | 5     | [`SECURITY_NT_AUTHORITY`]
/// | 32    | [`SECURITY_BUILTIN_DOMAIN_RID`]
/// | \*    | DOMAIN_USER_RID_\* <br> DOMAIN_GROUP_\*
pub mod builtin {
    pub mod user {
        //! [ADMIN], [GUEST], ...
        well_known_sids! {
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_USER_RID_ADMIN)]              pub const ADMIN             = sid!(S-1-5-0x20-0x1F4);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_USER_RID_GUEST)]              pub const GUEST             = sid!(S-1-5-0x20-0x1F5);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_USER_RID_KRBTGT)]             pub const KRBTGT            = sid!(S-1-5-0x20-0x1F6);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_USER_RID_DEFAULT_ACCOUNT)]    pub const DEFAULT_ACCOUNT   = sid!(S-1-5-0x20-0x1F7);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_USER_RID_WDAG_ACCOUNT)]
            ///
            /// [DOMAIN_USER_RID_WDAG_ACCOUNT]: https://learn.microsoft.com/en-us/windows/security/threat-protection/microsoft-defender-application-guard/faq-md-app-guard#what-is-the-wdagutilityaccount-local-account-
            pub const WDAG_ACCOUNT = sid!(S-1-5-0x20-0x1F8);
        }

        // missing from winapi, sourced from C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
        #[cfg(test)] const DOMAIN_USER_RID_WDAG_ACCOUNT : u32 = 0x000001F8;
    }

    pub mod group {
        //! [ADMINS], [USERS], [GUESTS], ...
        well_known_sids! {
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_ADMINS)]                pub const ADMINS                    = sid!(S-1-5-0x20-0x200);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_USERS)]                 pub const USERS                     = sid!(S-1-5-0x20-0x201);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_GUESTS)]                pub const GUESTS                    = sid!(S-1-5-0x20-0x202);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_COMPUTERS)]             pub const COMPUTERS                 = sid!(S-1-5-0x20-0x203);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_CONTROLLERS)]           pub const CONTROLLERS               = sid!(S-1-5-0x20-0x204);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_CERT_ADMINS)]           pub const CERT_ADMINS               = sid!(S-1-5-0x20-0x205);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_SCHEMA_ADMINS)]         pub const SCHEMA_ADMINS             = sid!(S-1-5-0x20-0x206);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_ENTERPRISE_ADMINS)]     pub const ENTERPRISE_ADMINS         = sid!(S-1-5-0x20-0x207);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_POLICY_ADMINS)]         pub const POLICY_ADMINS             = sid!(S-1-5-0x20-0x208);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_READONLY_CONTROLLERS)]  pub const READONLY_CONTROLLERS      = sid!(S-1-5-0x20-0x209);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_CLONEABLE_CONTROLLERS)] pub const CLONEABLE_CONTROLLERS     = sid!(S-1-5-0x20-0x20A);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_CDC_RESERVED)]          pub const CDC_RESERVED              = sid!(S-1-5-0x20-0x20C);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_PROTECTED_USERS)]       pub const PROTECTED_USERS           = sid!(S-1-5-0x20-0x20D);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_KEY_ADMINS)]            pub const KEY_ADMINS                = sid!(S-1-5-0x20-0x20E);
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_ENTERPRISE_KEY_ADMINS)] pub const ENTERPRISE_KEY_ADMINS     = sid!(S-1-5-0x20-0x20F);
        }
    }

    pub mod alias {
        //! [ADMINS], [USERS], [GUESTS], ...
        well_known_sids! {
            #[desc = r"BUILTIN\Administrators"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_ADMINS)]
            pub const ADMINS = sid!(S-1-5-32-0x220);

            #[desc = r"BUILTIN\Users"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_USERS)]
            pub const USERS = sid!(S-1-5-0x20-0x221);

            #[desc = r"BUILTIN\Guests"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_GUESTS)]
            pub const GUESTS = sid!(S-1-5-0x20-0x222);

            #[desc = r"BUILTIN\Power Users"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_POWER_USERS)]
            pub const POWER_USERS = sid!(S-1-5-0x20-0x223);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_ACCOUNT_OPS)]
            pub const ACCOUNT_OPS = sid!(S-1-5-0x20-0x224);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_SYSTEM_OPS)]
            pub const SYSTEM_OPS = sid!(S-1-5-0x20-0x225);

            #[desc = r"BUILTIN\Print Operators"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_PRINT_OPS)]
            pub const PRINT_OPS = sid!(S-1-5-0x20-0x226);

            #[desc = r"BUILTIN\Backup Operators"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_BACKUP_OPS)]
            pub const BACKUP_OPS = sid!(S-1-5-0x20-0x227);



            #[desc = r"BUILTIN\Replicator"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_REPLICATOR)]
            pub const REPLICATOR = sid!(S-1-5-0x20-0x228);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_RAS_SERVERS)]
            pub const RAS_SERVERS = sid!(S-1-5-0x20-0x229);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_PREW2KCOMPACCESS)]
            pub const PREW2KCOMPACCESS = sid!(S-1-5-0x20-0x22A);

            #[desc = r"BUILTIN\Remote Desktop Users"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_REMOTE_DESKTOP_USERS)]
            pub const REMOTE_DESKTOP_USERS = sid!(S-1-5-0x20-0x22B);

            #[desc = r"BUILTIN\Network Configuration Operators"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_NETWORK_CONFIGURATION_OPS)]
            pub const NETWORK_CONFIGURATION_OPS = sid!(S-1-5-0x20-0x22C);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_INCOMING_FOREST_TRUST_BUILDERS)]
            pub const INCOMING_FOREST_TRUST_BUILDERS = sid!(S-1-5-0x20-0x22D);



            #[desc = r"BUILTIN\Performance Monitor Users"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_MONITORING_USERS)]
            pub const MONITORING_USERS = sid!(S-1-5-0x20-0x22E);

            #[desc = r"BUILTIN\Performance Log Users"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_LOGGING_USERS)]
            pub const LOGGING_USERS = sid!(S-1-5-0x20-0x22F);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_AUTHORIZATIONACCESS)]
            pub const AUTHORIZATIONACCESS = sid!(S-1-5-0x20-0x230);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_TS_LICENSE_SERVERS)]
            pub const TS_LICENSE_SERVERS = sid!(S-1-5-0x20-0x231);

            #[desc = r"BUILTIN\Distributed COM Users"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_DCOM_USERS)]
            pub const DCOM_USERS = sid!(S-1-5-0x20-0x232);

            #[desc = r"BUILTIN\IIS_IUSRS"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_IUSERS)]
            pub const IUSERS = sid!(S-1-5-0x20-0x238);

            #[desc = r"BUILTIN\Cryptographic Operators"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_CRYPTO_OPERATORS)]
            pub const CRYPTO_OPERATORS = sid!(S-1-5-0x20-0x239);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_CACHEABLE_PRINCIPALS_GROUP)]
            pub const CACHEABLE_PRINCIPALS_GROUP = sid!(S-1-5-0x20-0x23B);

            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_NON_CACHEABLE_PRINCIPALS_GROUP)]
            pub const NON_CACHEABLE_PRINCIPALS_GROUP = sid!(S-1-5-0x20-0x23C);

            #[desc = r"BUILTIN\Event Log Readers"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_EVENT_LOG_READERS_GROUP)]
            pub const EVENT_LOG_READERS_GROUP = sid!(S-1-5-0x20-0x23D);

            #[desc = r"BUILTIN\Certificate Service DCOM Access"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_CERTSVC_DCOM_ACCESS_GROUP)]
            pub const CERTSVC_DCOM_ACCESS_GROUP = sid!(S-1-5-0x20-0x23E);

            #[desc = r"BUILTIN\RDS Remote Access Servers"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_RDS_REMOTE_ACCESS_SERVERS)]
            pub const RDS_REMOTE_ACCESS_SERVERS = sid!(S-1-5-0x20-0x23F);

            #[desc = r"BUILTIN\RDS Endpoint Servers"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_RDS_ENDPOINT_SERVERS)]
            pub const RDS_ENDPOINT_SERVERS = sid!(S-1-5-0x20-0x240);

            #[desc = r"BUILTIN\RDS Management Servers"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_RDS_MANAGEMENT_SERVERS)]
            pub const RDS_MANAGEMENT_SERVERS = sid!(S-1-5-0x20-0x241);

            #[desc = r"BUILTIN\Hyper-V Administrators"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_HYPER_V_ADMINS)]
            pub const HYPER_V_ADMINS = sid!(S-1-5-0x20-0x242);

            #[desc = r"BUILTIN\Access Control Assistance Operators"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_ACCESS_CONTROL_ASSISTANCE_OPS)]
            pub const ACCESS_CONTROL_ASSISTANCE_OPS = sid!(S-1-5-0x20-0x243);

            #[desc = r"BUILTIN\Remote Management Users"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_REMOTE_MANAGEMENT_USERS)]
            pub const REMOTE_MANAGEMENT_USERS = sid!(S-1-5-0x20-0x244);

            #[desc = r"BUILTIN\System Managed Accounts Group"]
            #[desc = r"BUILTIN\System Managed Group"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_DEFAULT_ACCOUNT)]
            pub const DEFAULT_ACCOUNT = sid!(S-1-5-0x20-0x245);

            #[desc = r"BUILTIN\Storage Replica Administrators"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_STORAGE_REPLICA_ADMINS)]
            pub const STORAGE_REPLICA_ADMINS = sid!(S-1-5-0x20-0x246);

            #[desc = r"BUILTIN\Device Owners"]
            #[values(SID_REVISION, SECURITY_NT_AUTHORITY, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_DEVICE_OWNERS)]
            ///
            /// [DOMAIN_ALIAS_RID_DEVICE_OWNERS]:   https://learn.microsoft.com/en-us/windows/win32/secauthz/well-known-sids
            pub const DEVICE_OWNERS = sid!(S-1-5-0x20-0x247);
        }

        // missing from winapi, sourced from C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
        #[cfg(test)] const DOMAIN_ALIAS_RID_DEVICE_OWNERS : u32 = 0x00000247;
    }
}
