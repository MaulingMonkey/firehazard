use crate::*;

use winapi::shared::guiddef::GUID;
use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header)\] ~ &ACE_HEADER
#[derive(Clone, Copy)] #[repr(transparent)] pub struct Ptr<'a>(*mut ace::Header, PhantomData<&'a ace::Header>);

impl Ptr<'_> {
    /// ### Safety
    /// `ace_header` should point to a valid [`ACE_HEADER`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header) for the lifetime `'a` given [`ace::Ptr<'a>`].
    pub const unsafe fn from_raw_unchecked(ace: *mut ace::Header) -> Self { Self(ace, PhantomData) }

    /// ### Safety
    /// `ace_header` should be null, or point to a valid [`ACE_HEADER`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header) for the lifetime `'a` given [`ace::Ptr<'a>`].
    pub unsafe fn from_raw(ace: *mut ace::Header) -> Option<Self> { if ace.is_null() { None } else { Some(Self(ace, PhantomData)) } }

    pub fn header(&self) -> &ace::Header { unsafe { &*self.0 } }
}

impl Debug for Ptr<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if self.0.is_null() { return write!(fmt, "NULL") }

        let header = unsafe { *self.0 };
        let mut d = fmt.debug_struct("ace::Ptr");
        d.field("header", &header);

        /// XXX: code bellow is chock full of improperly narrowing pointer spatial providence errors in the form:<br>
        /// `&DWORD` -> `*const DWORD` -> `*const SID` -> `*mut SID`
        ///
        /// Since the resulting `*mut SID` isn't directly dereferenced by Rust code, only indirectly via Win32 functions, in a read-only fashion, this is *probably* fine?
        ///
        /// https://doc.rust-lang.org/nightly/core/ptr/index.html#provenance
        macro_rules! sid_of { ( $ace:expr ) => {&{
            let sid_start : &u32 = &$ace.SidStart;
            sid::Ptr::from_raw_unchecked(sid_start as *const _ as *mut _)
        }}}

        struct Hex(u32);   impl Debug for Hex { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "0x{:08x}", self.0) } }
        struct Guid(GUID); impl Debug for Guid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{{...}}") } } // TODO: actually write out GUID

        // https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members
        match header.ty {
            ace::Type::ACCESS_ALLOWED => {
                let ace : &ACCESS_ALLOWED_ACE = unsafe { &*self.0.cast() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::ACCESS_ALLOWED_CALLBACK => {
                let ace : &ACCESS_ALLOWED_CALLBACK_ACE = unsafe { &*self.0.cast() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::ACCESS_ALLOWED_CALLBACK_OBJECT => {
                let ace : &ACCESS_ALLOWED_CALLBACK_OBJECT_ACE = unsafe { &*self.0.cast() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            // ace::Type::ACCESS_ALLOWED_COMPOUND => Reserved.
            ace::Type::ACCESS_ALLOWED_OBJECT => {
                let ace : &ACCESS_ALLOWED_OBJECT_ACE = unsafe { &*self.0.cast() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            ace::Type::ACCESS_DENIED => {
                let ace : &ACCESS_DENIED_ACE = unsafe { &*self.0.cast() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::ACCESS_DENIED_CALLBACK => {
                let ace : &ACCESS_DENIED_CALLBACK_ACE = unsafe { &*self.0.cast() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::ACCESS_DENIED_CALLBACK_OBJECT => {
                let ace : &ACCESS_DENIED_CALLBACK_OBJECT_ACE = unsafe { &*self.0.cast() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            ace::Type::ACCESS_DENIED_OBJECT => {
                let ace : &ACCESS_DENIED_OBJECT_ACE = unsafe { &*self.0.cast() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            // many reserved types
            ace::Type::SYSTEM_AUDIT => {
                let ace : &SYSTEM_AUDIT_ACE = unsafe { &*self.0.cast() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::SYSTEM_AUDIT_CALLBACK => {
                let ace : &SYSTEM_AUDIT_CALLBACK_ACE = unsafe { &*self.0.cast() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::SYSTEM_AUDIT_CALLBACK_OBJECT => {
                let ace : &SYSTEM_AUDIT_CALLBACK_OBJECT_ACE = unsafe { &*self.0.cast() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            ace::Type::SYSTEM_AUDIT_OBJECT => {
                let ace : &SYSTEM_AUDIT_OBJECT_ACE = unsafe { &*self.0.cast() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            ace::Type::SYSTEM_MANDATORY_LABEL => {
                let ace : &SYSTEM_MANDATORY_LABEL_ACE = unsafe { &*self.0.cast() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            _ => d.finish_non_exhaustive()
        }
    }
}
