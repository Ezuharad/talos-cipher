// 2025 Steven Chiacchira
use crate::automata::{ToroidalAutomaton, AutomatonRule};
use crate::matrix::{ToroidalBinaryMatrix, ToroidalBitMatrix, ToroidalMatrixIndex};
use crate::parse;

/// Number of rows in a matrix for the Talos encryption protocol.
pub const N_ROWS: usize = 16;
/// Number of columns in a matrix for the Talos encryption protocol.
pub const N_COLS: usize = 16;
/// Number of elements in an encryption block for the Talos encryption protocol.
pub const BLOCK_SIZE: usize = N_ROWS * N_COLS;

/// Initialization string for Transpose Matrix. See RFC-0 section 2.2.1 for details.
pub const T_INIT_MATRIX: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/init_matrix/T_init_matrix.txt"
));
/// Initialization string for Shift Matrix. See RFC-0 section 2.2.1 for details.
pub const S_INIT_MATRIX: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/init_matrix/S_init_matrix.txt"
));
/// Automaton rule used in the Talos encryption protocol. See RFC-0 section 2.2.2 for details.
pub const AUTOMATA_RULE: AutomatonRule = AutomatonRule {
    born: [false, false, true, true, true, true, true, false, false],
    dies: [true, true, false, false, false, true, true, true, true],
};

/// Number of iterations to perform for each encryption block.
const N_ITERS_PER_BLOCK: u32 = 11;

/// A ToroidalBitMatrix backed by a `Vec<u8>`. Allows for quick reading of character values.
pub type TalosMatrix = ToroidalBitMatrix<u8>;
/// A cellular automaton using a ToroidalBitMatrix backed by a `Vec<u8>`.
pub type TalosAutomaton = ToroidalAutomaton<TalosMatrix>;

/// Prepares and returns the transpose and shift automata proposed in RFC-0 section 2.
///
/// Performs:
/// * initialization based on the initial matrices proposed in RFC-0 section 2.2.1
/// * temporal seeding proposed in RFC-1 section 2.1
///
/// for both the shift and transpose automata.
///
/// # Arguments
/// * `seed` - the seed to use for automaton initialization and temporal seeding.
///
/// # Returns
/// A tuple containing the initialized transpose and shift automata.
#[must_use]
pub fn get_transpose_shift_automata(seed: u32) -> (TalosAutomaton, TalosAutomaton) {
    let mut char_map = parse::gen_char_map(seed);
    char_map.insert('#', true);
    char_map.insert('.', false);

    let s_table = parse::parse_bool_table(S_INIT_MATRIX, &char_map).unwrap();
    let t_table = parse::parse_bool_table(T_INIT_MATRIX, &char_map).unwrap();

    let s_state = TalosMatrix::new(s_table).unwrap();
    let t_state = TalosMatrix::new(t_table).unwrap();

    let mut s_automaton = ToroidalAutomaton::new(s_state, AUTOMATA_RULE);
    let mut t_automaton = ToroidalAutomaton::new(t_state, AUTOMATA_RULE);

    let s_temporal_seed_map = parse::get_temporal_seed_map(S_INIT_MATRIX);
    let t_temporal_seed_map = parse::get_temporal_seed_map(T_INIT_MATRIX);

    temporal_seed_automaton(&mut s_automaton, seed, &s_temporal_seed_map);
    temporal_seed_automaton(&mut t_automaton, seed, &t_temporal_seed_map);

    (s_automaton, t_automaton)
}

/// Encrypts a message with a 256 bit block using the Talos algorithm.
///
/// <div class="warning">
/// *DOES NOT* perform the temporal seeding as defined in RFC-1 section 2.1.
/// Matrix initialization is performed by other methods. See [`get_transpose_shift_automata`].
/// </div>
///
/// # Arguments
/// * `message` - the plaintext to be encrypted as a vector of bytes
/// * `shift_automata` - the initial automaton to be used for shifting during decryption
/// * `transpose_automata` - the initial automaton to be used for scrambling during decryption
///
/// # Returns
/// The encrypted message as a vector of bytes.
#[must_use]
pub fn encrypt_message_256(
    message: Vec<u8>,
    shift_automata: &mut TalosAutomaton,
    transpose_automata: &mut TalosAutomaton,
) -> Vec<u8> {
    let blocks = block_split_256_message(message);

    blocks
        .iter()
        .flat_map(|b| encrypt_block_256(b.to_vec(), shift_automata, transpose_automata))
        .collect()
}

