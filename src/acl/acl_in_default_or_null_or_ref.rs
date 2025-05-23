use crate::prelude::*;



#[doc(alias = "PACL")]
#[doc(alias = "ACL")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/null-dacls-and-empty-dacls)\]
/// ≈ [`None`] → Default ACL
///
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Default;

impl core::fmt::Debug for Default { fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result { write!(fmt, "acl::Default") } }

#[doc(alias = "PACL")]
#[doc(alias = "ACL")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/null-dacls-and-empty-dacls)\]
/// ≈ [`None`] → NULL ACL (allows anything)
///
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Null;

impl core::fmt::Debug for Null { fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result { write!(fmt, "acl::Null") } }

impl From<Null> for firehazard::Error { fn from(_: Null) -> Self { ERROR::INVALID_ACL.into() } }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/null-dacls-and-empty-dacls)\]
/// One of [`acl::Null`] (allow all!), or [`acl::Ref`] (specific dacl)
///
pub trait InNullOrRef<'a> {
    /// | `self`            | returns   | usage     |
    /// | ------------------| ----------| ----------|
    /// | [`acl::Null`]     | `None`    | ❌ All access for everyone, no security! ❌
    /// | [`acl::Ref`]      | `Some(…)` | A custom ACL
    ///
    fn into_ptr(self) -> Option<acl::Ref<'a>>;
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/null-dacls-and-empty-dacls)\]
/// One of [`acl::Default`], [`acl::Null`] (allow all!), or [`acl::Ref`] (specific dacl)
///
pub trait InDefaultOrNullOrRef<'a> {
    /// | `self`            | returns           | usage     |
    /// | ------------------| ------------------| ----------|
    /// | [`acl::Default`]  | `(false, None)`   | Default ACL for object type
    /// | [`acl::Null`]     | `(true, None)`    | ❌ All access for everyone, no security! ❌
    /// | [`acl::Ref`]      | `(true, Some(…))` | A custom ACL
    ///
    fn into_present_ptr(self) -> (bool, Option<acl::Ref<'a>>);
}



impl<'a>                        InNullOrRef<'a> for acl::Null               { fn into_ptr(self) -> Option<acl::Ref<'a>> { None              } }
//pl<'a>                        InNullOrRef<'a> for acl::Ref<'a>            { fn into_ptr(self) -> Option<acl::Ref<'a>> { Some(self)        } }
impl<'a, T: Into<acl::Ref<'a>>> InNullOrRef<'a> for T                       { fn into_ptr(self) -> Option<acl::Ref<'a>> { Some(self.into()) } }

//pl<'a>                        InDefaultOrNullOrRef<'a> for ()             { fn into_present_ptr(self) -> (bool, Option<acl::Ref<'a>>) { (false, None              ) } }
impl<'a>                        InDefaultOrNullOrRef<'a> for acl::Default   { fn into_present_ptr(self) -> (bool, Option<acl::Ref<'a>>) { (false, None              ) } }
impl<'a>                        InDefaultOrNullOrRef<'a> for acl::Null      { fn into_present_ptr(self) -> (bool, Option<acl::Ref<'a>>) { (true,  None              ) } }
//pl<'a>                        InDefaultOrNullOrRef<'a> for acl::Ref<'a>   { fn into_present_ptr(self) -> (bool, Option<acl::Ref<'a>>) { (true,  Some(self)        ) } }
impl<'a, T: Into<acl::Ref<'a>>> InDefaultOrNullOrRef<'a> for T              { fn into_present_ptr(self) -> (bool, Option<acl::Ref<'a>>) { (true,  Some(self.into()) ) } }
