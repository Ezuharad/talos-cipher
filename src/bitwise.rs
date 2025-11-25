// 2025 Steven Chiacchira
use crate::key;

pub trait BitWise {
    fn n_bits() -> usize;
    fn get_bit(&self, bit_index: usize) -> bool;
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
