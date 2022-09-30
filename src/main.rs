use rust_2048_solver::game::shift_row;

fn main() {
    let mut row = [0, 0, 1, 1, 2, 3, 2, 1, 1, 3];
    println!("{:?}", row);
    
    let mut new_row = shift_row(&row);
    while new_row != row {
        row = new_row;
        println!("{:?}", row);
        new_row = shift_row(&row);
    }
}
