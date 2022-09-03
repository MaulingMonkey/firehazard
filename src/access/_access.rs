//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-mask)\]
//! ACCESS_MASK generic types, functions, and constants
//!
//! ### References
//! *   [`ACCESS_MASK`](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-mask)
//! *   The Old New Thing / Raymond Chen
//!     *   [Is GENERIC_ALL equivalent to GENERIC_READ | GENERIC_WRITE | GENERIC_EXECUTE?](https://devblogs.microsoft.com/oldnewthing/20170310-00/?p=95705)
//!     *   [Anybody can make up a generic mapping](https://devblogs.microsoft.com/oldnewthing/?p=20733)
//!     *   [If you ask for STANDARD_RIGHTS_REQUIRED, you may as well ask for the moon](https://devblogs.microsoft.com/oldnewthing/20080227-00/?p=23303)

pub(crate) mod constants;       pub use constants::*;
mod mask;                       pub use mask::*;
