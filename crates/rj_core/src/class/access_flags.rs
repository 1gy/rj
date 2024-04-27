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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AccessFlags {
    flags: BitFlags,
}

impl AccessFlags {
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

    pub const fn from_bits(bits: u16) -> Self {
        Self {
            flags: BitFlags::from_bits(bits),
        }
    }
}

impl From<u16> for AccessFlags {
    fn from(bits: u16) -> Self {
        Self::from_bits(bits)
    }
}

impl AccessFlags {
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

impl BitOr for AccessFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

impl BitAnd for AccessFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        self.intersection(rhs)
    }
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

    #[test]
    fn test_access_flags() {
        let flags =
            AccessFlags::from_bits(AccessFlags::PUBLIC.flags.bits | AccessFlags::FINAL.flags.bits);
        assert!(flags.contains(AccessFlags::PUBLIC));
        assert!(flags.contains(AccessFlags::FINAL));
        assert!(!flags.contains(AccessFlags::SUPER));

        assert_eq!(AccessFlags::PUBLIC, flags & AccessFlags::PUBLIC);
        assert_eq!(AccessFlags::EMPTY, flags & AccessFlags::SUPER);
        assert_eq!(AccessFlags::PUBLIC, flags.intersection(AccessFlags::PUBLIC));
        assert_eq!(AccessFlags::EMPTY, flags.intersection(AccessFlags::SUPER));

        assert_eq!(flags, AccessFlags::PUBLIC | AccessFlags::FINAL);
        assert_eq!(flags, AccessFlags::PUBLIC.union(AccessFlags::FINAL));
    }
}
