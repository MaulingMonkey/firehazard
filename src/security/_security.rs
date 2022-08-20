//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_descriptor)\]
//! [`Attributes`], [`Descriptor`], [`DescriptorBuilder`], etc.

mod security_attributes;            pub use security_attributes::*;
mod security_descriptor;            pub use security_descriptor::*;
mod security_descriptor_builder;    pub use security_descriptor_builder::*;
mod security_impersonation_level;   pub use security_impersonation_level::*;
