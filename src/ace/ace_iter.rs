use crate::*;

use winapi::um::winnt::*;

use std::marker::PhantomData;
use std::mem::align_of;



#[derive(Clone)] pub struct Iter<'a> {
    next_ace:   *mut ace::Header,
    count:      u32,
    ph:         PhantomData<(&'a ACL, &'a [ace::Header])>,
}

impl<'a> Iter<'a> {
    pub fn new(ptr: acl::Ptr<'a>) -> Self {
        Self {
            next_ace:   unsafe { ptr.as_pacl().add(1).cast() },
            count:      ptr.get_acl_size_information().AceCount,
            ph:         PhantomData
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = ace::Ptr<'a>; // TODO: replace with ace::Ptr ?
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 { return None }
        let r = self.next_ace;
        let size = unsafe { (*r).size };
        self.count -= 1;
        self.next_ace = unsafe { (self.next_ace as *mut u8).add(size.into()).cast() };
        debug_assert!(self.next_ace as usize % align_of::<ace::Header>() == 0);
        Some(unsafe { ace::Ptr::from_raw_unchecked(r) })
    }
}
