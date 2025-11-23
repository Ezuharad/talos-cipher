// 2025 Steven Chiacchiraenc
use clap::Parser;
use rand::random;
use std::error::Error;
use std::fmt;
use std::fs;
use talos::{encrypt, parse};

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

    let mut char_map = parse::gen_char_map(seed);
    char_map.insert('#', true);
    char_map.insert('.', false);

    let (maybe_t_automaton, maybe_s_automaton) = encrypt::get_transpose_shift_automata(char_map);

    let mut t_automaton = maybe_t_automaton.unwrap();
    let mut s_automaton = maybe_s_automaton.unwrap();

    let t_temporal_seed_map = parse::get_temporal_seed_map(encrypt::T_INIT_MATRIX);
    let s_temporal_seed_map = parse::get_temporal_seed_map(encrypt::S_INIT_MATRIX);

    encrypt::temporal_seed_automaton(&mut t_automaton, seed, &t_temporal_seed_map);
    encrypt::temporal_seed_automaton(&mut s_automaton, seed, &s_temporal_seed_map);

    let input_buffer = match fs::read(args.input) {
        Ok(buffer) => buffer,
        Err(_) => {
            return Err(EncryptError::FileReadError());
        }
    };

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
