use sudoku_solver::{
    run,
    sudoku::{Field, Row},
};

fn main() {
    let fields = [
        Field::new(2),
        Field::new(3),
        Field::new(4),
        Field::new(5),
        Field::new(6),
        Field::new(7),
        Field::empty(Option::None),
        Field::new(8),
        Field::new(9),
    ];
    let row = Row::new(fields);
    let mutated_row = run(row);

    dbg!(mutated_row);
}
