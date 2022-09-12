use std::{fs, io::Error};

use sudoku_solver::{
    solver::{ByColumns, ByRows, Solvable},
    sudoku::{Field, Grid, Row},
};

fn main() {
    let grid = match create_grid() {
        Ok(grid) => grid,
        Err(err) => panic!("Cannot create grid: {}.", err),
    };

    grid.print();

    println!();

    let new_grid = ByRows::solve(&grid);
    new_grid.print();
    println!();

    let new_grid = ByColumns::solve(&grid);
    new_grid.print();
}

fn create_grid() -> Result<Grid, Error> {
    let file_content = fs::read_to_string("./grid3.txt")?;
    let mut rows: Vec<Row> = vec![];
    for line in file_content.lines().into_iter() {
        let mut fields = vec![];
        for s in line.split_whitespace().into_iter() {
            fields.push(match s.parse() {
                Ok(v) => Field::new(v),
                Err(_) => Field::empty(),
            });
        }

        rows.push(Row::new(fields));
    }

    Ok(Grid::new(rows))
}
