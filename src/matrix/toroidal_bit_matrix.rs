// 2025 Steven Chiacchira
use crate::key;
use crate::matrix::{
    MatrixConstructError, MatrixOpError, ToroidalBinaryMatrix, ToroidalMatrixIndex,
};

#[derive(Debug, Clone)]
pub struct ToroidalBitMatrix<T: key::Key> {
    pub rows: usize,
    pub cols: usize,
    storage: Vec<T>,
}

impl<T: key::Key> ToroidalBinaryMatrix for ToroidalBitMatrix<T> {
    fn get_rows(&self) -> usize {
        self.rows
    }
    fn get_cols(&self) -> usize {
        self.cols
    }
    fn new(table: Vec<Vec<bool>>) -> Result<Self, MatrixConstructError> {
        let rows = table.len();
        if rows == 0 {
            return Err(MatrixConstructError::EmptyTable());
        }
        let cols = table[0].len();
        if cols == 0 {
            return Err(MatrixConstructError::EmptyTable());
        }

        // If the each row's is not equal to the first's, the table is invalid
        if table
            .iter()
            .map(|row| row.len() != cols)
            .fold(false, |a, b| a | b)
        {
            return Err(MatrixConstructError::RaggedTable());
        }

        let n_bits = rows * cols;
        let n_bits_per_entry = T::zero().count_zeros() as usize;
        let n_storage_entries = n_bits.div_ceil(n_bits_per_entry);
        let storage: Vec<T> = vec![T::zero(); n_storage_entries];

        let mut result = Self {
            rows,
            cols,
            storage,
        };

        for (row, col_slice) in table.into_iter().enumerate() {
            for (col, val) in col_slice.into_iter().enumerate() {
                let idx = (row as isize, col as isize);
                result.set(&idx, val);
            }
        }

        Ok(result)
    }

    fn at(&self, idx: &ToroidalMatrixIndex) -> bool {
        let (row, col) = self.canonize_index(*idx);
        let (element_idx, bit_idx) = self.get_element_bit_index_from_canon_index((row, col));
        let element = self.storage[element_idx];

        Self::get_bit_t(element, bit_idx)
    }
    fn set(&mut self, idx: &ToroidalMatrixIndex, new_val: bool) -> bool {
        let (row, col) = self.canonize_index(*idx);
        let (element_idx, bit_idx) = self.get_element_bit_index_from_canon_index((row, col));
        let element = &mut self.storage[element_idx];

        Self::set_bit_t(element, bit_idx, new_val)
    }
    fn bitwise_xor(&mut self, other: &Self) -> Result<(), MatrixOpError> {
        if self.get_cols() != other.get_cols() || self.get_rows() != other.get_rows() {
            return Err(MatrixOpError::DifferentShapes());
        }
        for (this_element, other_element) in self.storage.iter_mut().zip(other.get_storage()) {
            *this_element = *this_element ^ *other_element;
        }

        Ok(())
    }
    fn popcount(&self) -> u32 {
        self.storage.iter().map(|b| b.count_ones()).sum()
    }
}

impl<T: key::Key> ToroidalBitMatrix<T> {
    pub fn get_storage(&self) -> &Vec<T> {
        &self.storage
    }

    pub fn from_storage(
        rows: usize,
        cols: usize,
        storage: Vec<T>,
    ) -> Result<Self, MatrixConstructError> {
        if rows == 0 || cols == 0 {
            return Err(MatrixConstructError::EmptyTable());
        }
        let bits_per_t = T::zero().count_zeros() as usize;
        let n_bits = rows * cols;
        if n_bits.div_ceil(bits_per_t) != storage.len() {
            return Err(MatrixConstructError::InvalidStorage());
        }

        Ok(Self {
            rows,
            cols,
            storage,
        })
    }

    fn get_element_bit_index_from_canon_index(&self, index: (usize, usize)) -> (usize, usize) {
        let (bit_row, bit_col) = index;
        let flat_bit_idx = self.get_cols() * bit_row + bit_col;
        let bits_per_t = T::zero().count_zeros() as usize;

        let element_idx = flat_bit_idx / bits_per_t;
        let bit_idx = flat_bit_idx % bits_per_t;

        (element_idx, bit_idx)
    }

    fn get_bit_t(val: T, bit_index: usize) -> bool {
        let bit_mask = T::one() << bit_index;
        (val & bit_mask) != T::zero()
    }

    fn set_bit_t(data: &mut T, bit_index: usize, new_val: bool) -> bool {
        let result = Self::get_bit_t(*data, bit_index);
        let bit_mask = T::one() << bit_index;
        if new_val {
            *data = *data | bit_mask;
        } else {
            *data = *data & !bit_mask;
        }
        result
    }
}
