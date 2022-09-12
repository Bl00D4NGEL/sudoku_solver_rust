use std::{fs, io::Error};

use sudoku_solver::{
    solvable::{ByColumns, ByRows, Solvable},
    sudoku::{Field, Grid, Printable},
};

fn main() {
    let mut grid = match create_grid() {
        Ok(grid) => grid,
        Err(err) => panic!("Cannot create grid: {}.", err),
    };
    let solved_grid = solve(&mut grid);

    solved_grid.print();
}

fn solve(grid: &mut Grid) -> &mut Grid {
    solve_with(grid, ByRows {});
    solve_with(grid, ByColumns {});

    if grid.is_solved() {
        return grid;
    }

    return solve(grid);
}

fn solve_with<'a>(grid: &'a mut Grid, solver_solvable: impl Solvable) -> &mut Grid {
    solver_solvable.solve(grid)
}

fn create_grid() -> Result<Grid, Error> {
    let file_content = fs::read_to_string("./grid5.txt")?;

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
