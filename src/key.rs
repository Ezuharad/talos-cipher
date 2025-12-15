// 2025 Steven Chiacchira
use std::str::FromStr;

use num_traits;
use rand;
use sha2::{Digest, Sha256};

pub trait Key = num_traits::PrimInt + num_traits::Unsigned;

#[derive(Debug, Clone)]
/// Enum of possible input key values. Used for the encryption and decryption CLI interfaces.
///
/// Allows for computing encryption keys from Strings and unsigned integers.
pub enum KeyArgument {
    /// A string to be used to generate an encryption key.
    String(String),
    /// A number to be used as an encryption key.
    Num(u32),
    /// No encryption key provided, indicating that one should be generated.
    None,
}

impl KeyArgument {
    /// Computes or generates an encryption key.
    ///
    /// The following three behavior variants are possible:
    /// * If `KeyArgument` is a `KeyArgument::String`, an encryption key will be deterministically
    ///   generated via sha256. See [`sha2`] crate for details.
    /// * If `KeyArgument` is a `KeyArgument::Num`, its value will be used as an encryption key.
    /// * If `KeyArgument` is a `KeyArgument::None`, a random key will be generated.
    ///
    /// # Returns
    /// An encryption key.
    #[must_use]
    pub fn get(self) -> u32 {
        match self {
            Self::String(key) => {
                let mut hasher = Sha256::new();
                hasher.update(key.as_bytes());
                let bytes = hasher.finalize();

                let first_four_bytes: [u8; 4] = bytes[0..4].try_into().unwrap();
                u32::from_le_bytes(first_four_bytes)
            }
            Self::Num(key) => key,
            Self::None => rand::random::<u32>(),
        }
    }
}

impl FromStr for KeyArgument {
    // Taken from https://stackoverflow.com/questions/73658377/how-to-have-number-or-string-as-a-cli-argument-in-clap
    // We don't ever parse to the None variant, so this works (use Option<KeyArgument> for clap)
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u32>()
            .map(KeyArgument::Num)
            .unwrap_or_else(|_| KeyArgument::String(s.to_string())))
    }
}
