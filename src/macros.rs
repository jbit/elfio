macro_rules! value_struct {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident($inner:ty) {
            $(
                $(#[$vattrs:meta])*
                $variant:ident = $value:literal
                $(=> $description:literal)?
            ),* $(,)?
        }
    ) => {
        $(#[$attrs])* // Include attributes
        #[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
        #[repr(transparent)]
        $vis struct $name($inner);
        #[allow(dead_code)]
        impl $name {
            // Define each enum value
            $(
                $(#[doc=$description])? // Use the optional description argument as doc string for variant
                $(#[$vattrs])* // Include attributes
                pub const $variant: $name = $name($value);
            )*
            /// Return the name of the value, or `None` if unknown
            pub fn name(self) -> Option<&'static str> {
                match self {
                    $(
                        Self::$variant => Some(stringify!($variant)),
                    )*
                    _ => None,
                }
            }
            /// Return description for the value, or `None` if no description
            pub fn description(self) -> Option<&'static str> {
                match self {
                    $($(
                        Self::$variant => Some($description),
                    )?)*
                    _ => None,
                }
            }
        }
        /// Convert from inner type
        impl Into<$inner> for $name {
            fn into(self) -> $inner {
                self.0
            }
        }
        /// Convert to inner type
        impl From<$inner> for $name {
            fn from(other: $inner) -> Self {
                Self(other)
            }
        }
    }
}

/// Macro for defining helpful enum-like new-type structs
macro_rules! enum_struct {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident($inner:ty) {
            $(
                $(#[$vattrs:meta])*
                $variant:ident = $value:literal
                $(=> $description:literal)?
            ),* $(,)?
        }
    ) => {
        // Implement values using values! macro
        value_struct!(
            $(#[$attrs])*
            $vis struct $name($inner) {
            $(
                $(#[$vattrs])*
                $variant = $value $(=> $description)?,
            )*
        });
        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                if let Some(name) = self.name() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "unknown({})", self.0)
                }
            }
        }
        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                if let Some(name) = self.description().or_else(|| self.name()) {
                    write!(f, "{}", name)
                } else {
                    write!(f, "unknown({})", self.0)
                }
            }
        }
    };
}

/// Macro for defining helpful flags new-type structs that can be combined and checked
macro_rules! flag_struct {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident($inner:ty) {
            $(
                $(#[$vattrs:meta])*
                $variant:ident = $value:literal
                $(=> $description:literal)?
            ),* $(,)?
        }
    ) => {
        // Implement values using values! macro
        value_struct!(
            $(#[$attrs])*
            $vis struct $name($inner) {
            $(
                $(#[$vattrs])*
                $variant = $value $(=> $description)?,
            )*
        });
        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0;
            }
        }
        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }
        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.0 &= other.0;
            }
        }
        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                use std::mem::size_of_val;
                use std::convert::TryInto;
                if self.0 == 0 {
                    // Special case empty flags
                    return write!(f, "{}", self.name().unwrap_or("none"));
                }

                let mut seperate = false;
                for bit in 0..size_of_val(&self.0) * 8 {
                    let value = Self((1<<bit).try_into().unwrap());
                    if *self & value == value {
                        if seperate {
                            write!(f, " | ")?;
                        }
                        if let Some(name) = value.name() {
                            write!(f, "{}", name)?;
                        } else {
                            write!(f, "bit{}", bit)?;
                        }
                        seperate = true;
                    }
                }
                Ok(())
            }
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn enum_struct() {
        enum_struct!(pub struct ABC(u8) {
            A   = 0,
            B   = 1,
            C   = 2,
        });
        assert_eq!(ABC::A, ABC::A);
        assert_eq!(ABC::B, ABC::B);
        assert_eq!(ABC::C, ABC::C);
        assert_ne!(ABC::A, ABC::B);
        assert_ne!(ABC::A, ABC::C);
        assert_eq!(ABC::from(9), ABC::from(9));
        assert_eq!(ABC::A, ABC::from(0));
        assert_eq!(ABC::B, ABC::from(1));
        assert_eq!(ABC::C, ABC::from(2));
        assert_ne!(ABC::A, ABC::from(9));
        assert_ne!(ABC::B, ABC::from(9));
        assert_ne!(ABC::C, ABC::from(9));
        assert_eq!(ABC::A.name(), Some("A"));
        assert_eq!(ABC::B.name(), Some("B"));
        assert_eq!(ABC::C.name(), Some("C"));
        assert_eq!(ABC::from(9).name(), None);
        assert_eq!(ABC::A.description(), None);
        assert_eq!(ABC::B.description(), None);
        assert_eq!(ABC::C.description(), None);
        assert_eq!(ABC::from(9).description(), None);
        assert_eq!(format!("{:?}", ABC::A), "A");
        assert_eq!(format!("{:?}", ABC::B), "B");
        assert_eq!(format!("{:?}", ABC::C), "C");
        assert_eq!(format!("{:?}", ABC::from(9)), "unknown(9)");
    }

    #[test]
    fn enum_struct_description() {
        enum_struct!(pub struct ABCdescription(u8) {
            A   = 0 => "First letter",
            B   = 1 => "Second letter",
            C   = 2,
        });
        assert_eq!(ABCdescription::A.name(), Some("A"));
        assert_eq!(ABCdescription::B.name(), Some("B"));
        assert_eq!(ABCdescription::C.name(), Some("C"));
        assert_eq!(ABCdescription::from(9).name(), None);
        assert_eq!(ABCdescription::A.description(), Some("First letter"));
        assert_eq!(ABCdescription::B.description(), Some("Second letter"));
        assert_eq!(ABCdescription::C.description(), None);
        assert_eq!(ABCdescription::from(9).description(), None);
        assert_eq!(format!("{:?}", ABCdescription::A), "A");
        assert_eq!(format!("{:?}", ABCdescription::B), "B");
        assert_eq!(format!("{:?}", ABCdescription::C), "C");
        assert_eq!(format!("{:?}", ABCdescription::from(9)), "unknown(9)");
    }

    #[test]
    fn flag_struct() {
        flag_struct!(pub struct ABC(u8) {
            EMPTY = 0,
            A     = 1,
            B     = 2,
            C     = 4,
        });
        assert_eq!(ABC::A, ABC::A);
        assert_eq!(ABC::B, ABC::B);
        assert_eq!(ABC::C, ABC::C);
        assert_ne!(ABC::A, ABC::B);
        assert_ne!(ABC::A, ABC::C);
        assert_eq!(ABC::from(9), ABC::from(9));
        assert_eq!(ABC::A, ABC::from(1));
        assert_eq!(ABC::B, ABC::from(2));
        assert_eq!(ABC::C, ABC::from(4));
        assert_ne!(ABC::A, ABC::from(9));
        assert_ne!(ABC::B, ABC::from(9));
        assert_ne!(ABC::C, ABC::from(9));
        assert_eq!(format!("{:?}", ABC::EMPTY), "EMPTY");
        assert_eq!(format!("{:?}", ABC::A), "A");
        assert_eq!(format!("{:?}", ABC::B), "B");
        assert_eq!(format!("{:?}", ABC::C), "C");
        assert_eq!(format!("{:?}", ABC::from(8)), "bit3");
        assert_eq!(format!("{:?}", ABC::from(7)), "A | B | C");
        assert_eq!(format!("{:?}", ABC::from(0xf)), "A | B | C | bit3");
    }
}
