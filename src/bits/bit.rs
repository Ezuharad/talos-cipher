// 2025 Steven Chiacchira

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
