// 2025 Steven Chiacchira
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// Represents a single bit.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bit(bool);

impl Bit {
    /// Zero-valued bit.
    pub const ZERO: Self = Bit(false);
    /// One-valued bit.
    pub const ONE: Self = Bit(true);

    /// Returns `true` if the bit is set, and `false` otherwise.
    ///
    /// # Returns
    /// Returns `true` if the bit is set, and `false` otherwise.
    #[must_use]
    pub fn is_set(&self) -> bool {
        self.0
    }

    /// Sets this bit to a `1` state
    pub fn set(&mut self) {
        self.0 = true;
    }

    /// Sets this bit to a `0` state
    pub fn clear(&mut self) {
        self.0 = false;
    }
}

impl From<bool> for Bit {
    fn from(is_set: bool) -> Self {
        Self(is_set)
    }
}

impl BitAnd for Bit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 && rhs.0)
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 || rhs.0)
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 != rhs.0)
    }
}

impl Not for Bit {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAndAssign for Bit {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 && rhs.0;
    }
}

impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 || rhs.0;
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = self.0 != rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::bits::Bit;

    #[test]
    fn test_bit_equals() {
        assert!(Bit::ONE == Bit::ONE);
        assert!(Bit::ZERO == Bit::ZERO);

        assert!(Bit::ONE != Bit::ZERO);
        assert!(Bit::ZERO != Bit::ONE);
    }

    #[test]
    fn test_bit_not() {
        assert!(!Bit::ONE == Bit::ZERO);
        assert!(!Bit::ZERO == Bit::ONE);
    }

    #[test]
    fn test_bit_and() {
        assert!(Bit::ONE & Bit::ONE == Bit::ONE);
        assert!(Bit::ONE & Bit::ZERO == Bit::ZERO);
        assert!(Bit::ZERO & Bit::ONE == Bit::ZERO);
        assert!(Bit::ZERO & Bit::ZERO == Bit::ZERO);
    }

    #[test]
    fn test_bit_and_assign() {
        let mut bit = Bit::ONE;

        bit &= Bit::ONE;
        assert!(bit.is_set());

        bit.set();
        bit &= Bit::ZERO;
        assert!(!bit.is_set());

        bit.clear();
        bit &= Bit::ONE;
        assert!(!bit.is_set());

        bit.clear();
        bit &= Bit::ZERO;
        assert!(!bit.is_set());
    }

    #[test]
    fn test_bit_or() {
        assert!(Bit::ONE | Bit::ONE == Bit::ONE);
        assert!(Bit::ONE | Bit::ZERO == Bit::ONE);
        assert!(Bit::ZERO | Bit::ONE == Bit::ONE);
        assert!(Bit::ZERO | Bit::ZERO == Bit::ZERO);
    }

    #[test]
    fn test_bit_or_assign() {
        let mut bit = Bit::ONE;

        bit |= Bit::ONE;
        assert!(bit.is_set());

        bit.set();
        bit |= Bit::ZERO;
        assert!(bit.is_set());

        bit.clear();
        bit |= Bit::ONE;
        assert!(bit.is_set());

        bit.clear();
        bit |= Bit::ZERO;
        assert!(!bit.is_set());
    }

    #[test]
    fn test_bit_xor_assign() {
        let mut bit = Bit::ONE;

        bit ^= Bit::ONE;
        assert!(!bit.is_set());

        bit.set();
        bit ^= Bit::ZERO;
        assert!(bit.is_set());

        bit.clear();
        bit ^= Bit::ONE;
        assert!(bit.is_set());

        bit.clear();
        bit ^= Bit::ZERO;
        assert!(!bit.is_set());
    }

    #[test]
    fn test_bit_xor() {
        assert!(Bit::ONE ^ Bit::ONE == Bit::ZERO);
        assert!(Bit::ONE ^ Bit::ZERO == Bit::ONE);
        assert!(Bit::ZERO ^ Bit::ONE == Bit::ONE);
        assert!(Bit::ZERO ^ Bit::ZERO == Bit::ZERO);
    }
}
