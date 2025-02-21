// 2025 Steven Chiacchira
use cipher::{automata, matrix, parse};
use clap::Parser;
use rand::random;
use std::collections::{hash_map::HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Flag for testing only contiguous seeds.
    #[arg(short, long, action)]
    use_contiguous_seeds: bool,

    /// The number of seeds to test.
    #[arg(short, long, default_value_t = 1)]
    seeds: u32,

    /// The number of generations to run the [Automaton](automata::Automaton) for.
    #[arg(short, long, default_value_t = 32_000)]
    generations: u32,

    /// File to use for initializing the [Automaton](automata::Automaton) state.
    #[arg(short, long)]
    init_file: String,
}

fn main() {
    let args = Args::parse();

    let mut global_used_states: HashSet<Vec<u32>, _> = HashSet::new();
    let mut global_duplicates: Vec<Vec<u32>> = Vec::new();

    let seed_gen = (0..args.seeds).map(if args.use_contiguous_seeds {
        |i| i
    } else {
        |_| random::<u32>()
    });

    println!("# Using contiguous seeds: {}", args.use_contiguous_seeds);
    println!("# Number of seeds: {}", args.seeds);
    println!("# Number of generations: {}", args.generations);
    println!("# Initial File: {}", &args.init_file);
    println!("test\tn_generations\tseed\tavg_alive\tcontains_global_duplicate");

    for (test, seed) in seed_gen.enumerate() {
        let mut char_map: HashMap<char, bool> = parse::gen_char_map(seed);
        char_map.insert('#', true);
        char_map.insert('.', false);
        let mut local_used_states: HashSet<Vec<u32>, _> = HashSet::new();
        let mut n_local_alive_total = 0;

        let table =
            parse::parse_bool_table(&read_to_string(&args.init_file).unwrap(), &char_map).unwrap();
        let state = matrix::ToroidalBitMatrix::new(table).unwrap();
        let rule = automata::AutomatonRule {
            born: [false, false, true, true, true, true, true, false, false],
            dies: [true, true, false, false, false, true, true, true, true],
        };

        let mut automaton = automata::Automaton::new(state, rule);

        let mut final_generation = args.generations;
        let mut contains_global_duplicate = false;

        for generation in 0..args.generations {
            let n_alive = automaton.popcount();
            n_local_alive_total += n_alive;

            let curr_state = automaton.get_storage();

            if global_used_states.contains(&curr_state) {
                global_duplicates.push(curr_state);
                contains_global_duplicate = true;
                final_generation = generation;
                break;
            } else if local_used_states.contains(&curr_state) {
                final_generation = generation;
                break;
            }
            local_used_states.insert(curr_state.clone());
            global_used_states.insert(curr_state);
            automaton.iter_rule(1);
        }

        let avg_alive: f64 =
            (n_local_alive_total as f64) / (16.0 * 16.0 * (final_generation as f64 + 1.0));

        println!(
            "{}\t{}\t{}\t{}\t{}",
            test, final_generation, seed, avg_alive, contains_global_duplicate
        );
    }
}
