use std::{fs, io::Error, time::Duration};

use sudoku_solver::{
    printable::Printable,
    solvable::{
        ByColumns, ByPossibilities, ByRows, BySinglePossibilitiesColumns,
        BySinglePossibilitiesRows, Solvable,
    },
    sudoku::{Field, Grid},
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
    solve_with(grid, ByPossibilities {});
    solve_with(grid, ByColumns {});
    solve_with(grid, ByRows {});
    solve_with(grid, BySinglePossibilitiesRows {});
    solve_with(grid, BySinglePossibilitiesColumns {});

    if grid.is_solved() {
        return grid;
    }

    println!("Grid not solved");
    grid.print();
    println!();
    std::thread::sleep(Duration::new(1, 0));

    return solve(grid);
}

fn solve_with<'a>(grid: &'a mut Grid, solver_solvable: impl Solvable) -> &mut Grid {
    grid.update_possibilities_in_rows();
    grid.update_possibilities_in_columns();
    grid.update_possibilities_in_boxes();
    solver_solvable.solve(grid);

    grid
}

fn create_grid() -> Result<Grid, Error> {
    let file_content = fs::read_to_string("./grid7.txt")?;

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
