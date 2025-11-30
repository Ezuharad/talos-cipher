// 2025 Steven Chiacchira
use crate::key;

/// Trait implementing bitwise operations
pub trait BitWise {
    /// The number of bits in the implementing type.
    fn n_bits() -> usize;
    /// Returns `true` if the bit at `bit_index` is set, and `false` otherwise.
    ///
    /// <div class="warning">
    /// Because this is a low-level operation, no checks are made to ensure `bit_index` is in range
    /// of the type. `false` will be returned if `bit_index` $\geq$ the implementing type's
    /// number of bits.
    /// </div>
    ///
    /// # Arguments
    /// * `bit_index` - the big-endian index of the bit to get the value of
    ///
    /// # Returns
    /// `true` if the bit at `bit_index` is set, and `false` otherwise.
    fn get_bit(&self, bit_index: usize) -> bool;
    /// Sets the bit at `bit_index` to `1` if `val`, and to `0` otherwise.
    ///
    /// <div class="warning">
    /// Because this is a low-level operation, no checks are made to ensure `bit_index` is in range
    /// of the type. Calling this method with an out of range `bit_index` is a no-op.
    /// `false` will be returned if `bit_index` $\geq$ the implementing type's number of bits.
    /// </div>
    ///
    /// # Arguments
    /// * `bit_index` - the big-endian index of the bit to set the value of
    /// * `val` - the value to set the specified bit to. `true` sets to `1`, and `false` sets to
    ///   `0`
    ///
    /// # Returns
    /// `true` if the bit was originally set, and `false` otherwise.
    fn set_bit(&mut self, bit_index: usize, val: bool) -> bool;
}

impl<T: key::Key> BitWise for T {
    fn n_bits() -> usize {
        T::zero().count_zeros() as usize
    }
    fn get_bit(&self, bit_index: usize) -> bool {
        let bit_mask = T::one() << bit_index;
        (*self & bit_mask) != T::zero()
    }
    fn set_bit(&mut self, bit_index: usize, new_val: bool) -> bool {
        let result = self.get_bit(bit_index);
        let bit_mask = T::one() << bit_index;
        if new_val {
            *self = *self | bit_mask;
        } else {
            *self = *self & !bit_mask;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::BitWise;

    #[test]
    fn test_n_bits() {
        assert_eq!(u8::n_bits(), u8::BITS as usize);
        assert_eq!(u16::n_bits(), u16::BITS as usize);
        assert_eq!(u32::n_bits(), u32::BITS as usize);
        assert_eq!(u64::n_bits(), u64::BITS as usize);
        assert_eq!(u128::n_bits(), u128::BITS as usize);
    }

    #[test]
    fn test_get_bit() {
        let bits = 0b01010101010101010101010101010101u32;
        let not_bits = !bits;

        for i in 0..u32::BITS {
            let is_even = i % 2 == 0;
            let idx = i as usize;
            if is_even {
                assert!(bits.get_bit(idx));
                assert!(!not_bits.get_bit(idx));
            } else {
                assert!(!bits.get_bit(idx));
                assert!(not_bits.get_bit(idx));
            }
        }
    }

    #[test]
    fn test_set_bit() {
        let mut bits = 0b00000000000000000000000000000000u32;

        for i in 0..u32::BITS {
            let idx = i as usize;
            bits.set_bit(idx, true);

            assert_eq!(bits, 1 << idx);

            bits.set_bit(idx, false);
            assert_eq!(bits, 0);
        }
    }
}
