// 2025 Steven Chiacchiraenc
use clap::Parser;
use rand::random;
use std::error::Error;
use std::fmt;
use std::fs;
use talos::encrypt;

#[derive(Parser, Debug)]
struct EncryptArgs {
    input: String,
    out: String,
    #[arg(short, long)]
    key: Option<u32>,
}

#[derive(Debug)]
enum EncryptError {
    FileReadError(),
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
    let seed = match args.key {
        Some(seed) => seed,
        None => random::<u32>(),
    };

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