/// Decrypts a message with a 256 bit block using the Talos algorithm.
///
/// <div class="warning">
/// *DOES NOT* perform the temporal seeding as defined in RFC-1 section 2.1.
/// Matrix initialization is performed by other methods. See [`get_transpose_shift_automata`].
/// </div>
///
/// # Arguments
/// * `ciphertext` - the ciphertext to be decrypted as a vector of bytes
/// * `shift_automata` - the initial automaton to be used for shifting during decryption
/// * `transpose_automata` - the initial automaton to be used for unscrambling during decryption
///
/// # Returns
/// The decrypted message as a vector of bytes.
#[must_use]
pub fn decrypt_message_256(
    ciphertext: Vec<u8>,
    shift_automata: &mut TalosAutomaton,
    transpose_automata: &mut TalosAutomaton,
) -> Vec<u8> {
    let blocks = block_split_256_message(ciphertext);
    blocks
        .iter()
        .flat_map(|b| decrypt_block_256(b.to_vec(), shift_automata, transpose_automata))
        .collect()
}

/// Applies the matrix scrambling algorithm $V$ explained in RFC-0 section 2.2.3.
///
/// # Arguments
/// * `message_matrix` - the matrix to scramble with $V$. Modified inplace
/// * `key` - the key to use for unscrambling
fn scramble_matrix_256<T: ToroidalBinaryMatrix>(message_matrix: &mut T, key: &T) {
    for row_block in 0..4 {
        // iterate over each row in the 'row block' and swap
        let block_offset: isize = 4 * row_block;
        for (row_offset, col_offset) in [0, 2, 1, 3].iter().enumerate() {
            let (r_offset, c_offset) = (row_offset as isize, *col_offset as isize);
            let row_swap_idx = read_4_bits(
                key,
                (block_offset + r_offset, c_offset),
                (block_offset + r_offset, 4 + c_offset),
                (block_offset + r_offset, 8 + c_offset),
                (block_offset + r_offset, 12 + c_offset),
            ) as isize;
            message_matrix.swap_rows(block_offset, row_swap_idx);
        }
    }
    for col_block in 0..4 {
        // iterate over each col in the 'col block' and swap
        let block_offset: isize = 4 * col_block;
        for (col_offset, row_offset) in [3, 0, 2, 1].iter().enumerate() {
            let (r_offset, c_offset) = (*row_offset as isize, col_offset as isize);
            let row_swap_idx = read_4_bits(
                key,
                (r_offset, block_offset + c_offset),
                (4 + r_offset, block_offset + c_offset),
                (8 + r_offset, block_offset + c_offset),
                (12 + r_offset, block_offset + c_offset),
            ) as isize;
            message_matrix.swap_rows(block_offset, row_swap_idx);
        }
    }
}

/// Applies the inverse matrix scrambling algorithm $V^(-1)$ explained in RFC-0 section 2.2.3.
///
/// # Arguments
/// * `message_matrix` - the matrix to unscramble with $V^(-1)$. Modified inplace
/// * `key` - the key to use for unscrambling
fn unscramble_matrix_256<T: ToroidalBinaryMatrix>(message_matrix: &mut T, key: &T) {
    for col_block in (0..4).rev() {
        // iterate over each col in the 'col block' and swap
        let block_offset: isize = 4 * col_block;
        for (col_offset, row_offset) in [3, 0, 2, 1].iter().enumerate().rev() {
            let (r_offset, c_offset) = (*row_offset as isize, col_offset as isize);
            let row_swap_idx = read_4_bits(
                key,
                (r_offset, block_offset + c_offset),
                (4 + r_offset, block_offset + c_offset),
                (8 + r_offset, block_offset + c_offset),
                (12 + r_offset, block_offset + c_offset),
            ) as isize;
            message_matrix.swap_rows(block_offset, row_swap_idx);
        }
    }

    for row_block in (0..4).rev() {
        // iterate over each row in the 'row block' and swap
        let block_offset: isize = 4 * row_block;
        for (row_offset, col_offset) in [0, 2, 1, 3].iter().enumerate().rev() {
            let (r_offset, c_offset) = (row_offset as isize, *col_offset as isize);
            let col_swap_idx = read_4_bits(
                key,
                (block_offset + r_offset, c_offset),
                (block_offset + r_offset, 4 + c_offset),
                (block_offset + r_offset, 8 + c_offset),
                (block_offset + r_offset, 12 + c_offset),
            ) as isize;
            message_matrix.swap_rows(block_offset, col_swap_idx);
        }
    }
}

