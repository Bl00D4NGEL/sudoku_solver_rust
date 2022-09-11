use sudoku_solver::{
    run,
    sudoku::{Field, Grid, Row},
};

fn main() {
    let grid = create_grid();
    grid.print();

    println!();
    let new_grid = Grid::new(grid.rows().clone().map(|r| run(r)));
    new_grid.print();
}

fn create_grid() -> Grid {
    let fields = Row::new([
        Field::empty(Option::None),
        Field::new(2),
        Field::new(3),
        Field::new(4),
        Field::new(5),
        Field::new(6),
        Field::new(7),
        Field::new(8),
        Field::new(9),
    ]);
    let fields2 = Row::new([
        Field::new(9),
        Field::new(1),
        Field::empty(Option::None),
        Field::new(3),
        Field::new(4),
        Field::new(5),
        Field::new(6),
        Field::new(7),
        Field::new(8),
    ]);
    let fields3 = Row::new([
        Field::new(8),
        Field::new(9),
        Field::new(1),
        Field::new(2),
        Field::empty(Option::None),
        Field::new(4),
        Field::new(5),
        Field::new(6),
        Field::new(7),
    ]);
    let fields4 = Row::new([
        Field::new(7),
        Field::new(8),
        Field::new(9),
        Field::new(1),
        Field::new(2),
        Field::new(3),
        Field::empty(Option::None),
        Field::new(5),
        Field::new(6),
    ]);
    let fields5 = Row::new([
        Field::new(6),
        Field::new(7),
        Field::new(8),
        Field::new(9),
        Field::new(1),
        Field::new(2),
        Field::new(3),
        Field::new(4),
        Field::empty(Option::None),
    ]);
    let fields6 = Row::new([
        Field::new(5),
        Field::empty(Option::None),
        Field::new(7),
        Field::new(8),
        Field::new(9),
        Field::new(1),
        Field::new(2),
        Field::new(3),
        Field::new(4),
    ]);
    let fields7 = Row::new([
        Field::new(4),
        Field::new(5),
        Field::new(6),
        Field::empty(Option::None),
        Field::new(8),
        Field::new(9),
        Field::new(1),
        Field::new(2),
        Field::new(3),
    ]);
    let fields8 = Row::new([
        Field::new(3),
        Field::new(4),
        Field::new(5),
        Field::new(6),
        Field::new(7),
        Field::empty(Option::None),
        Field::new(9),
        Field::new(1),
        Field::new(2),
    ]);
    let fields9 = Row::new([
        Field::new(2),
        Field::new(3),
        Field::new(4),
        Field::new(5),
        Field::new(6),
        Field::new(7),
        Field::new(8),
        Field::empty(Option::None),
        Field::new(1),
    ]);

    Grid::new([
        fields, fields2, fields3, fields4, fields5, fields6, fields7, fields8, fields9,
    ])
}
