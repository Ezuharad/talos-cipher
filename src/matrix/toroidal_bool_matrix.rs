// 2025 Steven Chiacchira
use crate::matrix::{MatrixConstructError, ToroidalMatrixIndex, MatrixOpError, ToroidalBinaryMatrix};

#[derive(Debug, Clone)]
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

        // if the table is ragged (every column is not the same size) then we reject the input and return an Err result
        if table
            .iter()
            .map(|row| row.len() != cols)
            .fold(false, |a, b| a | b)
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
    /// Returns the storage backing the matrix.
    pub fn get_storage(&self) -> &Vec<bool> {
        &self.storage
    }
    /// Constructs a new [`ToroidalBoolMatrix`] from storage, as well as the count of rows and
    /// columns. Returns an error if the storage is the wrong size for the specified matrix shape.
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
