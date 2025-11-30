// 2025 Steven Chiacchira
use std::error::Error;
use std::fmt;

/// Type used to specify elements of a [`ToroidalBinaryMatrix`].
///
/// Given ToroidalBinaryIndex (r, c), $r \in \bb{Z}$ represents the 0-indexed row of a element, while
/// $c \in \bb{Z}$ represents the 0-indexed column of the element.
/// Due to the Matrix's toroidal nature, a ToroidalMatrixIndex's coordinates can be any integer
/// value and still be valid.
///
/// A ToroidalMatrixIndex $(r, c)$ on an $R \times C$ ToroidalMatrix where either:
/// * $r < 0 \lor c < 0$
/// * $r > R \lor c > C$
///
/// Are said to be in "noncanonical" form. A ToroidalMatrixIndex can always be made into canonical
/// form given a ToroidalBinaryMatrix. See:
/// * [`ToroidalBinaryMatrix::canonize_index`]
/// * [`ToroidalBinaryMatrix::canonize_row_index`]
/// * [`ToroidalBinaryMatrix::canonize_col_index`]
///
/// for details on toroidal coordinate canonization. All publicly-available interfaces on Toroidal
/// matrices accept noncanonical and canonical indices.
pub type ToroidalMatrixIndex = (isize, isize);

/// Error occurring during Matrix initialization
///
/// If a table is both [ragged](MatrixConstructError::RaggedTable) *and*
/// [empty](MatrixConstructError::EmptyTable) (contains an empty row), the EmptyTable enum variant
/// takes precedence.
#[derive(Debug)]
pub enum MatrixConstructError {
    /// Every row of the table used to define a Matrix's initial state must have the same number of columns
    ///
    /// # Examples
    /// The table with initial state:
    /// ```text
    /// [
    ///     [true, true, true],
    ///     [false, true, true]
    /// ]
    /// ```
    ///
    /// is *ALLOWED*, because each row has 3 elements. Conversely,
    /// ```text
    /// [
    ///     [true, true, true],
    ///     [false]
    /// ]
    /// ```
    /// is *NOT ALLOWED*, because the first column has 3 elements, while the second has only 1.
    RaggedTable(),
    /// A Matrix cannot have no elements.
    ///
    /// # Examples
    /// A $1 \times 1$ Matrix is *ALLOWED*
    /// A $0 \times 0$ Matrix is *NOT ALLOWED*
    /// A $0 \times 5$ Matrix is *NOT ALLOWED*
    ///
    /// Takes precedence over the [`MatrixConstructError::RaggedTable`] enum variant.
    EmptyTable(),
    /// A Matrix should have precisely enough elements to store its entries. Use of this variant is
    /// not *required* for implementing the [`ToroidalBinaryMatrix`] trait. See
    /// * [`ToroidalBoolMatrix::from_storage`](crate::matrix)
    /// * [`ToroidalBitMatrix::from_storage`](crate::matrix)
    ///
    /// for methods which use this enum variant.
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
    ///
    /// See:
    /// [`ToroidalBinaryMatrix::bitwise_xor`]
    ///
    /// for an operation requiring equal shapes.
    DifferentShapes(),
    /// Some operations require matrices to have compatible shapes. This enum vatriant is currently
    /// unused.
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
                write!(f, "Incompatible shapes")
            }
        }
    }
}

