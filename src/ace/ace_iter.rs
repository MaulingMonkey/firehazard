use crate::prelude::*;

use winapi::um::winnt::*;



/// [`Iterator`] over contiguous variable-length [`ace::Ref`]s
#[derive(Clone)] pub struct Iter<'a> {
    next_ace:   NonNull<ace::Header>,
    count:      u32,
    ph:         PhantomData<(&'a ACL, &'a [ace::Header])>,
}

impl<'a> Iter<'a> {
    pub const fn empty() -> Self { Self { next_ace: NonNull::dangling(), count: 0, ph: PhantomData } }
    pub fn new(acl: impl Into<Self>) -> Self { acl.into() }
    pub(crate) fn as_ptr(&self) -> *mut ace::Header { self.next_ace.as_ptr() }
}

impl Default for Iter<'_> { fn default() -> Self { Self::empty() } }

impl<'a> From<acl::Ref<'a>> for Iter<'a> {
    fn from(acl: acl::Ref<'a>) -> Self {
        Self {
            next_ace:   unsafe { acl.as_pacl().add(1).cast() },
            count:      acl.get_acl_size_information().AceCount,
            ph:         PhantomData
        }
    }
}

impl ExactSizeIterator for Iter<'_> {}

impl<'a> Iterator for Iter<'a> {
    type Item = ace::Ref<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 { return None }
        let r = self.next_ace;
        let size = unsafe { r.as_ref().size };
        self.count -= 1;
        self.next_ace = unsafe { self.next_ace.byte_add(size.into()) };
        debug_assert!(self.next_ace.is_aligned());
        Some(unsafe { ace::Ref::from_raw_unchecked(r) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = usize::from32(self.count);
        (n, Some(n))
    }
}
