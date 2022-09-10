use crate::*;

use winapi::shared::winerror::*;
use winapi::shared::ntstatus::STATUS_SUCCESS;
use winapi::um::lsalookup::{LSA_OBJECT_ATTRIBUTES, LSA_REFERENCED_DOMAIN_LIST, LSA_TRANSLATED_NAME};
use winapi::um::ntlsa::*;
use winapi::um::securitybaseapi::EqualSid;
use winapi::um::winnt::{SID, PSID};

use core::fmt::{self, Debug, Formatter};
use core::hash::Hash;
use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\]
/// ~ SID
///
/// Should never be directly constructed - instead, allow various types to [`core::ops::Deref`] into a reference to this.
#[repr(transparent)] pub struct Value(*mut SID);
// DO NOT IMPLEMENT:
// * Clone, Copy: Given `&'a Value`, lifetime of the underlying SID may be limited to `'a` - these traits would allow a copy to escape to `'static`.

impl Value {
    // These fns return different types!  PSID is `*mut c_void`, not `*mut SID` !
    pub fn as_psid(&self) -> PSID { self.0.cast() }
    pub fn as_ptr_sid(&self) -> *mut SID { self.0 }

    pub fn revision(&self)  -> u8           { unsafe{*self.0}.Revision }
    pub fn authority(&self) -> [u8; 6]      { unsafe{*self.0}.IdentifierAuthority.Value }
    pub fn authority_u64(&self) -> u64      { let [a,b,c,d,e,f] = self.authority(); u64::from_be_bytes([0,0,a,b,c,d,e,f]) }
    pub fn subauthorities(&self) -> &[u32]  { unsafe{core::slice::from_raw_parts(core::ptr::addr_of!((*self.0).SubAuthority) as *const u32, (*self.0).SubAuthorityCount.into())} }
    fn as_tuple(&self) -> (u8, [u8; 6], &[u32]) { (self.revision(), self.authority(), self.subauthorities()) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-lsalookupsids2)\] LsaLookupSids2
    #[cfg(std)] pub fn lsa_lookup_sids2(&self) -> Result<std::string::String, Error> {
        if self.0.is_null() { return Err(Error(E_STRING_NOT_NULL_TERMINATED as _)) }
        // .cast() spam notes:
        // it appears PLSA_HANDLE points to void, not LSA_HANDLE, for whatever twisted reason.
        // it appears PSID points to void, not SID, for whatever twisted reason.

        let system_name = null_mut();
        let mut object_attributes = LSA_OBJECT_ATTRIBUTES::default();
        object_attributes.Length = core::mem::size_of_val(&object_attributes) as _;
        let desired_access = POLICY_LOOKUP_NAMES;
        let mut policy = null_mut();
        let ntstatus = unsafe { LsaOpenPolicy(system_name, &mut object_attributes, desired_access, &mut policy) };
        if ntstatus != STATUS_SUCCESS { return Err(Error(ntstatus as _)) }
        let policy = policy.cast(); // PSID -> *mut SID

        let lookup_options = LSA_LOOKUP_DISALLOW_CONNECTED_ACCOUNT_INTERNET_SID; // consider also: LSA_LOOKUP_PREFER_INTERNET_NAMES ?
        let mut domains = null_mut();
        let mut names = null_mut();
        let ntstatus = unsafe { LsaLookupSids2(policy, lookup_options, 1, [self.0.cast()].as_mut_ptr(), &mut domains, &mut names) };
        if ntstatus != STATUS_SUCCESS {
            assert_eq!(STATUS_SUCCESS, unsafe { LsaClose(policy) });
            return Err(Error(ntstatus as _));
        }

        let result = {
            let domains : &LSA_REFERENCED_DOMAIN_LIST = unsafe { &*domains };
            let _domains = unsafe { core::slice::from_raw_parts(domains.Domains, usize::from32(domains.Entries)) };
            let name : &LSA_TRANSLATED_NAME = unsafe { &*names };
            let name_name = unsafe { core::slice::from_raw_parts(name.Name.Buffer, (name.Name.Length/2).into()) }; // Length is in *bytes*, Buffer is *wchar_t* s
            //dbg!(domains.len());
            //dbg!(name.DomainIndex);
            //dbg!(name.Use);
            std::string::String::from_utf16_lossy(name_name)
        };

        assert_eq!(STATUS_SUCCESS, unsafe { LsaFreeMemory(domains.cast()) });
        assert_eq!(STATUS_SUCCESS, unsafe { LsaFreeMemory(names.cast()) });
        assert_eq!(STATUS_SUCCESS, unsafe { LsaClose(policy) });

        Ok(result)
    }

    #[cfg(not(std))] fn lsa_lookup_sids2(&self) -> Result<&'static str, Error> { Err(Error(ERROR_CALL_NOT_IMPLEMENTED)) }
}

impl PartialEq  for Value { fn eq(&self, other: &Self) -> bool { 0 != unsafe { EqualSid(self.as_psid(), other.as_psid()) } } }
impl Eq         for Value {}
impl PartialOrd for Value { fn partial_cmp  (&self, other: &Self) -> Option<core::cmp::Ordering> { self.as_tuple().partial_cmp   (&other.as_tuple()) } }
impl Ord        for Value { fn cmp          (&self, other: &Self) -> core::cmp::Ordering         { self.as_tuple().cmp           (&other.as_tuple()) } }
impl Hash       for Value { fn hash<H: core::hash::Hasher>(&self, state: &mut H) { self.as_tuple().hash(state) } }

impl Debug for Value {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if self.0.is_null() { return write!(fmt, "NULL") }
        write!(fmt, "S-{}-{}", self.revision(), self.authority_u64())?;
        for sa in self.subauthorities().iter().copied() { write!(fmt, "-{sa}")?; }
        if let Ok(lsa) = self.lsa_lookup_sids2() { write!(fmt, " {lsa:?}")?; }
        Ok(())
    }
}

#[cfg(std)] #[test] fn debug_fmt() {
    assert_eq!("S-1-2-3-4-5-6-7", std::format!("{:?}", sid!(S-1-2-3-4-5-6-7)));
}
