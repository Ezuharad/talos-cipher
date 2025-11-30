// 2025 Steven Chiacchira
use crate::matrix::{
    MatrixConstructError, MatrixOpError, ToroidalBinaryMatrix, ToroidalMatrixIndex,
};

#[derive(Debug, Clone)]
/// Struct implementing [`ToroidalBinaryMatrix`] backed by a `Vec<bool>`.
pub struct ToroidalBoolMatrix {
    pub rows: usize,
    pub cols: usize,
    storage: Vec<bool>,
}

impl ToroidalBinaryMatrix for ToroidalBoolMatrix {
    fn get_rows(&self) -> usize {
        self.rows
    }
    fn get_cols(&self) -> usize {
        self.cols
    }
    fn new(table: Vec<Vec<bool>>) -> Result<Self, MatrixConstructError> {
        let rows = table.len();
        let cols = if rows == 0 { 0 } else { table[0].len() };
        if cols == 0 {
            return Err(MatrixConstructError::EmptyTable());
        }

        if table
            .iter()
            .any(|row| row.is_empty())
        {
            return Err(MatrixConstructError::EmptyTable());
        }

        let cols = table[0].len();
        if table
            .iter()
            .any(|row| row.len() != cols)
        {
            return Err(MatrixConstructError::RaggedTable());
        }

        let storage = table.into_iter().flatten().collect();

        Ok(Self {
            rows,
            cols,
            storage,
        })
    }
    fn at(&self, idx: &ToroidalMatrixIndex) -> bool {
        let (row, col) = self.canonize_index(*idx);
        let vec_idx: usize = row * self.cols + col;

        self.storage[vec_idx]
    }
    fn set(&mut self, idx: &ToroidalMatrixIndex, value: bool) -> bool {
        let (row, col) = self.canonize_index(*idx);

        let vec_idx: usize = row * self.get_cols() + col;
        let result = self.storage[vec_idx];
        self.storage[vec_idx] = value;

        result
    }

    fn bitwise_xor(&mut self, other: &ToroidalBoolMatrix) -> Result<(), MatrixOpError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MatrixOpError::DifferentShapes());
        }
        for i in 0..(self.rows * self.cols) {
            self.storage[i] = self.storage[i] != other.storage[i];
        }
        Ok(())
    }
    fn swap_rows(&mut self, row1: isize, row2: isize) {
        let row_1_idx: usize = self.canonize_col_index(row1);
        let row_2_idx: usize = self.canonize_col_index(row2);
        let offset_1 = row_1_idx * self.cols;
        let offset_2 = row_2_idx * self.cols;

        for i in 0..self.cols {
            self.storage.swap(offset_1 + i, offset_2 + i);
        }
    }
    fn popcount(&self) -> u32 {
        self.storage.iter().map(|b| *b as u32).sum()
    }
}

impl ToroidalBoolMatrix {
    /// Returns the storage backing the Matrix.
    ///
    /// # Returns
    /// The storage backing the Matrix.
    pub fn get_storage(&self) -> &Vec<bool> {
        &self.storage
    }
    /// Constructs a new [`ToroidalBoolMatrix`] from storage, as well as the count of rows and
    /// columns.
    ///
    /// The following criteria must be met for Matrix construction:
    /// * $rows > 0 \land cols > 0$
    /// * $rows * cols$ = storage.size()
    ///
    /// See [`MatrixConstructError`] for possible error variants resulting from violating these
    /// criteria.
    ///
    /// # Arguments
    /// * `rows` - the number of rows the Matrix will have
    /// * `cols` - the number of columns the Matrix will have
    /// * `storage` the storage backing the Matrix. Note that elements are stored in row-major
    ///   order.
    ///
    /// # Returns
    /// A newly constructed Matrix if the storage, rows, and columns are valid, and a
    /// [`MatrixConstructError`] otherwise.
    pub fn from_storage(
        rows: usize,
        cols: usize,
        storage: Vec<bool>,
    ) -> Result<Self, MatrixConstructError> {
        if rows == 0 || cols == 0 {
            return Err(MatrixConstructError::EmptyTable());
        }
        if storage.len() != rows * cols {
            return Err(MatrixConstructError::InvalidStorage());
        }
        Ok(Self {
            rows,
            cols,
            storage,
        })
    }
}
