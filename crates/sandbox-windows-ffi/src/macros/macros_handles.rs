/// ### Usage
/// ```no_compile
/// handles!(unsafe impl *LocalHandle<HANDLE> for token::{Owned});
/// handles!(unsafe impl *LocalHandle<HANDLE> for token::{Owned, Borrowed});
/// handles!(unsafe impl *LocalHandle<HANDLE> for token::{Owned, Borrowed, Psuedo});
///
/// handles!(impl Debug for token::{Owned});
/// handles!(impl Debug for token::{Owned, Borrowed});
/// handles!(impl Debug for token::{Owned, Borrowed, Psuedo});
/// ```
macro_rules! handles {
    (unsafe impl *LocalHandleNN<$raw:ty> for $mod:ident :: { $owned:ident $(,$( $borrowed:ident $(,$( $psuedo:ident )?)? )?)? } ) => {
            impl FromLocalHandle<$raw> for $owned       { unsafe fn from_raw_nn(handle: core::ptr::NonNull<$raw>) -> Self { Self(handle) } }
        $($(
            impl FromLocalHandle<$raw> for $borrowed<'_>{ unsafe fn from_raw_nn(handle: core::ptr::NonNull<$raw>) -> Self { Self(handle, core::marker::PhantomData) } }
        $($(
            impl FromLocalHandle<$raw> for $psuedo<'_>  { unsafe fn from_raw_nn(handle: core::ptr::NonNull<$raw>) -> Self { Self(handle, core::marker::PhantomData) } }
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

    (unsafe impl {AsRef<@base>, From} for $mod:ident :: { $owned:ident $(,$( $borrowed:ident $(,$( $psuedo:ident )?)? )?)? } ) => {
            handles!(unsafe impl @convert     $mod::$owned      => handle::Owned        );
            handles!(unsafe impl @convert &'_ $mod::$owned      => handle::Borrowed<'_> );
            handles!(unsafe impl @convert &'_ $mod::$owned      => handle::Psuedo<'_>   );
        $($(
            handles!(unsafe impl @convert $mod::$borrowed<'_>   => handle::Borrowed<'_> );
            handles!(unsafe impl @convert $mod::$borrowed<'_>   => handle::Psuedo<'_>   );
        $($(
            handles!(unsafe impl @convert $mod::$psuedo<'_>     => handle::Psuedo<'_>   );
        )?)?
        )?)?
    };

    (unsafe impl {AsRef, From} for $mod:ident :: { $owned:ident $(,$( $borrowed:ident $(,$( $psuedo:ident )?)? )?)? } ) => {
            impl AsRef<$owned> for $owned { fn as_ref(&self) -> &$owned { self } }
        $($(
            impl<'a> AsRef<$borrowed<'a>> for $borrowed<'a> { fn as_ref(&self) -> &$borrowed<'a> { self } }
            handles!(unsafe impl @convert &'_ $mod::$owned => $mod::$borrowed<'_>);
        $($(
            impl<'a> AsRef<$psuedo<'a>> for $psuedo<'a> { fn as_ref(&self) -> &$psuedo<'a> { self } }
            handles!(unsafe impl @convert &'_ $mod::$owned    => $mod::$psuedo<'_>);
            handles!(unsafe impl @convert $mod::$borrowed<'_> => $mod::$psuedo<'_>);
        )?)?
        )?)?
    };

    (unsafe impl @convert $($src:ident)::+ => $($dst:ident)::+) => {
        impl AsRef<$($dst)::+> for $($src)::+ { fn as_ref(&self) -> &$($dst)::+ { unsafe { core::mem::transmute(self) } } }
        impl From<$($src)::+> for $($dst)::+ { fn from(h: $($src)::+) -> Self { unsafe { core::mem::transmute(h) } } }
        impl<'a> From<&'a $($src)::+> for &'a $($dst)::+ { fn from(h: &'a $($src)::+ ) -> Self { unsafe { core::mem::transmute(h) } } }
    };

    (unsafe impl @convert &'_ $($src:ident)::+ => $($dst:ident)::+<'_>) => {
        impl<'a> AsRef<$($dst)::+<'a>> for &'a $($src)::+ { fn as_ref(&self) -> &$($dst)::+<'a> { unsafe { core::mem::transmute(*self) } } }
        impl<'a> From<&'a $($src)::+> for $($dst)::+<'a> { fn from(h: &'a $($src)::+) -> Self { unsafe { Self::from_raw_nn(h.as_handle_nn().cast()) } } }
    };

    (unsafe impl @convert $($src:ident)::+<'_> => $($dst:ident)::+<'_>) => {
        impl<'a> AsRef<$($dst)::+<'a>> for $($src)::+<'a> { fn as_ref(&self) -> &$($dst)::+<'a> { unsafe { core::mem::transmute(self) } } }
        impl<'a> From<$($src)::+<'a>> for $($dst)::+<'a> { fn from(h: $($src)::+<'a>) -> Self { unsafe { Self::from_raw_nn(h.as_handle_nn()) } } }
    };
}
