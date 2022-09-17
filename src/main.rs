use std::{fs, io::Error};

use sudoku_solver::solver::Solver;
use sudoku_solver::{
    printable::Printable,
    sudoku::{Field, Grid},
};

fn main() {
    let mut grid = match create_grid() {
        Ok(grid) => grid,
        Err(err) => panic!("Cannot create grid: {}.", err),
    };

    let solver = Solver::new(true, true);

    let solved_grid = solver.solve(&mut grid);

    solved_grid.print();
}

fn create_grid() -> Result<Grid, Error> {
    let file_content = fs::read_to_string("./grid8.txt")?;

    let mut grid = Grid::create_empty();
    for (row, line) in file_content.lines().into_iter().enumerate() {
        for (col, s) in line.split_whitespace().into_iter().enumerate() {
            grid.set_field(
                row,
                col,
                match s.parse() {
                    Ok(v) => Field::new(v),
                    Err(_) => Field::empty(),
                },
            );
        }
    }

    return Ok(grid);
}
