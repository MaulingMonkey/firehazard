/// ### Usage
/// ```no_compile
/// handles!(unsafe impl *LocalHandle<HANDLE> for token::{Owned});
/// handles!(unsafe impl *LocalHandle<HANDLE> for token::{Owned, Borrowed<'_>});
/// handles!(unsafe impl *LocalHandle<HANDLE> for token::{Owned, Borrowed<'_>, Pseudo<'_>});
///
/// handles!(impl Debug for token::{Owned});
/// handles!(impl Debug for token::{Owned, Borrowed<'_>});
/// handles!(impl Debug for token::{Owned, Borrowed<'_>, Pseudo<'_>});
/// ```
macro_rules! handles {
    // impl Trait for mod::{Ty...}

    (  unsafe   impl Send                       for {$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl Send                    for $ty$(<$l>)?); )+};
    (  unsafe   impl Sync                       for {$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl Sync                    for $ty$(<$l>)?); )+};
    (  unsafe   impl TryCloneToOwned<$owned:ty> for {$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl TryCloneToOwned<$owned> for $ty$(<$l>)?); )+};
    (  unsafe   impl FromLocalHandle<$raw:ty>   for {$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl FromLocalHandle<$raw>   for $ty$(<$l>)?); )+};
    ($(unsafe)? impl AsLocalHandleNN<$raw:ty>   for {$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(       impl AsLocalHandleNN<$raw>   for $ty$(<$l>)?); )+};
    ($(unsafe)? impl Debug                      for {$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(       impl Debug                   for $ty$(<$l>)?); )+};

    (  unsafe   impl Send                       for $mo1:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl Send                    for $mo1::$ty$(<$l>)?); )+};
    (  unsafe   impl Sync                       for $mo1:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl Sync                    for $mo1::$ty$(<$l>)?); )+};
    (  unsafe   impl TryCloneToOwned<$owned:ty> for $mo1:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl TryCloneToOwned<$owned> for $mo1::$ty$(<$l>)?); )+};
    (  unsafe   impl FromLocalHandle<$raw:ty>   for $mo1:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl FromLocalHandle<$raw>   for $mo1::$ty$(<$l>)?); )+};
    ($(unsafe)? impl AsLocalHandleNN<$raw:ty>   for $mo1:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(       impl AsLocalHandleNN<$raw>   for $mo1::$ty$(<$l>)?); )+};
    ($(unsafe)? impl Debug                      for $mo1:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(       impl Debug                   for $mo1::$ty$(<$l>)?); )+};

    (  unsafe   impl Send                       for $mo1:ident::$mo2:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl Send                    for $mo1::$mo2::$ty$(<$l>)?); )+};
    (  unsafe   impl Sync                       for $mo1:ident::$mo2:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl Sync                    for $mo1::$mo2::$ty$(<$l>)?); )+};
    (  unsafe   impl TryCloneToOwned<$owned:ty> for $mo1:ident::$mo2:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl TryCloneToOwned<$owned> for $mo1::$mo2::$ty$(<$l>)?); )+};
    (  unsafe   impl FromLocalHandle<$raw:ty>   for $mo1:ident::$mo2:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(unsafe impl FromLocalHandle<$raw>   for $mo1::$mo2::$ty$(<$l>)?); )+};
    ($(unsafe)? impl AsLocalHandleNN<$raw:ty>   for $mo1:ident::$mo2:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(       impl AsLocalHandleNN<$raw>   for $mo1::$mo2::$ty$(<$l>)?); )+};
    ($(unsafe)? impl Debug                      for $mo1:ident::$mo2:ident::{$($ty:ident$(<$l:lifetime>)?),+$(,)?} ) => {$( handles!(       impl Debug                   for $mo1::$mo2::$ty$(<$l>)?); )+};

    // impl Trait for Ty

    (unsafe impl Send for $ty:ty) => { unsafe impl Send for $ty {} };
    (unsafe impl Sync for $ty:ty) => { unsafe impl Sync for $ty {} };
    (unsafe impl TryCloneToOwned<$owned:ty> for $ty:ty) => {
        unsafe impl TryCloneToOwned for $ty {
            type Owned = $owned;
            fn try_clone_to_owned(&self) -> firehazard::Result<Self::Owned> { unsafe { Self::Owned::from_raw(
                duplicate_handle_local_raw(self.as_handle(), None, false)?
            )}}
        }
    };
    (unsafe impl FromLocalHandle<$raw:ty> for $ty:ty) => {
        const _ : () = { fn type_check(handle: $ty) { let _ : &core::ptr::NonNull<$raw> = &handle.0; } };
        impl FromLocalHandle<$raw> for $ty {
            unsafe fn from_raw_nn       (handle:  core::ptr::NonNull<$raw>) ->  Self { unsafe { core::mem::transmute(handle) } }
            unsafe fn borrow_from_raw_nn(handle: &core::ptr::NonNull<$raw>) -> &Self { unsafe { core::mem::transmute(handle) } }
        }
    };
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
    ($(unsafe)? impl Debug for $ty:ty) => {
        impl core::fmt::Debug for $ty {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                crate::handle::debug(fmt, stringify!($ty), self.0)
            }
        }
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
        impl     From<    $($src)::+> for     $($dst)::+ { fn from(h:     $($src)::+) -> Self { unsafe { Self::from_raw_nn(h.into_handle_nn().cast()) } } }
        impl<'a> From<&'a $($src)::+> for &'a $($dst)::+ { fn from(h: &'a $($src)::+) -> Self { unsafe { core::mem::transmute(h) } } }
    };

    (unsafe impl @convert &'_ $($src:ident)::+ => $($dst:ident)::+<'_>) => {
        impl<'a> From<&'a $($src)::+> for $($dst)::+<'a> { fn from(h: &'a $($src)::+) -> Self { unsafe { Self::from_raw_nn(h.as_handle_nn().cast()) } } }
    };

    (unsafe impl @convert $($src:ident)::+<'_> => $($dst:ident)::+<'_>) => {
        impl<'a> From<$($src)::+<'a>> for $($dst)::+<'a> { fn from(h: $($src)::+<'a>) -> Self { unsafe { Self::from_raw_nn(h.as_handle_nn().cast()) } } }
    };
}
