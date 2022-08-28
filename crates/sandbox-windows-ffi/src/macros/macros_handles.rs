/// ### Usage
/// ```no_compile
/// handles!(impl *LocalHandle<HANDLE> for token::{Owned});
/// handles!(impl *LocalHandle<HANDLE> for token::{Owned, Borrowed});
/// handles!(impl *LocalHandle<HANDLE> for token::{Owned, Borrowed, Psuedo});
///
/// handles!(impl Debug for token::{Owned});
/// handles!(impl Debug for token::{Owned, Borrowed});
/// handles!(impl Debug for token::{Owned, Borrowed, Psuedo});
/// ```
macro_rules! handles {
    (impl *LocalHandleNN<$raw:ty> for $mod:ident :: { $owned:ident $(,$( $borrowed:ident $(,$( $psuedo:ident )?)? )?)? } ) => {
            impl FromLocalHandle<*mut $raw> for $owned       { unsafe fn from_raw(handle: *mut $raw) -> Result<Self, Error> { Ok(Self(core::ptr::NonNull::new(handle).ok_or(Error(winapi::shared::winerror::ERROR_INVALID_HANDLE))?)) } }
        $($(
        $($(
            impl FromLocalHandle<*mut $raw> for $psuedo<'_>  { unsafe fn from_raw(handle: *mut $raw) -> Result<Self, Error> { Ok(Self(core::ptr::NonNull::new(handle).ok_or(Error(winapi::shared::winerror::ERROR_INVALID_HANDLE))?, core::marker::PhantomData)) } }
        )?)?
        )?)?

            impl AsLocalHandleNN<$raw> for $owned         { fn as_handle_nn(&self) -> core::ptr::NonNull<$raw> { self.0 } }
        $($(
            impl AsLocalHandleNN<$raw> for $borrowed<'_>  { fn as_handle_nn(&self) -> core::ptr::NonNull<$raw> { self.0 } }
        $($(
            impl AsLocalHandleNN<$raw> for $psuedo<'_>    { fn as_handle_nn(&self) -> core::ptr::NonNull<$raw> { self.0 } }
        )?)?
        )?)?
    };

    (impl Debug for $mod:ident :: { $owned:ident $(,$( $borrowed:ident $(,$( $psuedo:ident )?)? )?)? } ) => {
            impl core::fmt::Debug for $owned        { fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result { write!(fmt, "{mo}::{ty}(0x{value:08x})", mo=stringify!($mod), ty=stringify!($owned    ), value=self.0.as_ptr() as usize) } }
        $($(
            impl core::fmt::Debug for $borrowed<'_> { fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result { write!(fmt, "{mo}::{ty}(0x{value:08x})", mo=stringify!($mod), ty=stringify!($borrowed ), value=self.0.as_ptr() as usize) } }
        $($(
            impl core::fmt::Debug for $psuedo<'_>   { fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result { write!(fmt, "{mo}::{ty}(0x{value:08x})", mo=stringify!($mod), ty=stringify!($psuedo   ), value=self.0.as_ptr() as usize) } }
        )?)?
        )?)?
    };

    (impl {AsRef<@base>, From} for $mod:ident :: { $owned:ident $(,$( $borrowed:ident $(,$( $psuedo:ident )?)? )?)? } ) => {
            impl     AsRef<crate::handle::Owned       > for     $owned    { fn as_ref(&self) -> &crate::handle::Owned        { unsafe { core::mem::transmute(self) } } }
            impl<'a> AsRef<crate::handle::Borrowed<'a>> for &'a $owned    { fn as_ref(&self) -> &crate::handle::Borrowed<'a> { unsafe { core::mem::transmute(*self) } } }
            impl<'a> AsRef<crate::handle::Psuedo<'a>  > for &'a $owned    { fn as_ref(&self) -> &crate::handle::Psuedo<'a>   { unsafe { core::mem::transmute(*self) } } }

            impl     From<    $owned    > for     crate::handle::Owned    { fn from(h:     $owned ) -> Self { unsafe { core::mem::transmute(h) } } }
            impl<'a> From<&'a $owned    > for &'a crate::handle::Owned    { fn from(h: &'a $owned ) -> Self { unsafe { core::mem::transmute(h) } } }
            impl<'a> From<&'a $owned    > for crate::handle::Borrowed<'a> { fn from(h: &'a $owned ) -> Self { unsafe { core::mem::transmute(h.0) } } }
            impl<'a> From<&'a $owned    > for crate::handle::Psuedo<'a>   { fn from(h: &'a $owned ) -> Self { unsafe { core::mem::transmute(h.0) } } }
        $($(
            impl<'a> AsRef<crate::handle::Borrowed<'a>> for $borrowed<'a> { fn as_ref(&self) -> &crate::handle::Borrowed<'a> { unsafe { core::mem::transmute(self) } } }
            impl<'a> AsRef<crate::handle::Psuedo<'a>  > for $borrowed<'a> { fn as_ref(&self) -> &crate::handle::Psuedo<'a>   { unsafe { core::mem::transmute(self) } } }

            impl<'a> From<$borrowed<'a> > for crate::handle::Borrowed<'a> { fn from(h: $borrowed<'a>) -> Self { unsafe { core::mem::transmute(h) } } }
            impl<'a> From<$borrowed<'a> > for crate::handle::Psuedo<'a>   { fn from(h: $borrowed<'a>) -> Self { unsafe { core::mem::transmute(h) } } }
        $($(
            impl<'a> AsRef<crate::handle::Psuedo<'a>  > for $psuedo<'a>   { fn as_ref(&self) -> &crate::handle::Psuedo<'a> { unsafe { core::mem::transmute(self) } } }

            impl<'a> From<$psuedo<'a>   > for crate::handle::Psuedo<'a>   { fn from(h: $psuedo<'a>  ) -> Self { unsafe { core::mem::transmute(h) } } }
        )?)?
        )?)?
    };

    (impl {AsRef, From} for $mod:ident :: { $owned:ident $(,$( $borrowed:ident $(,$( $psuedo:ident )?)? )?)? } ) => {
            impl     AsRef<$owned       > for     $owned    { fn as_ref(&self) -> &$owned         { self } }
        $($(
            impl<'a> AsRef<$borrowed<'a>> for &'a $owned    { fn as_ref(&self) -> &$borrowed<'a> { unsafe { core::mem::transmute(*self) } } }
            impl<'a> AsRef<$borrowed<'a>> for $borrowed<'a> { fn as_ref(&self) -> &$borrowed<'a> { self } }

            impl<'a> From<&'a $owned    > for $borrowed<'a> { fn from(h: &'a $owned ) -> Self { Self(h.0.cast(), PhantomData) } }
        $($(
            impl<'a> AsRef<$psuedo<'a>  > for &'a $owned    { fn as_ref(&self) -> &$psuedo<'a> { unsafe { core::mem::transmute(*self) } } }
            impl<'a> AsRef<$psuedo<'a>  > for $borrowed<'a> { fn as_ref(&self) -> &$psuedo<'a> { unsafe { core::mem::transmute(self) } } }
            impl<'a> AsRef<$psuedo<'a>  > for $psuedo<'a>   { fn as_ref(&self) -> &$psuedo<'a> { self } }

            impl<'a> From<&'a $owned    > for $psuedo<'a>   { fn from(h: &'a $owned ) -> Self { Self(h.0.cast(), PhantomData) } }
            impl<'a> From<$borrowed<'a> > for $psuedo<'a>   { fn from(h: $borrowed<'a>) -> Self { Self(h.0.cast(), PhantomData) } }
        )?)?
        )?)?
    };
}
