use criterion::{criterion_group, BenchmarkId, Criterion, Throughput};
use rand::seq::SliceRandom;
use rust_2048_solver::game::twenty_forty_eight::State;
use rust_2048_solver::game::{Discrete, GameState, Outcome};
use std::hash::{self, Hash as _};

fn generate_states(count: usize) -> Vec<State<4, 4>> {
    #[rustfmt::skip]
    let starting_state = State::from_cells([
        [3, 3, 1, 1],
        [1, 0, 5, 0],
        [0, 2, 7, 4],
        [6, 1, 6, 8],
    ]);

    let mut states = vec![starting_state];
    let mut rng = rand::thread_rng();
    while states.len() < count {
        let state = states.choose(&mut rng).unwrap().clone();

        for action in <State<4, 4> as GameState>::Action::iter() {
            let (_reward, outcome) = state.clone().outcome(action);
            states.extend(outcome.into_iter().map(|weighted| weighted.value));
        }
    }

    states.truncate(count);

    states
}

fn bench_hash(c: &mut Criterion) {
    const K: usize = 1024;
    let mut group = c.benchmark_group("hash states");
    let mut hasher = hash::DefaultHasher::new();

    #[allow(clippy::identity_op)]
    for num_states in [1, 2, 8, 16].map(|n| n * K) {
        let states = generate_states(num_states);

        group.throughput(Throughput::Elements(states.len().try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::new("hash_states", num_states),
            &states,
            |b, states| {
                b.iter(|| states.iter().for_each(|state| state.hash(&mut hasher)));
            },
        );
    }
}

fn bench_board_swipe(c: &mut Criterion) {
    let mut state = State::<4, 4>::new();

    c.bench_function("updates", |b| {
        b.iter(|| {
            for action in <State<4, 4> as GameState>::Action::iter() {
                let (_reward, outcome) = state.clone().outcome(action);
                state = outcome.collapse();
            }

            if state.is_terminal() {
                state = State::new();
            }
        })
    });
}

criterion_group!(
    name = board;
    config = Criterion::default()
        .significance_level(0.01);

    targets = bench_hash, bench_board_swipe
);
