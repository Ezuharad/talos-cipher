// 2025 Steven Chiacchira
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

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
}

impl From<bool> for Bit {
    fn from(is_set: bool) -> Self {
        Self(is_set)
    }
}

impl BitAnd for Bit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bit(self.0 && rhs.0)
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bit(self.0 || rhs.0)
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bit(self.0 != rhs.0)
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
