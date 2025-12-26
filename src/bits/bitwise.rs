// 2025 Steven Chiacchira
use crate::bits::Bit;
use crate::key;

/// Trait implementing bitwise operations
pub trait BitWise {
    /// The number of bytes in the implementing type
    #[must_use]
    fn n_bytes() -> u32 {
        Self::n_bits() / u8::BITS
    }
    /// The number of bits in the implementing type.
    #[must_use]
    fn n_bits() -> u32;
    /// Returns the state of the bit at `bit_index`, or `None` if `bit_index` is greater than or
    /// equal to [`BitWise::n_bits`] for the implementing type.
    ///
    /// # Arguments
    /// * `bit_index` - the big-endian index of the bit to get the value of
    ///
    /// # Returns
    /// The state of the bit at `bit_index`, or `None` if `bit_index` is greater than or equal to
    /// [`BitWise::n_bits`] for the implementing type.
    #[must_use]
    fn get_bit(&self, bit_index: usize) -> Option<Bit>;
    /// Sets the bit at `bit_index` to that of `val`.
    ///
    /// If `bit_index` is greater than or equal to [`BitWise::n_bits`] for the implementing type,
    /// `None` is returned and this operation is a no-op.
    ///
    /// # Arguments
    /// * `bit_index` - the big-endian index of the bit to set the value of
    /// * `val` - the value to set the specified bit to
    ///
    /// # Returns
    /// The original state of the bit at `bit_index`, or `None` if `bit_index` is greater than or
    /// equal to [`BitWise::n_bits`] for the implementing type.
    #[must_use]
    fn set_bit(&mut self, bit_index: usize, val: Bit) -> Option<Bit>;
    /// Returns the state of the bit at `bit_index`.
    ///
    /// # Safety
    /// Because this is a low-level operation, no checks are made to ensure `bit_index` is in range
    /// of the type. Using a `bit_index` which is greater than or equal to [`BitWise::n_bits`] for the
    /// implementing type will cause a panic in debug mode, and undefined behavior in release mode.
    ///
    /// Prefer [`BitWise::set_bit`] for a safe alternative to this method.
    ///
    /// # Arguments
    /// * `bit_index` - the big-endian index of the bit to get the value of
    ///
    /// # Returns
    /// The state of the bit at `bit_index`. Results in undefined behavior if `bit_index` is greater
    /// than or equal to [`BitWise::n_bits`] for the implementing type.
    #[must_use]
    unsafe fn get_bit_unchecked(&self, bit_index: usize) -> Bit;
    /// Sets the bit at `bit_index` to that of `val`.
    ///
    /// # Safety
    /// Because this is a low-level operation, no checks are made to ensure `bit_index` is in range
    /// of the type. Using a `bit_index` which is greater than or equal to [`BitWise::n_bits`] for
    /// the implementing type will cause a panic in debug mode, and undefined behavior in release mode.
    ///
    /// Prefer [`BitWise::get_bit`] for a safe alternative to this method.
    ///
    /// # Arguments
    /// * `bit_index` - the big-endian index of the bit to set the value of
    /// * `val` - the value to set the specified bit to
    ///
    /// # Returns
    /// The original state of the bit at `bit_index`. Results in undefined behavior if `bit_index`
    /// is greater than or equal to [`BitWise::n_bits`] for the implementing type.
    unsafe fn set_bit_unchecked(&mut self, bit_index: usize, val: Bit) -> Bit;
}

impl<T: key::Key> BitWise for T {
    fn n_bits() -> u32 {
        T::zero().count_zeros()
    }
    fn get_bit(&self, bit_index: usize) -> Option<Bit> {
        if bit_index >= Self::n_bits() as usize {
            return None;
        }

        unsafe { Some(self.get_bit_unchecked(bit_index)) }
    }
    fn set_bit(&mut self, bit_index: usize, val: Bit) -> Option<Bit> {
        if bit_index >= Self::n_bits() as usize {
            return None;
        }

        unsafe { Some(self.set_bit_unchecked(bit_index, val)) }
    }
    unsafe fn get_bit_unchecked(&self, bit_index: usize) -> Bit {
        debug_assert!((bit_index as u32) < T::n_bits());
        let bit_mask = T::one() << bit_index;
        let is_set: bool = (*self & bit_mask) != T::zero();
        Bit::from(is_set)
    }
    unsafe fn set_bit_unchecked(&mut self, bit_index: usize, new_val: Bit) -> Bit {
        debug_assert!((bit_index as u32) < T::n_bits());
        let result = self.get_bit_unchecked(bit_index);
        let bit_mask = T::one() << bit_index;
        if new_val.is_set() {
            *self = *self | bit_mask;
        } else {
            *self = *self & !bit_mask;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::bits::{Bit, BitWise};

    #[test]
    fn test_n_bits() {
        assert_eq!(u8::n_bits(), u8::BITS);
        assert_eq!(u16::n_bits(), u16::BITS);
        assert_eq!(u32::n_bits(), u32::BITS);
        assert_eq!(u64::n_bits(), u64::BITS);
        assert_eq!(u128::n_bits(), u128::BITS);
    }

    #[test]
    fn test_get_bit() {
        let bits = 0b01010101010101010101010101010101u32;
        let not_bits = !bits;

        for i in 0..u32::BITS {
            let is_even = i % 2 == 0;
            let idx = i as usize;
            if is_even {
                unsafe {
                    assert!(bits.get_bit_unchecked(idx).is_set());
                    assert!(!not_bits.get_bit_unchecked(idx).is_set());
                }
            } else {
                unsafe {
                    assert!(!bits.get_bit_unchecked(idx).is_set());
                    assert!(not_bits.get_bit_unchecked(idx).is_set());
                }
            }
        }
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn test_get_bit_out_of_bounds() {
        let bits = 0b11111111111111111111111111111111u32;
        unsafe {
            let _ = bits.get_bit_unchecked(32);
        }
    }

    #[test]
    fn test_set_bit() {
        let mut bits = 0b00000000000000000000000000000000u32;

        for i in 0..u32::BITS {
            let idx = i as usize;
            unsafe {
                bits.set_bit_unchecked(idx, Bit::ONE);
                assert_eq!(bits, 1 << idx);

                bits.set_bit_unchecked(idx, Bit::ZERO);
                assert_eq!(bits, 0);
            }
        }
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn test_set_bit_out_of_bounds() {
        let mut bits = 0b00000000000000000000000000000000u32;
        unsafe {
            bits.set_bit_unchecked(32, Bit::ONE);
        }
    }
}
