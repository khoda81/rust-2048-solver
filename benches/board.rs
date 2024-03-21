use criterion::BenchmarkId;
use criterion::{criterion_group, Criterion, Throughput};
use rand::seq::SliceRandom;
use rust_2048_solver::{
    board::{Direction, StateOf2048},
    game,
};
use std::hash::{self, Hash as _};

type State = StateOf2048<4, 4>;

fn generate_states(count: usize) -> Vec<State> {
    let starting_state = State::from([[3, 3, 1, 1], [1, 0, 5, 0], [0, 2, 7, 4], [6, 1, 6, 8]]);

    let mut states = vec![starting_state];
    let mut rng = rand::thread_rng();
    while states.len() < count {
        let state = *states.choose(&mut rng).unwrap();
        let game = game::Swipe2048::from(state);
        let iter = game
            .transitions()
            .flat_map(|transition| transition.next.spawns())
            .map(|(new_state, _)| new_state);

        states.extend(iter);
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
    let mut board = State::new();

    c.bench_function("updates", |b| {
        b.iter(|| {
            board.swipe(Direction::Up);
            board.swipe(Direction::Right);
            board.swipe(Direction::Down);
            board.swipe(Direction::Left);

            if board.is_lost() {
                board = StateOf2048::new();
            }
        })
    });
}

criterion_group!(board, bench_hash, bench_board_swipe);
