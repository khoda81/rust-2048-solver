use std::time::{Duration, Instant};

use rust_2048_solver::{
    board::Direction,
    game::{Game, Solver},
};

fn main() {
    let mut game = Game::<4, 4>::new();
    let mut ai = Solver::new();

    loop {
        println!("{:?}", game);
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
        // input = input.trim().to_lowercase();

        // let direction = match input.as_str() {
        //     "w" => Direction::Up,
        //     "a" => Direction::Left,
        //     "s" => Direction::Down,
        //     "d" => Direction::Right,
        //     "q" => break,
        //     _ => continue,
        // };

        // let direction: Direction = rand::random();
        // println!("{:?}", direction);

        let timeout = Duration::from_millis(1000);
        let deadline = Instant::now() + timeout;
        let (_depth, _expected, direction): (u8, f64, Direction) = ai.get_timed(&game.board, deadline);

        if game.step(direction) {
            break;
        }
    }

    println!("{:?}", game);
}
