// 2025 Steven Chiacchira
use crate::matrix::ToroidalMatrixIndex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::iter::zip;

/// Error occurring during the reading of a string defining a table of `bool` values.
#[derive(Debug)]
pub enum TableReadError {
    /// Invalid character in the file read
    InvalidCharacter(char),
    /// Non-uniform table
    ///
    /// # Examples
    ///
    /// ```text
    /// a.b.c.d
    /// c.d#e#f
    /// ```
    /// is *ALLOWED* because each row has the same number of characters. Conversely,
    /// ```text
    /// a.b
    /// c#d#e.f
    /// ```
    /// is *NOT ALLOWED*, because row 2 has more characters than row 1.
    RaggedTable(),
}

impl Error for TableReadError {}
impl fmt::Display for TableReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidCharacter(c) => {
                write!(f, "Invalid character: {}", c)
            }
            Self::RaggedTable() => {
                write!(f, "Ragged table")
            }
        }
    }
}

/// Default keys to be used in generating a character map.
/// The character at index `i` is the base-32 representation of `i`.
///
/// |Base 32|A|B|C|D|E|F|G|H|I|J|K |L |M |N |O |P |Q |R |S |T |U |V |W |X |Y |Z |2 |3 |4 |5 |6 |7 |
/// |-------|-|-|-|-|-|-|-|-|-|-|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
/// |Base 10|0|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20|21|22|23|24|25|26|27|28|29|30|31|
pub const BASE_32_DIGITS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

/// Generates a map from base-32 digits to `bool` values from a `u32`.
///
/// See [`BASE_32_DIGITS`] for information on base-32 counting.
///
/// # Arguments
/// * `seed` - the number to generate a character map from.
///
/// # Returns
/// A [`HashMap`] mapping base-32 digits to boolean values.
///
/// # Examples
/// The number 1, represented as `00000000000000000000000000000001` with 32 digits, would create a
/// `HashMap` containing `false` for all characters except `A`, or 0 in base 32.
#[must_use]
pub fn gen_char_map(seed: u32) -> HashMap<char, bool> {
    zip(
        BASE_32_DIGITS.chars(),
        (0..BASE_32_DIGITS.len()).map(|n| (seed >> n) & 1 != 0),
    )
    .collect::<HashMap<char, bool>>()
}

/// Reads `string` as a `bool` table state with characters from `char_map`.
///
/// `string` and `char_map` must meet the following criteria:
/// * every line in `string` contains the same number of characters.
/// * every character in `string` (excluding newlines) must be a key in `char_map`.
///
/// # Arguments
/// * `string` the string to parse to a `bool` table
/// * `char_map` a map from characters to `bool` values. Note that `char_map` must have a key
///   for each character in `string` (excluding newlines).
///
/// # Returns
/// A new bool table on a success, or a [`TableReadError`] on a failure.
///
/// # Examples
/// Given `char_map = { '#': true, '.': false }`:
/// ```text
/// .....
/// ..#..
/// ...#.
/// .###.
/// ```
///
/// specifies the table
/// ```text
/// FFFFF
/// FFTFF
/// FFFTF
/// FTTTF
/// ```
pub fn parse_bool_table(
    string: &str,
    char_map: &HashMap<char, bool>,
) -> Result<Vec<Vec<bool>>, TableReadError> {
    let mut table: Vec<Vec<bool>> = Vec::new();
    for line in string.lines() {
        let val_row: Vec<bool> = line
            .chars()
            .map(|c| match char_map.get(&c) {
                Some(v) => Ok(v.to_owned()),
                None => Err(TableReadError::InvalidCharacter(c)),
            })
            .collect::<Result<Vec<bool>, TableReadError>>()?;

        table.push(val_row);
    }

    Ok(table)
}

/// Given a string representing an initial matrix state with base-32 digits for variable values,
/// returns a vector `X`, where `X\[i\]` is the set of ToroidalMatrixIndices of the base-32
/// representation of `i` in the string.
///
/// See section 2.1 of RFC-1 for details on where this method is used.
/// See also [`get_char_indices`](crate::parse) for details on obtaining the ToroidalMatrixIndices from `string`.
/// See also [`BASE_32_DIGITS`](crate::parse) for information on base-32 counting.
/// See also
///
/// # Arguments
/// * `string` - the string to find the ToroidalMatrixIndices of base-32 digits in
///
/// # Returns
/// A vector containing the ToroidalMatrixIndices for each base-32 digit in `string`
///
/// # Examples
/// For `string`
/// ```text
/// A.A.B
/// ##A.A
/// ```
/// A is at canon toroidal indices [(0, 0), (0, 2), (0, 4), (1, 2), (1, 4)]. Thus, the resulting vector
/// at index 0 ('A' in base-32) is the vector [(0, 0), (0, 2), (0, 4), (1, 2), (1, 4)].
#[must_use]
pub fn get_temporal_seed_map(string: &str) -> Vec<Vec<ToroidalMatrixIndex>> {
    let mut result = Vec::new();
    for character in BASE_32_DIGITS.chars() {
        result.push(get_char_indices(string, character))
    }
    result
}

/// Returns the indices of `character` in `string` as canonical
/// [`ToroidalMatrixIndices`](talos::matrix::ToroidalMatrixIndex).
///
/// # Arguments
/// * `string` - the string to search for `character` in
/// * `character` - the character to find the indices of in `string`
///
/// # Returns
/// A vector of positions in `string` containing `character`.
///
/// # Examples
/// For `string`
/// ```text
/// A.A.B
/// ##A.A
/// ```
/// A is at indices [(0, 0), (0, 2), (0, 4), (1, 2), (1, 4)].
#[must_use]
fn get_char_indices(string: &str, character: char) -> Vec<ToroidalMatrixIndex> {
    let mut result = Vec::new();
    for (row, line) in string.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == character {
                result.push((row as isize, col as isize));
            }
        }
    }

    result
}
