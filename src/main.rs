use sudoku_solver::solver::Solver;
use sudoku_solver::{printable::Printable, sudoku::Grid};

fn main() {
    let mut grid = match Grid::create_from_file("./grid8.txt") {
        Ok(grid) => grid,
        Err(err) => panic!("Cannot create grid: {}.", err),
    };

    let solver = Solver::new(true, true);

    let solved_grid = solver.solve(&mut grid);

    solved_grid.print();
}
