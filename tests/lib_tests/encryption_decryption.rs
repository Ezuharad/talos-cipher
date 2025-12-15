// 2025 Steven Chiacchira
use std::fs;

#[test]
#[cfg_attr(miri, ignore)]
fn test_encrypt_decrypt_equal() {
    let message_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01.txt");
    let message =
        fs::read(message_file).expect("Could not find plaintext in data/tests directory.");
    let message_size = message.len();

    for i in 0..32 {
        let (mut s_automaton, mut t_automaton) = talos::encrypt::get_transpose_shift_automata(i);
        let ciphertext = talos::encrypt::encrypt_message_256(
            message.clone(),
            &mut s_automaton,
            &mut t_automaton,
        );

        let (mut s_automaton, mut t_automaton) = talos::encrypt::get_transpose_shift_automata(i);
        let decrypted =
            talos::encrypt::decrypt_message_256(ciphertext, &mut s_automaton, &mut t_automaton);

        // It is possible we will have leftover bits
        assert_eq!(message, decrypted[..message_size]);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_decrypt_breaking() {
    let message_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01.txt");
    let message =
        fs::read(message_file).expect("Could not find plaintext in data/tests directory.");
    let message_size = message.len();

    for i in 0..3 {
        let encrypted_file =
            env!("CARGO_MANIFEST_DIR").to_owned() + &format!("/data/tests/text_01_k{}.enc", i);
        let ciphertext =
            fs::read(encrypted_file).expect("Could not find ciphertext in data/tests directory.");

        let (mut s_automaton, mut t_automaton) = talos::encrypt::get_transpose_shift_automata(i);
        let decrypted =
            talos::encrypt::decrypt_message_256(ciphertext, &mut s_automaton, &mut t_automaton);

        assert_eq!(message, decrypted[..message_size]);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_encrypt_breaking() {
    let message_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01.txt");
    let message =
        fs::read(message_file).expect("Could not find plaintext in data/tests directory.");

    for i in 0..3 {
        let (mut s_automaton, mut t_automaton) = talos::encrypt::get_transpose_shift_automata(i);
        let encrypted_message = talos::encrypt::encrypt_message_256(
            message.clone(),
            &mut s_automaton,
            &mut t_automaton,
        );

        let encrypted_file =
            env!("CARGO_MANIFEST_DIR").to_owned() + &format!("/data/tests/text_01_k{}.enc", i);
        let ciphertext =
            fs::read(encrypted_file).expect("Could not find ciphertext in data/tests directory.");

        assert_eq!(encrypted_message, ciphertext);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_encrypt_is_unique() {
    let message_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01.txt");
    let message =
        fs::read(message_file).expect("Could not find plaintext in data/tests directory.");

    let mut set: std::collections::HashSet<std::vec::Vec<u8>> = std::collections::HashSet::new();

    for i in 0..32 {
        let (mut s_automaton, mut t_automaton) = talos::encrypt::get_transpose_shift_automata(i);
        let encrypted_message = talos::encrypt::encrypt_message_256(
            message.clone(),
            &mut s_automaton,
            &mut t_automaton,
        );

        assert!(!set.contains(&encrypted_message));
        set.insert(encrypted_message);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_decrypt_is_unique() {
    let message_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01.txt");
    let message =
        fs::read(message_file).expect("Could not find plaintext in data/tests directory.");
    let message_size = message.len();

    let encrypted_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01_k0.enc");
    let ciphertext =
        fs::read(encrypted_file).expect("Could not find plaintext in data/tests directory.");

    let mut set: std::collections::HashSet<std::vec::Vec<u8>> = std::collections::HashSet::new();

    for i in 1..32 {
        let (mut s_automaton, mut t_automaton) = talos::encrypt::get_transpose_shift_automata(i);
        let plaintext = talos::encrypt::decrypt_message_256(
            ciphertext.clone(),
            &mut s_automaton,
            &mut t_automaton,
        );

        assert_ne!(message, plaintext[..message_size]);
        assert!(!set.contains(&plaintext));
        set.insert(plaintext);
    }
}
