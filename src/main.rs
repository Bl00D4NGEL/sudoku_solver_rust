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

    let solved_grid = solve(&grid);
    solved_grid.print();
}

fn solve(grid: &Grid) -> Grid {
    let new_grid = solve_with(&grid, ByRows {});
    let new_grid = solve_with(&new_grid, ByColumns {});

    if new_grid.is_solved() {
        return new_grid;
    }

    return solve(&new_grid);
}

fn solve_with<'a>(grid: &'a Grid, solver_solvable: impl Solvable) -> Grid {
    solver_solvable.solve(grid)
}

fn create_grid() -> Result<Grid, Error> {
    let file_content = fs::read_to_string("./grid4.txt")?;
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
