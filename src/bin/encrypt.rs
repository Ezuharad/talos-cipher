// 2025 Steven Chiacchiraenc
use clap::Parser;
use std::error::Error;
use std::fmt;
use std::fs;
use talos::{encrypt, key};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
/// CLI tool for encryption using the Talos encryption protocol.
struct EncryptArgs {
    /// The file to be encrypted
    input: String,
    /// The file path to save ciphertext to
    out: String,
    #[arg(short, long)]
    /// The encryption key to be used. If a numerical input is given, it will be used as the
    /// encryption key. If a string is given, it will be used to deterministically generate a key
    /// using SHA2567. If no key is given, a random key will be used and displayed to the user.
    key: Option<key::KeyArgument>,
}

#[derive(Debug)]
/// Possible error states for CLI encryption.
enum EncryptError {
    /// An error occurred reading the specified plaintext file.
    FileReadError(),
    /// An error occurred writing to the specified output file.
    FileWriteError(),
}

impl Error for EncryptError {}
impl fmt::Display for EncryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileReadError() => {
                write!(f, "Error reading file")
            }
            Self::FileWriteError() => {
                write!(f, "Error writing output")
            }
        }
    }
}

fn main() -> Result<(), EncryptError> {
    let args = EncryptArgs::parse();
    let seed = args.key.unwrap_or(key::KeyArgument::None).get();

    let input_buffer = match fs::read(args.input) {
        Ok(buffer) => buffer,
        Err(_) => {
            return Err(EncryptError::FileReadError());
        }
    };

    let (mut s_automaton, mut t_automaton) = encrypt::get_transpose_shift_automata(seed);

    eprintln!("Using key {}", seed);
    let now = std::time::Instant::now();
    let output_bytes =
        encrypt::encrypt_message_256(input_buffer, &mut s_automaton, &mut t_automaton);

    eprintln!(
        "Finished encrypting in {} miliseconds",
        now.elapsed().as_millis()
    );

    let result = fs::write(args.out, output_bytes);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(EncryptError::FileWriteError()),
    }
}
