use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlags {
    bits: u16,
}

impl AccessFlags {
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
        Self { bits }
    }
}

impl From<u16> for AccessFlags {
    fn from(bits: u16) -> Self {
        Self::from_bits(bits)
    }
}

impl AccessFlags {
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
        let flags = AccessFlags::from(0x0001);
        assert_eq!(0x0001, flags.bits);
    }

    #[test]
    fn test_from_bits() {
        let flags = AccessFlags::from_bits(0x0001);
        assert_eq!(0x0001, flags.bits);
    }

    #[test]
    fn test_contains() {
        let flags = AccessFlags::from_bits(0b_01);
        assert!(flags.contains(AccessFlags::from_bits(0b_01)));
        assert!(!flags.contains(AccessFlags::from_bits(0b_10)));
    }

    #[test]
    fn test_or() {
        assert_eq!(
            AccessFlags::from_bits(0b_11),
            AccessFlags::from_bits(0b01).union(AccessFlags::from_bits(0b10))
        );
        assert_eq!(
            AccessFlags::from_bits(0b_11),
            AccessFlags::from_bits(0b01) | AccessFlags::from_bits(0b10)
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(
            AccessFlags::from_bits(0b01),
            AccessFlags::from_bits(0b11).intersection(AccessFlags::from_bits(0b01))
        );
        assert_eq!(
            AccessFlags::from_bits(0b01),
            AccessFlags::from_bits(0b11) & AccessFlags::from_bits(0b01)
        );
    }
}
