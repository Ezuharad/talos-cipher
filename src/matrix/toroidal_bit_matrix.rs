// 2025 Steven Chiacchira
use crate::bits::{Bit, BitWise};
use crate::key;
use crate::matrix::{
    MatrixConstructError, MatrixOpError, ToroidalBinaryMatrix, ToroidalMatrixIndex,
};

#[derive(Debug, Clone)]
/// Struct implementing [`ToroidalBinaryMatrix`] backed by a `Vec<T>`. `T` must be an unsigned
/// integer primitive such as `u8` or `u64`.
pub struct ToroidalBitMatrix<T: key::Key> {
    rows: usize,
    cols: usize,
    storage: Vec<T>,
}

impl<T: key::Key> ToroidalBinaryMatrix for ToroidalBitMatrix<T> {
    fn get_n_rows(&self) -> usize {
        self.rows
    }
    fn get_n_cols(&self) -> usize {
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
        let n_storage_entries = n_bits.div_ceil(n_bits_per_entry as usize);
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

        unsafe { element.get_bit_unchecked(bit_idx).is_set() }
    }
    fn set(&mut self, idx: &ToroidalMatrixIndex, new_val: bool) -> bool {
        let (row, col) = self.canonize_index(*idx);
        let (element_idx, bit_idx) = self.get_element_bit_index_from_canon_index((row, col));
        let element = &mut self.storage[element_idx];

        unsafe {
            element
                .set_bit_unchecked(bit_idx, Bit::from(new_val))
                .is_set()
        }
    }
    fn bitwise_xor(&mut self, other: &Self) -> Result<(), MatrixOpError> {
        if self.get_n_cols() != other.get_n_cols() || self.get_n_rows() != other.get_n_rows() {
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
    #[must_use]
    pub fn get_storage(&self) -> &Vec<T> {
        &self.storage
    }
    /// Constructs a new [`ToroidalBitMatrix`] from storage in row-major fashion, as well as the count
    /// of rows and columns.
    ///
    /// Because `storage` contains bits in chunks of size `T::BITS`, it is possible that `rows` *
    /// `cols` will be greater than the number of bits in `storage`. In this case, extra bits at
    /// the end of the vector will be zeroed.
    ///
    /// The following criteria must be met for Matrix construction:
    /// * `rows > 0 && cols > 0`
    /// * `rows * cols >= storage.size() * T::BITS && rows * cols < (storage.size() + 1) *
    ///   T::BITS`
    ///
    /// Where `T::BITS` is the number of bits in the [`key::Key`] implementing type `T`. See [`MatrixConstructError`]
    /// for possible error variants resulting from violating these criteria. Note that
    /// [`MatrixConstructError::RaggedTable`] is never returned from this constructor.
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
        mut storage: Vec<T>,
    ) -> Result<Self, MatrixConstructError> {
        if rows == 0 || cols == 0 || storage.is_empty() {
            return Err(MatrixConstructError::EmptyTable());
        }
        let bits_per_t = T::n_bits();
        let n_bits = (rows * cols) as u32;
        if n_bits.div_ceil(bits_per_t) != (storage.len() as u32) {
            return Err(MatrixConstructError::InvalidStorage());
        }

        let n_bits_in_storage = bits_per_t * (storage.len() as u32);
        let n_extra_bits = n_bits_in_storage - n_bits;
        if n_extra_bits > 0 {
            let last_byte = storage.last_mut().unwrap();
            let bit_mask = T::max_value() << (n_extra_bits as usize);

            *last_byte = *last_byte & bit_mask;
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
    /// Given a `4 \times 12` Matrix with a `Vec<u8>` backing storage:
    /// * `(2, 3) -> (1, 1)`
    /// * `(6, 7) -> (9, 7)`
    /// * `(0, 11) -> (1, 3)`
    #[must_use]
    fn get_element_bit_index_from_canon_index(&self, index: (usize, usize)) -> (usize, usize) {
        let (bit_row, bit_col) = index;
        let flat_bit_idx = self.get_n_cols() * bit_row + bit_col;
        let bits_per_t = T::n_bits() as usize;

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

        assert_eq!(mat_1.get_n_rows(), 2);
        assert_eq!(mat_1.get_n_cols(), 3);

        assert_eq!(mat_2.get_n_rows(), 4);
        assert_eq!(mat_2.get_n_cols(), 1);
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

        assert!(matches!(
            mat_ragged,
            Err(MatrixConstructError::RaggedTable())
        ));
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

    #[test]
    fn test_from_storage_ok() {
        let storage = vec![
            0b0000_0000_0000_0000_0000_0000_0000_0000u32,
            0b0000_0000_0000_0000_0000_0000_0000_0000u32,
            0b0000_0000_0000_0000_0000_0000_0000_0000u32,
        ];

        let mat_1 = ToroidalBitMatrix::<u32>::from_storage(3, 32, storage.clone()).unwrap();
        let mat_2 = ToroidalBitMatrix::<u32>::from_storage(32, 3, storage.clone()).unwrap();
        let mat_3 = ToroidalBitMatrix::<u32>::from_storage(31, 3, storage.clone()).unwrap();

        assert_eq!(mat_1.get_n_rows(), 3);
        assert_eq!(mat_1.get_n_cols(), 32);

        assert_eq!(mat_2.get_n_rows(), 32);
        assert_eq!(mat_2.get_n_cols(), 3);

        assert_eq!(mat_3.get_n_rows(), 31);
        assert_eq!(mat_3.get_n_cols(), 3);
    }

    #[test]
    fn test_from_storage_empty() {
        let storage = vec![
            0b0000_0000_0000_0000_0000_0000_0000_0000u32,
            0b0000_0000_0000_0000_0000_0000_0000_0000u32,
            0b0000_0000_0000_0000_0000_0000_0000_0000u32,
        ];

        let empty_storage: std::vec::Vec<u32> = vec![];

        let err_1 = ToroidalBitMatrix::<u32>::from_storage(0, 0, storage.clone());
        let err_2 = ToroidalBitMatrix::<u32>::from_storage(1, 0, storage.clone());
        let err_3 = ToroidalBitMatrix::<u32>::from_storage(0, 1, storage.clone());
        let err_4 = ToroidalBitMatrix::<u32>::from_storage(1, 1, empty_storage.clone());

        assert!(matches!(err_1, Err(MatrixConstructError::EmptyTable())));
        assert!(matches!(err_2, Err(MatrixConstructError::EmptyTable())));
        assert!(matches!(err_3, Err(MatrixConstructError::EmptyTable())));
        assert!(matches!(err_4, Err(MatrixConstructError::EmptyTable())));
    }

    #[test]
    fn test_from_storage_invalid() {
        let storage = vec![
            0b0000_0000_0000_0000_0000_0000_0000_0000u32,
            0b0000_0000_0000_0000_0000_0000_0000_0000u32,
            0b0000_0000_0000_0000_0000_0000_0000_0000u32,
        ];

        // because storage contains 32 + 32 + 32 = 96 bits in 32-bit chunks, any number of bits not
        // in [65, 96] is invalid

        let err_1 = ToroidalBitMatrix::<u32>::from_storage(64, 1, storage.clone());
        let err_2 = ToroidalBitMatrix::<u32>::from_storage(97, 1, storage.clone());

        assert!(matches!(err_1, Err(MatrixConstructError::InvalidStorage())));
        assert!(matches!(err_2, Err(MatrixConstructError::InvalidStorage())));
    }
}
