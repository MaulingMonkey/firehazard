/// \[[microsoft.com](https://learn.microsoft.com/en-us/cpp/cpp/char-wchar-t-char16-t-char32-t?view=msvc-170)\]
/// wchar_t
///
#[allow(non_camel_case_types)]
pub type wchar_t = u16;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/cpp/cpp/char-wchar-t-char16-t-char32-t?view=msvc-170)\]
/// char
///
#[allow(non_camel_case_types)]
pub use core::ffi::c_char;

#[doc(alias = "TCHAR")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/tchar)\]
/// TCHAR
///
#[allow(non_camel_case_types)]
pub type tchar = wchar_t;



/// [u8] | [u16]
pub trait Unit
    : Clone + Copy + bytemuck::Pod
    + Default + bytemuck::Zeroable
    + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash
    + abistr::Unit
    + TryFrom<char>
    //where Self::CChar : Copy
{}

impl Unit for u8  {}
impl Unit for u16 {}



/// `␀`
const SYMBOL_FOR_NULL_CHAR : char = '\u{2400}';

#[test] fn symbol_for_null() {
    match '\u{2400}'.encode_utf16(&mut [0;2]) {
        [u] => assert_eq!(*u, SYMBOL_FOR_NULL_CHAR as u16),
        _ => panic!("expected `␀` to be encoded as a single `wchar_t`"),
    }
}
