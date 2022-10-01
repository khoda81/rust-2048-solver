use rust_2048_solver::{board::Direction, game::Game};

fn main() {
    let mut game = Game::<4, 4>::new();

    loop {
        // println!("{:?}", game);
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

        let direction: Direction = rand::random();
        println!("{:?}", direction);
        if game.step(direction) {
            break;
        }
    }

    println!("{:?}", game);
}
