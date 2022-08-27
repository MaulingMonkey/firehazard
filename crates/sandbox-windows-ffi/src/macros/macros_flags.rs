macro_rules! flags {
    (impl .. for $flags:ident ( $($inner:ty),+ $(,)? ) - $mask:ident) => {
        flags!(impl *LocalHandle<HANDLE> for $flags ( $($inner),+ ) - $mask);
        flags!(impl @extra for $flags);
    };

    (impl *LocalHandle<HANDLE> for $flags:ident ( $($inner:ty),+ $(,)? ) - $mask:ident) => {
        impl From<()                               > for $flags { fn from(_: ()                               ) -> Self { Self(0) } }
        impl From<()                               > for $mask  { fn from(_: ()                               ) -> Self { Self(0) } }
        impl From<Option<core::convert::Infallible>> for $flags { fn from(_: Option<core::convert::Infallible>) -> Self { Self(0) } }
        impl From<Option<core::convert::Infallible>> for $mask  { fn from(_: Option<core::convert::Infallible>) -> Self { Self(0) } }
    $(  impl From<$flags>                            for $inner { fn from(flags: $flags                       ) -> Self { flags.0 as _ } } )*
    $(  impl From<$mask >                            for $inner { fn from(mask:  $mask                        ) -> Self { mask .0 as _ } } )*

        impl core::ops::BitAnd              for $flags { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) } }
        impl core::ops::BitOr               for $flags { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) } }
        impl core::ops::BitAndAssign        for $flags { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0 } }
        impl core::ops::BitOrAssign         for $flags { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.0 } }

        impl core::ops::Not                 for $flags  { type Output = $mask;  fn not(self) -> Self::Output { $mask(!self.0) } }
        impl core::ops::BitAnd<$mask>       for $flags  { type Output = $flags; fn bitand(self, rhs: $mask ) -> $flags { $flags(self.0 & rhs.0) } }
        impl core::ops::BitAnd<$flags>      for $mask   { type Output = $flags; fn bitand(self, rhs: $flags) -> $flags { $flags(self.0 & rhs.0) } }
        impl core::ops::BitAndAssign<$mask> for $flags  { fn bitand_assign(&mut self, rhs: $mask) { self.0 &= rhs.0 } }

        impl core::ops::BitOr<()> for $flags                                { type Output = $flags; fn bitor (self, rhs: ()                                 ) -> Self::Output { $flags::from(self) | $flags::from(rhs) } }
        impl core::ops::BitOr<$flags> for ()                                { type Output = $flags; fn bitor (self, rhs: $flags                             ) -> Self::Output { $flags::from(self) | $flags::from(rhs) } }
        impl core::ops::BitOr<Option<core::convert::Infallible>> for $flags { type Output = $flags; fn bitor (self, rhs: Option<core::convert::Infallible>  ) -> Self::Output { $flags::from(self) | $flags::from(rhs) } }
        impl core::ops::BitOr<$flags> for Option<core::convert::Infallible> { type Output = $flags; fn bitor (self, rhs: $flags                             ) -> Self::Output { $flags::from(self) | $flags::from(rhs) } }

        impl core::ops::Mul<bool> for $flags { type Output = $flags; fn mul(self, rhs: bool  ) -> Self::Output { if rhs  { self } else { $flags(0) } } }
        impl core::ops::Mul<$flags> for bool { type Output = $flags; fn mul(self, rhs: $flags) -> Self::Output { if self { rhs  } else { $flags(0) } } }
    };

    (impl @extra for AccessRights) => {
        impl From<access::Mask> for AccessRights { fn from(am: access::Mask) -> Self { Self(am.into()) } }
        impl From<AccessRights> for access::Mask { fn from(am: AccessRights) -> Self { unsafe { access::Mask::from_unchecked(am.into()) } } }

        impl core::ops::BitAnd         <access::Mask> for AccessRights { type Output = Self; fn bitand(self, rhs: access::Mask) -> Self::Output { Self(u32::from(self) & u32::from(rhs)) } }
        impl core::ops::BitOr          <access::Mask> for AccessRights { type Output = Self; fn bitor (self, rhs: access::Mask) -> Self::Output { Self(u32::from(self) | u32::from(rhs)) } }
        impl core::ops::BitAndAssign   <access::Mask> for AccessRights { fn bitand_assign(&mut self, rhs: access::Mask) { self.0 &= u32::from(rhs) } }
        impl core::ops::BitOrAssign    <access::Mask> for AccessRights { fn bitor_assign (&mut self, rhs: access::Mask) { self.0 |= u32::from(rhs) } }

        impl core::ops::BitAnd<access::MaskMask>       for AccessRights { type Output = AccessRights; fn bitand(self, rhs: access::MaskMask) -> AccessRights { AccessRights(u32::from(self) & u32::from(rhs)) } }
        impl core::ops::BitAnd<AccessRights>       for access::MaskMask { type Output = AccessRights; fn bitand(self, rhs: AccessRights    ) -> AccessRights { AccessRights(u32::from(self) & u32::from(rhs)) } }
        impl core::ops::BitAndAssign<access::MaskMask> for AccessRights { fn bitand_assign(&mut self, rhs: access::MaskMask) { self.0 &= u32::from(rhs) } }
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
