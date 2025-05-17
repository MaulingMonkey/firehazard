handles!(unsafe impl AsLocalHandleNN<c_void>        for pipe::sync::{OwnedDuplex, OwnedReader, OwnedWriter, BorrowedDuplex<'_>, BorrowedReader<'_>, BorrowedWriter<'_>});

impl FromLocalHandle<c_void> for OwnedDuplex {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { io::sync::type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for OwnedReader {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { io::sync::type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for OwnedWriter {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { io::sync::type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedDuplex<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) -> Self  { unsafe { io::sync::type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedReader<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) -> Self  { unsafe { io::sync::type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedWriter<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { io::sync::type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}



handles!(unsafe impl TryCloneToOwned<OwnedDuplex>   for pipe::sync::{OwnedDuplex, BorrowedDuplex<'_>});
handles!(unsafe impl TryCloneToOwned<OwnedReader>   for pipe::sync::{OwnedReader, BorrowedReader<'_>});
handles!(unsafe impl TryCloneToOwned<OwnedWriter>   for pipe::sync::{OwnedWriter, BorrowedWriter<'_>});

impl CloneToOwned for pipe::sync::OwnedDuplex {}
impl CloneToOwned for pipe::sync::OwnedReader {}
impl CloneToOwned for pipe::sync::OwnedWriter {}

impl CloneToOwned for pipe::sync::BorrowedDuplex<'_> {}
impl CloneToOwned for pipe::sync::BorrowedReader<'_> {}
impl CloneToOwned for pipe::sync::BorrowedWriter<'_> {}



handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => pipe::sync::OwnedReader         );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => pipe::sync::OwnedWriter         );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => pipe::sync::BorrowedDuplex<'_>  );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => pipe::sync::BorrowedReader<'_>  );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => pipe::sync::BorrowedWriter<'_>  );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => io::sync::OwnedDuplex           );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => io::sync::OwnedReader           );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => io::sync::OwnedWriter           );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => io::sync::BorrowedDuplex<'_>    );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => io::sync::BorrowedReader<'_>    );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => io::sync::BorrowedWriter<'_>    );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => handle::Owned                   );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => handle::Borrowed<'_>            );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => handle::Pseudo<'_>              );

handles!(unsafe impl @convert &'_ pipe::sync::OwnedReader       => pipe::sync::BorrowedReader<'_>  );
handles!(unsafe impl @convert     pipe::sync::OwnedReader       => io::sync::OwnedReader           );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedReader       => io::sync::BorrowedReader<'_>    );
handles!(unsafe impl @convert     pipe::sync::OwnedReader       => handle::Owned                   );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedReader       => handle::Borrowed<'_>            );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedReader       => handle::Pseudo<'_>              );

handles!(unsafe impl @convert &'_ pipe::sync::OwnedWriter       => pipe::sync::BorrowedWriter<'_>  );
handles!(unsafe impl @convert     pipe::sync::OwnedWriter       => io::sync::OwnedWriter           );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedWriter       => io::sync::BorrowedWriter<'_>    );
handles!(unsafe impl @convert     pipe::sync::OwnedWriter       => handle::Owned                   );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedWriter       => handle::Borrowed<'_>            );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedWriter       => handle::Pseudo<'_>              );

handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => pipe::sync::BorrowedReader<'_>  );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => pipe::sync::BorrowedWriter<'_>  );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => io::sync::BorrowedDuplex<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => io::sync::BorrowedReader<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => io::sync::BorrowedWriter<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => handle::Borrowed<'_>            );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => handle::Pseudo<'_>              );

handles!(unsafe impl @convert pipe::sync::BorrowedReader<'_>    => io::sync::BorrowedReader<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedReader<'_>    => handle::Borrowed<'_>            );
handles!(unsafe impl @convert pipe::sync::BorrowedReader<'_>    => handle::Pseudo<'_>              );

handles!(unsafe impl @convert pipe::sync::BorrowedWriter<'_>    => io::sync::BorrowedWriter<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedWriter<'_>    => handle::Borrowed<'_>            );
handles!(unsafe impl @convert pipe::sync::BorrowedWriter<'_>    => handle::Pseudo<'_>              );



impl crate::os::windows::io::FromRawHandle for pipe::sync::OwnedDuplex { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::FromRawHandle for pipe::sync::OwnedReader { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::FromRawHandle for pipe::sync::OwnedWriter { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }

impl crate::os::windows::io::IntoRawHandle for pipe::sync::OwnedDuplex { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for pipe::sync::OwnedReader { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for pipe::sync::OwnedWriter { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }



unsafe impl valrow::Borrowable for pipe::sync::OwnedDuplex  { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for pipe::sync::OwnedReader  { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for pipe::sync::OwnedWriter  { type Abi = HANDLENN; }
//safe impl valrow::Borrowable for pipe::sync::Borrowed*    { type Abi = HANDLENN; } // valid but pointless



#[cfg(test)] mod conversion {
    use crate::prelude::*;
    use crate::os::windows::io::FromRawHandle;

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_pipe_duplex() {
        let _null = unsafe { pipe::sync::OwnedDuplex::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_pipe_reader() {
        let _null = unsafe { pipe::sync::OwnedReader::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_pipe_writer() {
        let _null = unsafe { pipe::sync::OwnedWriter::from_raw_handle(null_mut()) };
    }
}