/// Splits `message` into 256 bit blocks, represented as flat vectors of bytes.
///
/// Each element of the result will contain 32 `u8s`.
/// The final block of `message` is padded to 256 bits.
///
/// # Arguments
/// * `message` - the message to split into blocks
///
/// # Returns
/// `message` split into blocks containing 256 bits (32 u8s).
#[must_use]
fn block_split_256_message(message: Vec<u8>) -> Vec<Vec<u8>> {
    let u8s_per_block = BLOCK_SIZE / u8::BITS as usize;
    let mut blocks: Vec<Vec<u8>> = message.chunks(u8s_per_block).map(|c| c.to_vec()).collect();

    if let Some(last) = blocks.last_mut() {
        last.resize(u8s_per_block, 0_u8);
    }

    blocks
}

/// Encrypts a 256 bit message block with the Talos algorithm.
#[must_use]
fn encrypt_block_256(
    message_block: Vec<u8>,
    shift_automata: &mut TalosAutomaton,
    transpose_automata: &mut TalosAutomaton,
) -> Vec<u8> {
    let mut message_matrix = TalosMatrix::from_storage(N_ROWS, N_COLS, message_block).unwrap();
    shift_automata.iter_rule(N_ITERS_PER_BLOCK);
    transpose_automata.iter_rule(N_ITERS_PER_BLOCK);

    scramble_matrix_256(&mut message_matrix, shift_automata.get_state());
    let _ = message_matrix.bitwise_xor(transpose_automata.get_state());

    message_matrix.get_storage().to_vec()
}

/// Decrypts a 256 bit message block with the Talos algorithm.
///
/// # Arguments
/// * `encrypted_block` - the block to decrypt as a vector of bytes
/// * `shift_automata` - the Automaton to use as the shift automaton
/// * `transpose_automata` - the Automaton to use for the scrambling algorithm
///
/// # Returns
/// The decrypted ciphertext block as a vector of bytes.
#[must_use]
fn decrypt_block_256(
    encrypted_block: Vec<u8>,
    shift_automata: &mut TalosAutomaton,
    transpose_automata: &mut TalosAutomaton,
) -> Vec<u8> {
    let mut message_matrix = TalosMatrix::from_storage(N_ROWS, N_COLS, encrypted_block).unwrap();
    shift_automata.iter_rule(N_ITERS_PER_BLOCK);
    transpose_automata.iter_rule(N_ITERS_PER_BLOCK);

    let _ = message_matrix.bitwise_xor(transpose_automata.get_state());
    unscramble_matrix_256(&mut message_matrix, shift_automata.get_state());

    message_matrix.get_storage().to_vec()
}

/// Performs temporal seeding as described in RFC-1 section 2.1.
///
/// # Arguments
/// * `automaton` - the `ToroidalAutomaton` to be seeded.
/// * `key` - the key to use for temporal seeding.
/// * `seed_positions` - a vector containing the ToroidalMatrixIndices to seed each key bit at.
///   `seed_positions[i]` contains the ToroidalMatrixIndices in `automaton` which will be set to
///   `key` bit `i`.
pub fn temporal_seed_automaton(
    automaton: &mut TalosAutomaton,
    key: u32,
    seed_positions: &[Vec<ToroidalMatrixIndex>],
) {
    automaton.iter_rule(8);
    for (bit_pos, seed_position) in seed_positions.iter().enumerate() {
        let overwritten_value: bool = (key >> bit_pos & 1) > 0;
        for matrix_idx in seed_position {
            automaton.set_state(matrix_idx, overwritten_value);
        }
    }
    automaton.iter_rule(8);
}

/// Reads 4 bit values at `idx0`, `idx`, `idx2`, `idx3`, in `matrix`, then concatenates them into a
/// `u8`.
///
/// See section 2.2.3 for details on the matrix scrambling algorithm $V$ as well as the matrix
///   unscrambling algorithm $V^{-1}$ where this is used.
///
/// # Arguments
/// * `matrx` - the matrix to read from.
/// * `idx0` - the first index to read a bit value from.
/// * `idx1` - the second index to read a bit value from.
/// * `idx2` - the third index to read a bit value from.
/// * `idx3` - the fourth index to read a bit value from.
///
/// # Returns
/// The concatenation of the read values as a `u8`.
///
/// # Examples
/// Given that the values [0, 1, 1, 0] are read for `idx0`, `idx1`, `idx2`, `idx3` respectively,
/// returns the value `6`, or `01100000` in binary.
#[must_use]
pub fn read_4_bits<T: ToroidalBinaryMatrix>(
    matrix: &T,
    idx0: ToroidalMatrixIndex,
    idx1: ToroidalMatrixIndex,
    idx2: ToroidalMatrixIndex,
    idx3: ToroidalMatrixIndex,
) -> u8 {
    let mut result: u8 = 0;
    for (i, idx) in [idx0, idx1, idx2, idx3].iter().enumerate() {
        result += if matrix.at(idx) {
            2_u8.pow(i as u32)
        } else {
            0
        };
    }

    result
}
