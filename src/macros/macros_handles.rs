/// ### Usage
/// ```no_compile
/// handles!(unsafe impl *LocalHandle<HANDLE> for token::{Owned});
/// handles!(unsafe impl *LocalHandle<HANDLE> for token::{Owned, Borrowed<'_>});
/// handles!(unsafe impl *LocalHandle<HANDLE> for token::{Owned, Borrowed<'_>, Psuedo<'_>});
///
/// handles!(impl Debug for token::{Owned});
/// handles!(impl Debug for token::{Owned, Borrowed<'_>});
/// handles!(impl Debug for token::{Owned, Borrowed<'_>, Psuedo<'_>});
/// ```
macro_rules! handles {
    // impl Trait for mod::{Ty...}

    (  unsafe   impl Send                       for $mo:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl Send                    for $mo::$ty$(<$l>)?); )+};
    (  unsafe   impl Sync                       for $mo:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl Sync                    for $mo::$ty$(<$l>)?); )+};
    (  unsafe   impl FromLocalHandle<$raw:ty>   for $mo:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl FromLocalHandle<$raw>   for $mo::$ty$(<$l>)?); )+};
    ($(unsafe)? impl AsRef<Self>                for $mo:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(       impl AsRef<Self>             for $mo::$ty$(<$l>)?); )+};
    ($(unsafe)? impl AsLocalHandleNN<$raw:ty>   for $mo:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(       impl AsLocalHandleNN<$raw>   for $mo::$ty$(<$l>)?); )+};
    ($(unsafe)? impl Debug                      for $mo:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(       impl Debug                   for $mo::$ty$(<$l>)?); )+};

    // impl Trait for Ty

    (unsafe impl Send for $ty:ty) => { unsafe impl Send for $ty {} };
    (unsafe impl Sync for $ty:ty) => { unsafe impl Sync for $ty {} };
    (unsafe impl FromLocalHandle<$raw:ty> for $mo:ident::$ty:ident<'_>) => {
        impl FromLocalHandle<$raw> for $mo::$ty<'_> {
            unsafe fn from_raw_nn       (handle:  core::ptr::NonNull<$raw>) ->  Self { Self(handle, core::marker::PhantomData) }
            unsafe fn borrow_from_raw_nn(handle: &core::ptr::NonNull<$raw>) -> &Self { unsafe { core::mem::transmute(handle) } }
        }
    };
    (unsafe impl FromLocalHandle<$raw:ty> for $mo:ident::$ty:ident    ) => {
        impl FromLocalHandle<$raw> for $mo::$ty     {
            unsafe fn from_raw_nn       (handle:  core::ptr::NonNull<$raw>) ->  Self { Self(handle) }
            unsafe fn borrow_from_raw_nn(handle: &core::ptr::NonNull<$raw>) -> &Self { unsafe { core::mem::transmute(handle) } }
        }
    };
    ($(unsafe)? impl AsRef<Self> for $mo:ident::$ty:ident<'_>) => { impl<'a> AsRef<$mo::$ty<'a>> for $mo::$ty<'a> { fn as_ref(&self) -> &Self { self } } };
    ($(unsafe)? impl AsRef<Self> for $mo:ident::$ty:ident    ) => { impl     AsRef<$mo::$ty    > for $mo::$ty     { fn as_ref(&self) -> &Self { self } } };
    ($(unsafe)? impl AsLocalHandleNN<$raw:ty> for $ty:ty) => {
        impl AsLocalHandleNN<$raw> for $ty {
            fn as_handle_nn(&self) -> core::ptr::NonNull<$raw> { self.0 }
        }
        impl crate::os::windows::io::AsRawHandle for $ty {
            fn as_raw_handle(&self) -> crate::os::windows::io::RawHandle { self.0.as_ptr().cast() }
        }
        impl crate::os::windows::io::AsHandle for $ty {
            fn as_handle(&self) -> crate::os::windows::io::BorrowedHandle { unsafe { crate::os::windows::io::BorrowedHandle::borrow_raw(self.0.as_ptr().cast()) } }
        }
    };
    ($(unsafe)? impl Debug for $mo:ident::$ty:ident$(<$l:lifetime>)?) => {
        impl core::fmt::Debug for $mo::$ty$(<$l>)? {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                crate::handle::debug(fmt, stringify!($mo), stringify!($ty), self.0)
            }
        }
    };

    // "Legacy"

    (unsafe impl {AsRef<@base>, From<@base>} for $mod:ident :: { $owned:ident $(,$( $borrowed:ident<'_> $(,$( $psuedo:ident<'_> )?)? )?)? } ) => {
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

    (unsafe impl {AsRef, From} for $mod:ident :: { $owned:ident $(,$( $borrowed:ident<'_> $(,$( $psuedo:ident<'_> )?)? )?)? } ) => {
        $($(
            handles!(unsafe impl @convert &'_ $mod::$owned => $mod::$borrowed<'_>);
        $($(
            handles!(unsafe impl @convert &'_ $mod::$owned    => $mod::$psuedo<'_>);
            handles!(unsafe impl @convert $mod::$borrowed<'_> => $mod::$psuedo<'_>);
        )?)?
        )?)?
    };

    // Multi-impls

    (unsafe impl *LocalHandleNN<$raw:ty> for $($tt:tt)* ) => {
        handles!(unsafe impl FromLocalHandle<$raw> for $($tt)*);
        handles!(unsafe impl AsLocalHandleNN<$raw> for $($tt)*);
    };

    (unsafe impl {Send, Sync} for $($tt:tt)* ) => {
        handles!(unsafe impl Send for $($tt)*);
        handles!(unsafe impl Sync for $($tt)*);
    };

    // unsafe impl @convert Src => Dest

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
