use crate::prelude::*;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// _Owned_, _non-null_ `HANDLE`.
///
/// Will ≈[`close_handle`] on [`Drop`].
/// It's worth noting that some `HANDLE` types cannot be [`close_handle`]d but *are* compatible with other `HANDLE`-accepting functions.
/// These will generally have their own handle types that won't convert to [`handle::Owned`], but *may* convert to [`handle::Borrowed`].
///
/// [`CloseHandle`]:    https://learn.microsoft.com/en-us/wsindows/win32/api/handleapi/nf-handleapi-closehandle
///
#[repr(transparent)] pub struct Owned(HANDLENN);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// _Borrowed_, _non-null_ `HANDLE` to a kernel object.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Borrowed<'a>(HANDLENN, PhantomData<&'a HANDLENN>);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// _Borrowed_ or _[pseudo](handle::Pseudo)_, _non-null_ `HANDLE` to a kernel object.
///
/// Pseudo handles include, but may not be limited to:
///
/// | Function                          | Current <br> Value    |
/// | ----------------------------------|:---------------------:|
/// | GetCurrentProcess()               | -1 |
/// | GetCurrentThread()                | -2 |
/// | GetCurrentSession()               | -3 |
/// | GetCurrentProcessToken()          | -4 |
/// | GetCurrentThreadToken()           | -5 |
/// | GetCurrentThreadEffectiveToken()  | -6 |
///
/// Several of these change meaning if sent between threads.
/// As such, <code>[Pseudo] : \![Send] + \![Sync]</code>:
///
/// ```compile_fail
/// # use firehazard::*;
/// let h : handle::Pseudo = get_current_thread().into();
/// std::thread::spawn(move ||{
///     let _h = h; // error[E0277]: `NonNull<c_void>` cannot be shared between threads safely
/// }).join().unwrap();
/// ```
///
/// ```compile_fail
/// # use firehazard::*;
/// # let h : handle::Pseudo = get_current_thread().into();
/// std::thread::spawn(||{
///     let _h = &h; // error[E0277]: `NonNull<c_void>` cannot be shared between threads safely
/// }).join().unwrap();
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Pseudo<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(hidden)] #[deprecated = "it's spelled Pseudo"] pub use Pseudo as Psuedo;



handles!(unsafe impl *LocalHandleNN<c_void> for handle::{Owned, Borrowed<'_>, Pseudo<'_>});
handles!(unsafe impl TryCloneToOwned<Owned> for handle::{Owned, Borrowed<'_>, Pseudo<'_>});
handles!(       impl Debug                  for handle::{Owned, Borrowed<'_>, Pseudo<'_>});

handles!(unsafe impl @convert &'_ handle::Owned     => handle::Borrowed<'_> );
handles!(unsafe impl @convert &'_ handle::Owned     => handle::Pseudo<'_>   );
handles!(unsafe impl @convert handle::Borrowed<'_>  => handle::Pseudo<'_>   );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for Owned { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

unsafe impl valrow::Borrowable for Owned         { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Borrowed<'_>  { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Pseudo<'_>    { type Abi = HANDLENN; }

impl CloneToOwned for Owned         {}
impl CloneToOwned for Borrowed<'_>  {}
//pl CloneToOwned for Pseudo<'_>    {} // XXX: token pseudo-handles can be converted to generic pseudo-handles, which will fail to clone



#[cfg(test)] pub(crate) mod invalid {
    use crate::prelude::*;

    #[repr(transparent)] struct Invalid(HANDLE);
    #[repr(transparent)] struct InvalidNN(HANDLENN);

    impl AsLocalHandle   for Invalid                { fn as_handle(&self)       -> HANDLE   { self.0 } }
    impl AsLocalHandleNN for InvalidNN              { fn as_handle_nn(&self)    -> HANDLENN { self.0 } }
    impl From<InvalidNN> for handle::Owned          { fn from(_: InvalidNN) -> Self { unsafe { core::mem::transmute(winapi::um::handleapi::INVALID_HANDLE_VALUE) } } }
    impl From<InvalidNN> for handle::Borrowed<'_>   { fn from(_: InvalidNN) -> Self { unsafe { core::mem::transmute(winapi::um::handleapi::INVALID_HANDLE_VALUE) } } }
    impl From<InvalidNN> for handle::Pseudo<'_>     { fn from(_: InvalidNN) -> Self { unsafe { core::mem::transmute(winapi::um::handleapi::INVALID_HANDLE_VALUE) } } }

    pub(crate) fn null()                    -> impl AsLocalHandle                                                                                                           { Invalid(null_mut()) }
    pub(crate) fn invalid_value()           -> impl AsLocalHandle + AsLocalHandleNN + Into<handle::Owned> + Into<handle::Borrowed<'static>> + Into<handle::Pseudo<'static>> { InvalidNN(unsafe { NonNull::new_unchecked(winapi::um::handleapi::INVALID_HANDLE_VALUE.cast()) }) }
    pub(crate) fn never_valid()             -> impl AsLocalHandle + AsLocalHandleNN + Into<handle::Owned> + Into<handle::Borrowed<'static>> + Into<handle::Pseudo<'static>> { InvalidNN(unsafe { NonNull::new_unchecked(0x12345678_usize as *mut _) }) }
    #[cfg(std)] pub(crate) fn dangling()    -> impl AsLocalHandle + AsLocalHandleNN + Into<handle::Owned> + Into<handle::Borrowed<'static>> + Into<handle::Pseudo<'static>> {
        use std::os::windows::io::AsRawHandle;
        let file = std::fs::File::open("Readme.md").unwrap();
        let dangling = file.as_raw_handle().cast();
        drop(file);
        InvalidNN(NonNull::new(dangling).unwrap())
    }
}
