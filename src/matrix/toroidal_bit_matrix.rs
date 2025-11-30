// 2025 Steven Chiacchira
use crate::bitwise::BitWise;
use crate::key;
use crate::matrix::{
    MatrixConstructError, MatrixOpError, ToroidalBinaryMatrix, ToroidalMatrixIndex,
};

#[derive(Debug, Clone)]
/// Struct implementing [`ToroidalBinaryMatrix`] backed by a `Vec<T>`. `T` must be an unsigned
/// integer primitive such as `u8` or `u64`.
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

        if table.iter().any(|row| row.is_empty()) {
            return Err(MatrixConstructError::EmptyTable());
        }

        let cols = table[0].len();
        if table.iter().any(|row| row.len() != cols) {
            return Err(MatrixConstructError::RaggedTable());
        }

        let n_bits = rows * cols;
        let n_bits_per_entry = T::n_bits();
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

        element.get_bit(bit_idx)
    }
    fn set(&mut self, idx: &ToroidalMatrixIndex, new_val: bool) -> bool {
        let (row, col) = self.canonize_index(*idx);
        let (element_idx, bit_idx) = self.get_element_bit_index_from_canon_index((row, col));
        let element = &mut self.storage[element_idx];

        element.set_bit(bit_idx, new_val)
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
    /// Returns the storage backing the Matrix.
    ///
    /// # Returns
    /// The storage backing the Matrix.
    pub fn get_storage(&self) -> &Vec<T> {
        &self.storage
    }
    /// Constructs a new [`ToroidalBitMatrix`] from stroage, as well as the count of rows and
    /// columns.
    ///
    /// The following criteria must be met for Matrix construction:
    /// * $rows > 0 \land cols > 0$
    /// * $rows * cols \geq$ storage.size() * `T::BITS` $land rows * cols \lt$ (storage.size() + 1) *
    ///   `T::BITS`
    ///
    /// Where T::BITS is the number of bits in the unsigned integer type `T`.
    ///
    /// See [`MatrixConstructError`] for possible error variants resulting from violating thee
    /// criteria.
    ///
    /// # Arguments
    /// * `rows` - the number of rows the Matrix will have
    /// * `cols` - the number of columns the Matrix will have
    /// * `storage` the storage backing the Matrix. Note that elements are stored in row-major
    ///
    /// # Returns
    /// A newly constructed Matrix if the storage, rows, and columns are valid, and a
    /// [`MatrixConstructError`] otherwise.
    pub fn from_storage(
        rows: usize,
        cols: usize,
        storage: Vec<T>,
    ) -> Result<Self, MatrixConstructError> {
        if rows == 0 || cols == 0 {
            return Err(MatrixConstructError::EmptyTable());
        }
        let bits_per_t = T::n_bits();
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

    /// Returns the bit index of the element at canonized index `index`.
    ///
    /// Once an index has been canonized, it is necessary to find the element which contains the
    /// specified bit, as well as the bit of the element should be accessed. This function finds
    /// these element and bit indices from the canon index.
    ///
    /// Note that the element index will always be less than the size of the Matrix's backing
    /// storage, and the bit index will always be less than the number of bits in type `T`.
    ///
    /// # Arguments
    /// * `index` - the canon index to find the element and bit indices for.
    ///
    /// # Returns
    /// The computed element and bit indices from canonized index `index`.
    ///
    /// # Examples
    /// Given a $4 \times 12$ Matrix with a Vec<u8> backing storage:
    /// * $(2, 3) \rightarrow (1, 1)$
    /// * $(6, 7) \rightarrow (9, 7)$
    /// * $(0, 11) \rightarrow (1, 3)$
    fn get_element_bit_index_from_canon_index(&self, index: (usize, usize)) -> (usize, usize) {
        let (bit_row, bit_col) = index;
        let flat_bit_idx = self.get_cols() * bit_row + bit_col;
        let bits_per_t = T::n_bits();

        let element_idx = flat_bit_idx / bits_per_t;
        let bit_idx = flat_bit_idx % bits_per_t;

        (element_idx, bit_idx)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{MatrixConstructError, ToroidalBinaryMatrix, ToroidalBitMatrix};
    #[test]
    fn test_new_ok() {
        let table_1 = vec![vec![false, false, false], vec![false, false, true]];
        let table_2 = vec![vec![false], vec![true], vec![true], vec![true]];

        let mat_1 = ToroidalBitMatrix::<u32>::new(table_1).unwrap();
        let mat_2 = ToroidalBitMatrix::<u32>::new(table_2).unwrap();

        assert_eq!(mat_1.get_rows(), 2);
        assert_eq!(mat_1.get_cols(), 3);

        assert_eq!(mat_2.get_rows(), 4);
        assert_eq!(mat_2.get_cols(), 1);
    }

    #[test]
    fn test_new_empty() {
        let empty_table_1: std::vec::Vec<std::vec::Vec<bool>> = vec![];
        let empty_table_2: std::vec::Vec<std::vec::Vec<bool>> = vec![vec![], vec![]];

        let mat_1 = ToroidalBitMatrix::<u32>::new(empty_table_1);
        let mat_2 = ToroidalBitMatrix::<u32>::new(empty_table_2);

        assert!(matches!(mat_1, Err(MatrixConstructError::EmptyTable())));
        assert!(matches!(mat_2, Err(MatrixConstructError::EmptyTable())));
    }

    #[test]
    fn test_new_ragged() {
        let ragged_table = vec![vec![false], vec![false, true]];

        let mat_ragged = ToroidalBitMatrix::<u32>::new(ragged_table);

        assert!(matches!(mat_ragged, Err(MatrixConstructError::RaggedTable())));
    }

    #[test]
    fn test_new_empty_ragged() {
        let table_1 = vec![vec![false], vec![false, true], vec![]];
        let table_2 = vec![vec![], vec![false, true], vec![false]];

        let mat_1 = ToroidalBitMatrix::<u32>::new(table_1);
        let mat_2 = ToroidalBitMatrix::<u32>::new(table_2);

        assert!(matches!(mat_1, Err(MatrixConstructError::EmptyTable())));
        assert!(matches!(mat_2, Err(MatrixConstructError::EmptyTable())));
    }
}
