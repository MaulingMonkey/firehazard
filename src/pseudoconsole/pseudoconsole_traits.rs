/// `COORD` | `[i16; 2]` | `(i16, i16)`
pub trait IntoSize : sealed::IntoSize {}

impl IntoSize for winapi::um::wincontypes::COORD    {}
impl IntoSize for [i16; 2]                          {}
impl IntoSize for (i16, i16)                        {}

pub(crate) mod sealed {
    #![allow(non_snake_case)] // X, Y
    use winapi::um::wincontypes::COORD;
    pub trait IntoSize              { fn into(self) -> COORD; }
    impl IntoSize for COORD         { fn into(self) -> COORD { self } }
    impl IntoSize for [i16; 2]      { fn into(self) -> COORD { let [X, Y] = self; COORD { X, Y } } }
    impl IntoSize for (i16, i16)    { fn into(self) -> COORD { let (X, Y) = self; COORD { X, Y } } }
}
