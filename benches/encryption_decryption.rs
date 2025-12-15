// 2025 Steven Chiacchira
use std::hint::black_box;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use talos::encrypt::{decrypt_message_256, encrypt_message_256, get_shift_transpose_automata};

#[must_use]
pub fn generate_message(n_elements: usize) -> Vec<u8> {
    let mut result = vec![0; n_elements];

    for element in result.iter_mut() {
        *element = rand::random();
    }

    result
}

criterion_group!(benches, encryption_decryption_in_memory);
criterion_main!(benches);

const MESSAGE_LEN: usize = 10_000;

fn encryption_decryption_in_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("Encrypt and Decrypt in Memory");
    group.measurement_time(Duration::from_secs(6));

    let message: Vec<u8> = generate_message(MESSAGE_LEN);
    let key: u32 = rand::random();

    group.bench_function("encrypt_message_256() 20_000", |b| {
        b.iter_batched(
            || {
                let message_clone = message.clone();
                let (s_automaton, t_automaton) = get_shift_transpose_automata(key);
                (message_clone, s_automaton, t_automaton)
            },
            |(message_clone, mut s_automaton, mut t_automaton)| {
                let ciphertext =
                    encrypt_message_256(message_clone, &mut s_automaton, &mut t_automaton);
                black_box(ciphertext);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("decrypt_message_256() 20_000", |b| {
        b.iter_batched(
            || {
                let message_clone = message.clone();
                let (s_automaton, t_automaton) = get_shift_transpose_automata(key);
                (message_clone, s_automaton, t_automaton)
            },
            |(message_clone, mut s_automaton, mut t_automaton)| {
                let plaintext =
                    decrypt_message_256(message_clone, &mut s_automaton, &mut t_automaton);
                black_box(plaintext);
            },
            BatchSize::SmallInput,
        );
    });
}
