#![cfg_attr(not(std), allow(dead_code))]

pub(crate) struct AssertSendable<T>(T);
unsafe impl<T> Send for AssertSendable<T> {}

impl<T> AssertSendable<T> {
    /// ### Safety
    /// `value` must be [`Send`]able even though `T` is not.
    pub const unsafe fn new(value: T) -> Self { Self(value) }

    pub fn into_inner(self) -> T { let Self(value) = self; value }
}
