use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BitFlags {
    bits: u16,
}

impl BitFlags {
    pub const fn from_bits(bits: u16) -> Self {
        Self { bits }
    }
}

impl From<u16> for BitFlags {
    fn from(bits: u16) -> Self {
        Self::from_bits(bits)
    }
}

impl BitFlags {
    pub fn contains(self, other: Self) -> bool {
        self.bits & other.bits == other.bits
    }

    pub fn union(self, other: Self) -> Self {
        Self::from_bits(self.bits | other.bits)
    }

    pub fn intersection(self, other: Self) -> Self {
        Self::from_bits(self.bits & other.bits)
    }
}

impl BitOr for BitFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

impl BitAnd for BitFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        self.intersection(rhs)
    }
}

macro_rules! define_flags {
    ($flag_type: ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $flag_type {
            flags: BitFlags,
        }

        impl $flag_type {
            pub const fn from_bits(bits: u16) -> Self {
                Self {
                    flags: BitFlags::from_bits(bits),
                }
            }

            pub fn contains(&self, other: Self) -> bool {
                self.flags.contains(other.flags)
            }

            pub fn union(&self, other: Self) -> Self {
                Self::from_bits(self.flags.bits | other.flags.bits)
            }

            pub fn intersection(&self, other: Self) -> Self {
                Self::from_bits(self.flags.bits & other.flags.bits)
            }
        }

        impl From<u16> for $flag_type {
            fn from(bits: u16) -> Self {
                Self::from_bits(bits)
            }
        }

        impl BitOr for $flag_type {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                self.union(rhs)
            }
        }

        impl BitAnd for $flag_type {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                self.intersection(rhs)
            }
        }
    };
}

define_flags!(ClassAccessFlags);

impl ClassAccessFlags {
    pub const EMPTY: Self = Self::from_bits(0);
    pub const PUBLIC: Self = Self::from_bits(0x0001);
    pub const FINAL: Self = Self::from_bits(0x0010);
    pub const SUPER: Self = Self::from_bits(0x0020);
    pub const INTERFACE: Self = Self::from_bits(0x0200);
    pub const ABSTRACT: Self = Self::from_bits(0x0400);
    pub const SYNTHETIC: Self = Self::from_bits(0x1000);
    pub const ANNOTATION: Self = Self::from_bits(0x2000);
    pub const ENUM: Self = Self::from_bits(0x4000);
    pub const MODULE: Self = Self::from_bits(0x8000);
}

define_flags!(FieldAccessFlags);

impl FieldAccessFlags {
    pub const EMPTY: Self = Self::from_bits(0);
    pub const PUBLIC: Self = Self::from_bits(0x0001);
    pub const PRIVATE: Self = Self::from_bits(0x0002);
    pub const PROTECTED: Self = Self::from_bits(0x0004);
    pub const STATIC: Self = Self::from_bits(0x0008);
    pub const FINAL: Self = Self::from_bits(0x0010);
    pub const VOLATILE: Self = Self::from_bits(0x0040);
    pub const TRANSIENT: Self = Self::from_bits(0x0080);
    pub const SYNTHETIC: Self = Self::from_bits(0x1000);
    pub const ENUM: Self = Self::from_bits(0x4000);
}

define_flags!(MethodAccessFlags);

impl MethodAccessFlags {
    pub const EMPTY: Self = Self::from_bits(0);
    pub const PUBLIC: Self = Self::from_bits(0x0001);
    pub const PRIVATE: Self = Self::from_bits(0x0002);
    pub const PROTECTED: Self = Self::from_bits(0x0004);
    pub const STATIC: Self = Self::from_bits(0x0008);
    pub const FINAL: Self = Self::from_bits(0x0010);
    pub const SYNCHRONIZED: Self = Self::from_bits(0x0020);
    pub const BRIDGE: Self = Self::from_bits(0x0040);
    pub const VARARGS: Self = Self::from_bits(0x0080);
    pub const NATIVE: Self = Self::from_bits(0x0100);
    pub const ABSTRACT: Self = Self::from_bits(0x0400);
    pub const STRICT: Self = Self::from_bits(0x0800);
    pub const SYNTHETIC: Self = Self::from_bits(0x1000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u16() {
        let flags = BitFlags::from(0x0001);
        assert_eq!(0x0001, flags.bits);
    }

    #[test]
    fn test_from_bits() {
        let flags = BitFlags::from_bits(0x0001);
        assert_eq!(0x0001, flags.bits);
    }

    #[test]
    fn test_contains() {
        let flags = BitFlags::from_bits(0b_01);
        assert!(flags.contains(BitFlags::from_bits(0b_01)));
        assert!(!flags.contains(BitFlags::from_bits(0b_10)));
    }

    #[test]
    fn test_or() {
        assert_eq!(
            BitFlags::from_bits(0b_11),
            BitFlags::from_bits(0b01).union(BitFlags::from_bits(0b10))
        );
        assert_eq!(
            BitFlags::from_bits(0b_11),
            BitFlags::from_bits(0b01) | BitFlags::from_bits(0b10)
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(
            BitFlags::from_bits(0b01),
            BitFlags::from_bits(0b11).intersection(BitFlags::from_bits(0b01))
        );
        assert_eq!(
            BitFlags::from_bits(0b01),
            BitFlags::from_bits(0b11) & BitFlags::from_bits(0b01)
        );
    }
}
