#![feature(test)]

#[cfg(test)]
mod bench_game_of_2048 {
    extern crate test;

    use std::collections::HashSet;

    use rand::seq::SliceRandom;
    use rust_2048_solver::{
        board::{Direction, StateOf2048},
        game,
    };

    use test::{black_box, Bencher};

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

    #[bench]
    fn bench_hash(b: &mut Bencher) {
        let states = generate_states(1000);

        println!("Hashing {} states", states.len());
        b.iter(|| {
            let set = HashSet::<State>::from_iter(states.clone());
            black_box(set);
        });
    }

    #[bench]
    fn bench_board_swipe(b: &mut Bencher) {
        let mut board = State::new();

        b.iter(|| {
            (0..10_000).for_each(|_| {
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
}