/// Trait specifying methods for matrices with binary entries on a genus-1 torus.
pub trait ToroidalBinaryMatrix: Sized {
    /// Creates a new Matrix instance with entries from a table of `bool` values.
    ///
    /// `table` represents the initial Matrix state, with `table[row][col]` specifying the Matrix
    /// element at `(row, col)`. Note that `table` implicitly describes the Matrix shape.
    ///
    /// It is important that `table` meet the following criteria:
    /// * `table` is non-ragged, meaning that every row has the same number of entries.
    /// * `table` is nonempty, and its rows are nonempty.
    ///
    /// See [`MatrixConstructError`] for possible error variants resulting from violating these
    /// criteria, as well as how edge cases are handled.
    ///
    /// # Arguments
    /// * `table` - a table of booleans representing the inital Matrix state
    ///
    /// # Returns
    /// A new instance of the implementing class with the state specified by `table`
    fn new(table: Vec<Vec<bool>>) -> Result<Self, MatrixConstructError>;
    /// Returns the number of rows the Matrix has.
    ///
    /// A Matrix will always have a positive (nonzero) number of rows.
    ///
    /// # Returns
    /// The number of rows the Matrix has.
    ///
    /// # Examples
    /// A $4 \times 3$ matrix has 4 rows.
    fn get_rows(&self) -> usize;
    /// Returns the number of columns the Matrix has.
    ///
    /// A Matrix will always have a positive (nonzero) number of columns.
    ///
    /// # Returns
    /// The number of columns the Matrix has
    ///
    /// # Examples
    /// A $4 \times 3$ matrix has 3 rows.
    fn get_cols(&self) -> usize;
    /// Returns the number of elements the Matrix has.
    ///
    /// A Matrix will always have a positive (nonzero) number of elements.
    ///
    /// # Returns
    /// The number of elements the Matrix has
    ///
    /// # Examples
    /// A $4 \times 3$ matrix has $4 * 3 = 12$ elements.
    fn num_elements(&self) -> usize {
        self.get_rows() * self.get_cols()
    }
    /// Returns the value of the Matrix element at possibly canonized ToroidalMatrixIndex `idx`.
    ///
    /// See:
    /// * [`ToroidalMatrixIndex`]
    /// * [`ToroidalBinaryMatrix::canonize_index`]
    /// * [`ToroidalBinaryMatrix::canonize_row_index`]
    /// * [`ToroidalBinaryMatrix::canonize_col_index`]
    ///
    /// for details on index canonization.
    ///
    /// # Arguments
    /// * `idx` - the index of the element to access
    ///
    /// # Returns
    /// The state of the accessed element
    fn at(&self, idx: &ToroidalMatrixIndex) -> bool;
    /// Sets the value of the Matrix element at possibly canonized ToroidalMatrixIndex `idx` to
    /// `value` and returns the original value.
    ///
    /// See:
    /// * [`ToroidalMatrixIndex`]
    /// * [`ToroidalBinaryMatrix::canonize_index`]
    /// * [`ToroidalBinaryMatrix::canonize_row_index`]
    /// * [`ToroidalBinaryMatrix::canonize_col_index`]
    ///
    /// for details on index canonization.
    ///
    /// # Arguments
    /// * `idx` - the index of the element to modify
    /// * `value` - the value to set the element at index to
    ///
    /// # Returns
    /// The previous value of the element at `idx`
    fn set(&mut self, idx: &ToroidalMatrixIndex, value: bool) -> bool;
    /// Performs bitwise xor of this Matrix with `other`, returning a [`MatrixOpError`] if the two
    /// matrices have different shapes. Note that this method only modifies the *calling* Matrix.
    /// It does *not* return a new Matrix.
    ///
    /// If the matrices are of different shapes this method is a no-op.
    ///
    /// # Arguments
    /// * `other` - the Matrix to compute a bitwise xor with.
    ///
    /// # Returns
    /// A [`MatrixOpError`] if the calling Matrix and `other` are of different shapes, and a unit
    /// tuple if the call succeeded
    fn bitwise_xor(&mut self, other: &Self) -> Result<(), MatrixOpError>;
    /// Converts `col_index` to a canonized column index.
    /// Given col index i, canonized index i' = i % cols, where x % y is the Euclidean remainder of
    /// x / y.
    ///
    /// Note that the Euclidean remainder is not equivalent to the modulus operator when $y < 0$. See
    /// [`u32::rem_euclid`] for details.
    ///
    /// # Arguments
    /// * `col_idx` - the column index to be canonized
    ///
    /// # Returns
    /// The canonized column index.
    ///
    /// # Examples
    /// Given a $3 \times 5$ Matrix:
    /// * $1 \rightarrow 1$
    /// * $-1 \rightarrow 3$
    /// * $5 \rightarrow 0$
    fn canonize_col_index(&self, col_index: isize) -> usize {
        col_index.rem_euclid(self.get_cols() as isize) as usize
    }
    /// Converts `row_index` to a canonized row index.
    /// Given row index i, canonized index i' = i % rows, where x % y is the Euclidean remainder of
    /// x / y.
    ///
    /// Note that the Euclidean remainder is not equivalent to the modulus operator when $y < 0$. See
    /// [`u32::rem_euclid`] for details.
    ///
    /// # Arguments
    /// * `row_index` - the row index to be canonized
    ///
    /// # Returns
    /// The canonized row index
    ///
    /// # Examples
    /// Given a $3 \times 5$ Matrix:
    /// * $1 \rightarrow 1$
    /// * $-1 \rightarrow 2$
    /// * $5 \rightarrow 2$
    fn canonize_row_index(&self, row_index: isize) -> usize {
        row_index.rem_euclid(self.get_rows() as isize) as usize
    }
    /// Converts `index` to a canonized index.
    /// Given index i = (a, b), canonized index i' = (a % rows, b % cols), where x % y is the Euclidean
    /// remainder of x / y.
    ///
    /// Note that the Euclidean remainder is not equivalent to the modulus operator when $y < 0$. See
    /// [`u32::rem_euclid`] for details.
    ///
    /// # Arguments
    /// * `index` - the ToroidalMatrixIndex to be canonized
    ///
    /// # Returns
    /// The canonized element index
    ///
    /// # Examples
    /// Given a $3 \times 5$ Matrix:
    /// * $(1, 3) \rightarrow (1, 3)$
    /// * $(-2, 15) \rightarrow (1, 0)$
    /// * $(5, -12) \rightarrow (2, -3)$
    fn canonize_index(&self, index: ToroidalMatrixIndex) -> (usize, usize) {
        let (row, col) = index;
        let row_result = self.canonize_row_index(row);
        let col_result = self.canonize_col_index(col);
        (row_result, col_result)
    }
    /// Swaps the value at `entry1` with the value at `entry2`.
    ///
    /// # Arguments
    /// * `entry1` - the first entry to be swapped
    /// * `entry2` - the second entry to be swapped
    fn swap_entries(&mut self, entry1: &ToroidalMatrixIndex, entry2: &ToroidalMatrixIndex) {
        let temp = self.set(entry1, self.at(entry2));
        self.set(entry2, temp);
    }
    /// Swaps the two rows indexed by `row1` and `row2` of this Matrix.
    ///
    /// # Arguments
    /// * `row1` - Possibly canonized index of the first row to be swapped
    /// * `row2` - Possibly canonized index of the second row to be swapped
    fn swap_rows(&mut self, row1: isize, row2: isize) {
        for col in 0..self.get_cols() {
            let entry1 = (row1, col as isize);
            let entry2 = (row2, col as isize);
            self.swap_entries(&entry1, &entry2);
        }
    }
    /// Swaps the two columns indexed by `col1` and `col2` of this Matrix.
    ///
    /// # Arguments
    /// * `col1` - Possibly canonized index of the first column to be swapped
    /// * `col2` - Possibly canonized index of the second column to be swapped
    fn swap_cols(&mut self, col1: isize, col2: isize) {
        for row in 0..self.get_rows() {
            let entry1 = (row as isize, col1);
            let entry2 = (row as isize, col2);
            self.swap_entries(&entry1, &entry2);
        }
    }
    /// Returns the number of true elements in the Matrix.
    ///
    /// # Returns
    /// The number of `true` elements in the Matrix.
    fn popcount(&self) -> u32;
}
