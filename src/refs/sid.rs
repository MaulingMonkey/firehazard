// https://docs.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-lsalookupsids

use crate::From32;

use abistr::CStrPtr;

use winapi::shared::ntstatus::STATUS_SUCCESS;
use winapi::shared::sddl::ConvertSidToStringSidA;
use winapi::um::lsalookup::{LSA_OBJECT_ATTRIBUTES, LSA_REFERENCED_DOMAIN_LIST, LSA_TRANSLATED_NAME};
use winapi::um::ntlsa::*;
use winapi::um::winbase::LocalFree;
use winapi::um::winnt::{SID, LPSTR, SID_AND_ATTRIBUTES};

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::mem::{align_of, size_of};
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\] ~ PSID
#[repr(transparent)] pub struct SidPtr<'a>(*mut SID, PhantomData<&'a SID>);

impl SidPtr<'_> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsida)\] ConvertSidToStringSidA
    pub fn to_string_sid_a(&self) -> LocalString {
        let mut local_string = null_mut();
        let succeeded = 0 != unsafe { ConvertSidToStringSidA(self.0.cast(), &mut local_string) };
        let local_string = unsafe { LocalString::from_raw(local_string) };
        assert!(succeeded, "ConvertSidToStringSidA");
        local_string
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-lsalookupsids2)\] LsaLookupSids2
    pub fn lsa_lookup_sids2(&self) -> String {
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

        return result;
    }
}

impl<'a> Debug for SidPtr<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{} {:?}", self.to_string_sid_a(), self.lsa_lookup_sids2())
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes)\] SID_AND_ATTRIBUTES
#[repr(C)] pub struct SidAndAttributes<'a> {
    pub sid:        SidPtr<'a>,
    pub attributes: u32,
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



pub struct LocalString(LPSTR);
impl LocalString {
    pub const unsafe fn from_raw(raw: LPSTR) -> Self { Self(raw) }
    pub fn to_string_lossy<'s>(&'s self) -> Cow<'s, str> { self.as_cstr_ptr().to_string_lossy() }
}
impl LocalString {
    fn as_cstr_ptr<'s>(&'s self) -> CStrPtr<'s> { unsafe { CStrPtr::from_ptr_unbounded(self.0) } }
}
impl Drop       for LocalString { fn drop(&mut self) { assert!(unsafe { LocalFree(self.0.cast()) }.is_null()) } }
impl Debug      for LocalString { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{:?}", &*self.to_string_lossy()) } }
impl Display    for LocalString { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}", self.to_string_lossy()) } }
