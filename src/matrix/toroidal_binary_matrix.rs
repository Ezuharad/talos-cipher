// 2025 Steven Chiacchira
use std::error::Error;
use std::fmt;

/// Type used to specify elements of a [`ToroidalBinaryMatrix`].
pub type ToroidalMatrixIndex = (isize, isize);

/// Error occurring during Matrix initialization
#[derive(Debug)]
pub enum MatrixConstructError {
    /// Every row of the table used to define a Matrix's initial state must have the same number of columns
    RaggedTable(),
    /// A Matrix cannot have no cells
    EmptyTable(),
    /// A Matrix should have precisely enough elements to store its entries.
    InvalidStorage(),
}

impl Error for MatrixConstructError {}
impl fmt::Display for MatrixConstructError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::RaggedTable() => {
                write!(f, "Ragged table")
            }
            Self::EmptyTable() => {
                write!(f, "Empty table")
            }
            Self::InvalidStorage() => {
                write!(f, "Invalid storage")
            }
        }
    }
}

/// Error arising from applying a matrix operation
#[derive(Debug)]
pub enum MatrixOpError {
    /// Some operations require matrices to have the same shape.
    DifferentShapes(),
    /// Some operations require matrices to have compatible shapes.
    IncompatibleShapes(),
}

impl Error for MatrixOpError {}
impl fmt::Display for MatrixOpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DifferentShapes() => {
                write!(f, "Different shapes")
            }
            Self::IncompatibleShapes() => {
                write!(f, "Incompativle shapes")
            }
        }
    }
}

/// Trait specifying methods for matrices with binary entries on a torus.
pub trait ToroidalBinaryMatrix: Sized {
    /// Creates a new instance of a matrix with entries from a table of `bool` values.
    fn new(table: Vec<Vec<bool>>) -> Result<Self, MatrixConstructError>;
    /// Returns the number of rows the matrix has.
    fn get_rows(&self) -> usize;
    /// Returns the number of columns the matrix has.
    fn get_cols(&self) -> usize;
    /// Returns the value of the matrix element at `idx`. If the row or column coordinate in `idx` is
    /// negative or greater than the number of rows or columns of the matrix respectively, the
    /// modulo of the coordinate will be used. This property is what makes the matrix 'toroidal'.
    fn at(&self, idx: &ToroidalMatrixIndex) -> bool;
    /// Sets the value of the matrix element at `idx` to `value` and returns the original value.
    /// If the row or column coordinate in `idx` is negative or greater than the number of rows
    /// or columns of the matrix respectively, the modulo of the coordinate will be used. This
    /// property is what makes the matrix 'toroidal'.
    fn set(&mut self, idx: &ToroidalMatrixIndex, value: bool) -> bool;
    /// Performs bitwise xor of this matrix with `other`, returning a [`MatrixOpError`] if the two
    /// matrices have different shapes.
    fn bitwise_xor(&mut self, other: &Self) -> Result<(), MatrixOpError>;

    /// Converts `col_index` to a cannonized column index.
    /// Given col index i, canonized index i' = i % cols, where x % y is the Euclidean remainder of
    /// x / y.
    fn canonize_col_index(&self, col_index: isize) -> usize {
        col_index.rem_euclid(self.get_cols() as isize) as usize
    }
    /// Converts `row_index` to a cannonized row index.
    /// Given row index i, canonized index i' = i % rows, where x % y is the Euclidean remainder of
    /// x / y.
    fn canonize_row_index(&self, row_index: isize) -> usize {
        row_index.rem_euclid(self.get_rows() as isize) as usize
    }
    /// Converts `index` to a canonized index.
    /// Given index i = (a, b), canonized index i' = (a % rows, b % cols), where x % y is the Euclidean
    /// remainder of x / y.
    fn canonize_index(&self, index: ToroidalMatrixIndex) -> (usize, usize) {
        let (row, col) = index;
        let row_result = self.canonize_row_index(row);
        let col_result = self.canonize_col_index(col);
        (row_result, col_result)
    }
    /// Swaps the value at `entry1` with the value at `entry2`.
    fn swap_entries(&mut self, entry1: &ToroidalMatrixIndex, entry2: &ToroidalMatrixIndex) {
        let temp = self.set(entry1, self.at(entry2));
        self.set(entry2, temp);
    }
    /// Swaps the two rows indexed by `row1` and `row2` of this Matrix.
    fn swap_rows(&mut self, row1: isize, row2: isize) {
        for col in 0..self.get_cols() {
            let entry1 = (row1, col as isize);
            let entry2 = (row2, col as isize);
            self.swap_entries(&entry1, &entry2);
        }
    }
    /// Swaps the two columns indexed by `col1` and `col2` of this Matrix.
    fn swap_cols(&mut self, col1: isize, col2: isize) {
        for row in 0..self.get_rows() {
            let entry1 = (row as isize, col1);
            let entry2 = (row as isize, col2);
            self.swap_entries(&entry1, &entry2);
        }
    }
    /// Returns the number of 'alive' (1) elements in the Matrix.
    fn popcount(&self) -> u32;
}
