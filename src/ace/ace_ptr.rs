use crate::prelude::*;

use winapi::shared::guiddef::GUID;
use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};



#[doc(alias = "ACE_HEADER")]
#[doc(alias = "PACE_HEADER")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header)\]
/// ≈ &ACE_HEADER
///
#[derive(Clone, Copy)] #[repr(transparent)] pub struct Ref<'a>(NonNull<ace::Header>, PhantomData<&'a ace::Header>);

impl Ref<'_> {
    /// ### Safety
    /// `ace_header` should point to a valid [`ACE_HEADER`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header) for the lifetime `'a` given [`ace::Ref<'a>`].
    pub const unsafe fn from_raw_unchecked(ace: NonNull<ace::Header>) -> Self { Self(ace, PhantomData) }

    /// ### Safety
    /// `ace_header` should be null, or point to a valid [`ACE_HEADER`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header) for the lifetime `'a` given [`ace::Ref<'a>`].
    pub unsafe fn from_raw(ace: *mut ace::Header) -> Option<Self> { NonNull::new(ace).map(|ace| unsafe { Self::from_raw_unchecked(ace) }) }

    pub fn header(&self) -> &ace::Header { unsafe { self.0.as_ref() } }
}

impl Debug for Ref<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let header = unsafe { self.0.read() };
        let mut d = fmt.debug_struct("ace::Ref");
        d.field("header", &header);

        macro_rules! sid_of { ( $ace:expr ) => {&{
            let sid_start : *const u32 = provenance_addr(self.0.as_ptr(), &$ace.SidStart);
            sid::Ptr::from_raw_unchecked(sid_start as *mut _)
        }}}

        struct Hex(u32);   impl Debug for Hex { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "0x{:08x}", self.0) } }
        struct Guid(GUID); impl Debug for Guid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
            let [a,b,c,d,e,f,g,h] = self.0.Data4;
            write!(fmt,
                "{{{:08X}-{:04X}-{:04X}-{a:02X}{b:02X}-{c:02X}{d:02X}{e:02X}{f:02X}{g:02X}{h:02X}}}",
                self.0.Data1, self.0.Data2, self.0.Data3,
            )
        }}

        // https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members
        match header.ty {
            ace::Type::ACCESS_ALLOWED => {
                let ace : &ACCESS_ALLOWED_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::ACCESS_ALLOWED_CALLBACK => {
                let ace : &ACCESS_ALLOWED_CALLBACK_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::ACCESS_ALLOWED_CALLBACK_OBJECT => {
                let ace : &ACCESS_ALLOWED_CALLBACK_OBJECT_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            // ace::Type::ACCESS_ALLOWED_COMPOUND => Reserved.
            ace::Type::ACCESS_ALLOWED_OBJECT => {
                let ace : &ACCESS_ALLOWED_OBJECT_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            ace::Type::ACCESS_DENIED => {
                let ace : &ACCESS_DENIED_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::ACCESS_DENIED_CALLBACK => {
                let ace : &ACCESS_DENIED_CALLBACK_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::ACCESS_DENIED_CALLBACK_OBJECT => {
                let ace : &ACCESS_DENIED_CALLBACK_OBJECT_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            ace::Type::ACCESS_DENIED_OBJECT => {
                let ace : &ACCESS_DENIED_OBJECT_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            // many reserved types
            ace::Type::SYSTEM_AUDIT => {
                let ace : &SYSTEM_AUDIT_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::SYSTEM_AUDIT_CALLBACK => {
                let ace : &SYSTEM_AUDIT_CALLBACK_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            ace::Type::SYSTEM_AUDIT_CALLBACK_OBJECT => {
                let ace : &SYSTEM_AUDIT_CALLBACK_OBJECT_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            ace::Type::SYSTEM_AUDIT_OBJECT => {
                let ace : &SYSTEM_AUDIT_OBJECT_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask",                     &Hex(ace.Mask)                  );
                d.field("flags",                    &Hex(ace.Flags)                 );
                d.field("object_type",              &Guid(ace.ObjectType)           );
                d.field("inherited_object_type",    &Guid(ace.InheritedObjectType)  );
                d.field("sid",                      unsafe { sid_of!(ace) }         );
                d.finish()
            },
            ace::Type::SYSTEM_MANDATORY_LABEL => {
                let ace : &SYSTEM_MANDATORY_LABEL_ACE = unsafe { self.0.cast().as_ref() };
                d.field("mask", &Hex(ace.Mask)).field("sid", unsafe { sid_of!(ace) }).finish()
            },
            _ => d.finish_non_exhaustive()
        }
    }
}
