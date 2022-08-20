use crate::*;

use winapi::shared::ntstatus::STATUS_SUCCESS;
use winapi::shared::sddl::{ConvertSidToStringSidA};
use winapi::um::lsalookup::{LSA_OBJECT_ATTRIBUTES, LSA_REFERENCED_DOMAIN_LIST, LSA_TRANSLATED_NAME};
use winapi::um::ntlsa::*;
use winapi::um::securitybaseapi::EqualSid;
use winapi::um::winnt::{SID, PSID};

use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\]
/// ~ SID
///
/// Should never be directly constructed - instead, allow various types to [`std::ops::Deref`] into a reference to this.
#[repr(transparent)] pub struct Value(*mut SID);
// DO NOT IMPLEMENT:
// * Clone, Copy: Given `&'a Value`, lifetime of the underlying SID may be limited to `'a` - these traits would allow a copy to escape to `'static`.

impl Value {
    // These fns return different types!  PSID is `*mut c_void`, not `*mut SID` !
    pub fn as_psid(&self) -> PSID { self.0.cast() }
    pub fn as_ptr_sid(&self) -> *mut SID { self.0 }

    fn revision(&self)  -> u8           { unsafe{*self.0}.Revision }
    fn authority(&self) -> [u8; 6]      { unsafe{*self.0}.IdentifierAuthority.Value }
    fn subauthorities(&self) -> &[u32]  { unsafe{std::slice::from_raw_parts(std::ptr::addr_of!((*self.0).SubAuthority) as *const u32, (*self.0).SubAuthorityCount.into())} }
    fn as_tuple(&self) -> (u8, [u8; 6], &[u32]) { (self.revision(), self.authority(), self.subauthorities()) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsida)\] ConvertSidToStringSidA
    pub fn to_string_sid_a(&self) -> Option<alloc::CString<alloc::LocalAllocFree>> {
        if self.0.is_null() { return None }
        let mut local_string = null_mut();
        let succeeded = 0 != unsafe { ConvertSidToStringSidA(self.0.cast(), &mut local_string) };
        let local_string = unsafe { alloc::CString::from_raw(local_string) };
        assert!(succeeded, "ConvertSidToStringSidA");
        Some(local_string)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-lsalookupsids2)\] LsaLookupSids2
    pub fn lsa_lookup_sids2(&self) -> Option<String> {
        if self.0.is_null() { return None }
        // .cast() spam notes:
        // it appears PLSA_HANDLE points to void, not LSA_HANDLE, for whatever twisted reason.
        // it appears PSID points to void, not SID, for whatever twisted reason.

        let system_name = null_mut();
        let mut object_attributes = unsafe { std::mem::zeroed::<LSA_OBJECT_ATTRIBUTES>() };
        object_attributes.Length = std::mem::size_of_val(&object_attributes) as _;
        let desired_access = POLICY_LOOKUP_NAMES;
        let mut policy = null_mut();
        if STATUS_SUCCESS != unsafe { LsaOpenPolicy(system_name, &mut object_attributes, desired_access, &mut policy) } { return None }
        let policy = policy.cast(); // PSID -> *mut SID

        let lookup_options = LSA_LOOKUP_DISALLOW_CONNECTED_ACCOUNT_INTERNET_SID; // consider also: LSA_LOOKUP_PREFER_INTERNET_NAMES ?
        let mut domains = null_mut();
        let mut names = null_mut();
        assert!(STATUS_SUCCESS == unsafe { LsaLookupSids2(policy, lookup_options, 1, [self.0.cast()].as_mut_ptr(), &mut domains, &mut names) });

        let result = {
            let domains : &LSA_REFERENCED_DOMAIN_LIST = unsafe { &*domains };
            let _domains = unsafe { std::slice::from_raw_parts(domains.Domains, usize::from32(domains.Entries)) };
            let name : &LSA_TRANSLATED_NAME = unsafe { &*names };
            let name_name = unsafe { std::slice::from_raw_parts(name.Name.Buffer, (name.Name.Length/2).into()) }; // Length is in *bytes*, Buffer is *wchar_t* s
            //dbg!(domains.len());
            //dbg!(name.DomainIndex);
            //dbg!(name.Use);
            String::from_utf16_lossy(name_name)
        };

        assert!(STATUS_SUCCESS == unsafe { LsaFreeMemory(domains.cast()) });
        assert!(STATUS_SUCCESS == unsafe { LsaFreeMemory(names.cast()) });
        assert!(STATUS_SUCCESS == unsafe { LsaClose(policy) });

        return Some(result);
    }
}

impl PartialEq  for Value { fn eq(&self, other: &Self) -> bool { 0 != unsafe { EqualSid(self.as_psid(), other.as_psid()) } } }
impl Eq         for Value {}
impl PartialOrd for Value { fn partial_cmp  (&self, other: &Self) -> Option<std::cmp::Ordering> { self.as_tuple().partial_cmp   (&other.as_tuple()) } }
impl Ord        for Value { fn cmp          (&self, other: &Self) -> std::cmp::Ordering         { self.as_tuple().cmp           (&other.as_tuple()) } }
impl Hash       for Value { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.as_tuple().hash(state) } }

impl Debug for Value {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if self.0.is_null() { return write!(fmt, "NULL") }

        let sid = self.to_string_sid_a().unwrap();
        if let Some(lsa) = self.lsa_lookup_sids2() {
            write!(fmt, "{sid} {lsa:?}")
        } else {
            write!(fmt, "{sid}")
        }
    }
}
