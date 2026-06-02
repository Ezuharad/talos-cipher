// 2026 Steven Chiacchira
#![no_main]

use libfuzzer_sys::fuzz_target;
use talos::encrypt::{self, decrypt_message_256};

fuzz_target!(|data: &[u8]| {
    let (mut shift_automata, mut transpose_automata) = encrypt::get_shift_transpose_automata(0);

    let _ =
        encrypt::encrypt_message_256(data.to_vec(), &mut shift_automata, &mut transpose_automata);
    let _ = decrypt_message_256(data.to_vec(), &mut shift_automata, &mut transpose_automata);
});
