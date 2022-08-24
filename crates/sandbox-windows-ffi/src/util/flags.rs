macro_rules! flags {
    (impl .. for $flags:ident ( $($inner:ty),+ $(,)? ) - $mask:ident { /* TODO: values */ } ) => {
        flags!(impl @basic for $flags ( $($inner),+ ) - $mask { /* TODO: values */ });
        flags!(impl @extra for $flags);
    };

    (impl @basic for $flags:ident ( $($inner:ty),+ $(,)? ) - $mask:ident { /* TODO: values */ } ) => {
        impl From<()                               > for $flags { fn from(_: ()                               ) -> Self { Self(0) } }
        impl From<Option<core::convert::Infallible>> for $flags { fn from(_: Option<core::convert::Infallible>) -> Self { Self(0) } }
    $(  impl From<$flags>                            for $inner { fn from(flags: $flags                       ) -> Self { flags.0 as _ } } )*

        impl core::ops::BitAnd              for $flags { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) } }
        impl core::ops::BitXor              for $flags { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) } }
        impl core::ops::BitOr               for $flags { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) } }
        impl core::ops::BitAndAssign        for $flags { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0 } }
        impl core::ops::BitXorAssign        for $flags { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0 } }
        impl core::ops::BitOrAssign         for $flags { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.0 } }

        impl core::ops::Not                 for $flags  { type Output = $mask;  fn not(self) -> Self::Output { $mask(!self.0) } }
        impl core::ops::BitAnd<$mask>       for $flags  { type Output = $flags; fn bitand(self, rhs: $mask ) -> $flags { $flags(self.0 & rhs.0) } }
        impl core::ops::BitAnd<$flags>      for $mask   { type Output = $flags; fn bitand(self, rhs: $flags) -> $flags { $flags(self.0 & rhs.0) } }
        impl core::ops::BitAndAssign<$mask> for $flags  { fn bitand_assign(&mut self, rhs: $mask) { self.0 &= rhs.0 } }
    };

    (impl @extra for AccessRights) => {
        impl From<access::Mask> for AccessRights { fn from(am: access::Mask) -> Self { Self(am.into()) } }
        impl From<AccessRights> for access::Mask { fn from(am: AccessRights) -> Self { unsafe { access::Mask::from_unchecked(am.as_u32()) } } }

        impl core::ops::BitAnd         <access::Mask> for AccessRights { type Output = Self; fn bitand(self, rhs: access::Mask) -> Self::Output { Self(self.as_u32() & rhs.as_u32()) } }
        impl core::ops::BitXor         <access::Mask> for AccessRights { type Output = Self; fn bitxor(self, rhs: access::Mask) -> Self::Output { Self(self.as_u32() ^ rhs.as_u32()) } }
        impl core::ops::BitOr          <access::Mask> for AccessRights { type Output = Self; fn bitor (self, rhs: access::Mask) -> Self::Output { Self(self.as_u32() | rhs.as_u32()) } }
        impl core::ops::BitAndAssign   <access::Mask> for AccessRights { fn bitand_assign(&mut self, rhs: access::Mask) { self.0 &= rhs.as_u32() } }
        impl core::ops::BitXorAssign   <access::Mask> for AccessRights { fn bitxor_assign(&mut self, rhs: access::Mask) { self.0 ^= rhs.as_u32() } }
        impl core::ops::BitOrAssign    <access::Mask> for AccessRights { fn bitor_assign (&mut self, rhs: access::Mask) { self.0 |= rhs.as_u32() } }

        impl core::ops::BitAnd<access::MaskMask>       for AccessRights { type Output = AccessRights; fn bitand(self, rhs: access::MaskMask) -> AccessRights { AccessRights(self.as_u32() & rhs.as_u32()) } }
        impl core::ops::BitAnd<AccessRights>       for access::MaskMask { type Output = AccessRights; fn bitand(self, rhs: AccessRights    ) -> AccessRights { AccessRights(self.as_u32() & rhs.as_u32()) } }
        impl core::ops::BitAndAssign<access::MaskMask> for AccessRights { fn bitand_assign(&mut self, rhs: access::MaskMask) { self.0 &= rhs.as_u32() } }
    };
    (impl @extra for $ty:ident) => {};

    ($value:expr, $fmt:expr, $final:literal, [$($flag:ident),+$(,)?]) => {{
        let mut value = $value;
        if value == 0 { return write!($fmt, "0") }

    $(  if value & $flag != 0 {
            write!($fmt, "{}", stringify!($flag))?;
            value &= !$flag;
            if value != 0 { write!($fmt, " | ")?; }
        })+

        if value != 0 { write!($fmt, $final, value)? }

        Ok(())
    }};
}
