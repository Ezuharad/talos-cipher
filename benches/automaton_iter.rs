// 2025 Steven Chiacchira
use std::hint::black_box;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use talos::automata::ToroidalAutomaton;
use talos::encrypt::{AUTOMATA_RULE, N_COLS, N_ROWS};
use talos::matrix::{ToroidalBinaryMatrix, ToroidalBitMatrix, ToroidalBoolMatrix};

criterion_group!(benches, automata_black_box);
criterion_main!(benches);

#[must_use]
fn generate_bool_table(rows: usize, cols: usize) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; cols]; rows];

    for row in result.iter_mut() {
        for val in row.iter_mut() {
            *val = rand::random_bool(0.5);
        }
    }

    result
}

fn automata_black_box(c: &mut Criterion) {
    const N_ITERS: u32 = 10_000;

    let mut group = c.benchmark_group("Automata Black Box");
    group.measurement_time(Duration::from_secs(30));

    let table = generate_bool_table(N_ROWS, N_COLS);

    let mat_bool = black_box(ToroidalBoolMatrix::new(table.clone()).unwrap());
    let mat_u8 = black_box(ToroidalBitMatrix::<u8>::new(table.clone()).unwrap());
    let mat_u32 = black_box(ToroidalBitMatrix::<u32>::new(table.clone()).unwrap());

    let mut automaton_bool = ToroidalAutomaton::<ToroidalBoolMatrix>::new(mat_bool, AUTOMATA_RULE.clone());
    let mut automaton_u8 = ToroidalAutomaton::<ToroidalBitMatrix<u8>>::new(mat_u8, AUTOMATA_RULE.clone());
    let mut automaton_u32 =
        ToroidalAutomaton::<ToroidalBitMatrix<u32>>::new(mat_u32, AUTOMATA_RULE.clone());

    group.bench_function("Automaton<ToroidalBoolMatrix>.iter_rule(10_000)", |b| {
        b.iter(|| automaton_bool.iter_rule(N_ITERS))
    });
    group.bench_function("Automaton<ToroidalBitMatrix<u8>>.iter_rule(10_000)", |b| {
        b.iter(|| automaton_u8.iter_rule(N_ITERS))
    });
    group.bench_function("Automaton<ToroidalBitMatrix<u32>>.iter_rule(10_000)", |b| {
        b.iter(|| automaton_u32.iter_rule(N_ITERS))
    });
}
