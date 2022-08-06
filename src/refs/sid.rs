use crate::error::LastError;
use crate::{From32, LocalString};

use abistr::{AsCStr, TryIntoAsCStr};

use winapi::shared::ntstatus::STATUS_SUCCESS;
use winapi::shared::sddl::{ConvertSidToStringSidA, ConvertStringSidToSidW, ConvertStringSidToSidA};
use winapi::shared::winerror::{ERROR_INVALID_PARAMETER, ERROR_INVALID_SID};
use winapi::um::lsalookup::{LSA_OBJECT_ATTRIBUTES, LSA_REFERENCED_DOMAIN_LIST, LSA_TRANSLATED_NAME};
use winapi::um::ntlsa::*;
use winapi::um::winnt::{SID, SID_AND_ATTRIBUTES};

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::mem::{align_of, size_of};
use std::ptr::null_mut;



pub struct LocalFree;
pub struct FreeSid;

pub unsafe trait SidDeallocator             { unsafe fn free(sid: *mut SID); }
unsafe impl SidDeallocator for LocalFree    { unsafe fn free(sid: *mut SID) { assert!(unsafe { winapi::um::winbase::LocalFree(sid.cast()) }.is_null()) } }
unsafe impl SidDeallocator for FreeSid      { unsafe fn free(sid: *mut SID) { assert!(unsafe { winapi::um::securitybaseapi::FreeSid(sid.cast()) }.is_null()) } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\] ~ Box<(SID, ???), LocalAllocFree>
#[repr(transparent)] pub struct Sid<D: SidDeallocator>(*mut SID, PhantomData<D>);

impl<D: SidDeallocator> Drop for Sid<D> {
    fn drop(&mut self) {
        unsafe { D::free(self.0) }
    }
}

impl<D: SidDeallocator> Debug for Sid<D> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Debug::fmt(&SidPtr(self.0, PhantomData), fmt)
    }
}

impl<'s, D: SidDeallocator> From<&'s Sid<D>> for SidPtr<'s> {
    fn from(sid: &'s Sid<D>) -> Self {
        Self(sid.0, PhantomData)
    }
}

impl<D: SidDeallocator> Sid<D> {
    /// ### Safety
    /// *   `sid` should be a valid [`LocalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc)ed buffer containing a valid [`SID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid).
    /// *   `sid` should not be [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)ed by anything else as [`Sid::from_raw`] takes ownership.
    /// *   As an exception to the above, `sid` may be null if you do nothing but drop the resulting [`Sid`]
    pub unsafe fn from_raw_unchecked(sid: *mut SID) -> Self { Self(sid, PhantomData) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosida)\] ConvertStringSidToSidA
pub fn convert_string_sid_to_sid_a(s: impl TryIntoAsCStr) -> Result<Sid<LocalFree>, LastError> {
    let s = s.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let mut sid = null_mut();
    let success = 0 != unsafe { ConvertStringSidToSidA(s.as_cstr(), &mut sid) };
    let sid = unsafe { Sid::from_raw_unchecked(sid.cast()) };

    if !success             { Err(LastError::get()) }
    else if sid.0.is_null() { Err(LastError(ERROR_INVALID_SID)) }
    else                    { Ok(sid) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosidw)\] ConvertStringSidToSidW
pub fn convert_string_sid_to_sid_w(s: impl TryIntoAsCStr<u16>) -> Result<Sid<LocalFree>, LastError> {
    let s = s.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let mut sid = null_mut();
    let success = 0 != unsafe { ConvertStringSidToSidW(s.as_cstr(), &mut sid) };
    let sid = unsafe { Sid::from_raw_unchecked(sid.cast()) };

    if !success             { Err(LastError::get()) }
    else if sid.0.is_null() { Err(LastError(ERROR_INVALID_SID)) }
    else                    { Ok(sid) }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\] ~ PSID
#[derive(Clone, Copy)] #[repr(transparent)] pub struct SidPtr<'a>(*mut SID, PhantomData<&'a SID>);
// TODO: consider merging SidPtr<'a> into Sid by introducing Borrower<'a> ?

impl SidPtr<'_> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsida)\] ConvertSidToStringSidA
    pub fn to_string_sid_a(&self) -> Option<LocalString> {
        if self.0.is_null() { return None }
        let mut local_string = null_mut();
        let succeeded = 0 != unsafe { ConvertSidToStringSidA(self.0.cast(), &mut local_string) };
        let local_string = unsafe { LocalString::from_raw(local_string) };
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
        assert!(STATUS_SUCCESS == unsafe { LsaOpenPolicy(system_name, &mut object_attributes, desired_access, &mut policy) });
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

impl<'a> Debug for SidPtr<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if self.0.is_null() { return write!(fmt, "SidPtr::NULL") }

        let sid = self.to_string_sid_a().unwrap();
        if let Some(lsa) = self.lsa_lookup_sids2() {
            write!(fmt, "{sid} {lsa:?}")
        } else {
            write!(fmt, "{sid} (no LSA name)")
        }
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes)\] SID_AND_ATTRIBUTES
#[repr(C)] #[derive(Clone, Copy)] pub struct SidAndAttributes<'a> {
    pub sid:        SidPtr<'a>,
    pub attributes: u32,
}

impl<'a> SidAndAttributes<'a> {
    pub fn new(sid: impl Into<SidPtr<'a>>, attributes: u32) -> Self {
        Self { sid: sid.into(), attributes }
    }
}

impl Debug for SidAndAttributes<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        struct AsHex(u32);
        impl Debug for AsHex { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "0x{:08x}", self.0) } }
        fmt.debug_struct("SidAndAttributes")
            .field("sid", &self.sid)
            .field("attributes", &AsHex(self.attributes))
            .finish()
    }
}

const _SID_AND_ATTRIBUTES_STATIC_ASSERTS : () = {
    assert!(align_of::<SID_AND_ATTRIBUTES>() == align_of::<SidAndAttributes>());
    assert!(size_of ::<SID_AND_ATTRIBUTES>() == size_of ::<SidAndAttributes>());
};
