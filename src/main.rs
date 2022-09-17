use std::time::Duration;

use sudoku_solver::solver::Solver;
use sudoku_solver::{printable::Printable, sudoku::Grid};

fn main() {
    for i in 2..=10 {
        println!("Solving grid {}.", i);
        let mut grid = match Grid::create_from_file(format!("./grid{}.txt", i).as_str()) {
            Ok(grid) => grid,
            Err(err) => panic!("Cannot create grid: {}.", err),
        };

        let solver = Solver::new(Duration::from_millis(100), true);

        let solved_grid = solver.solve(&mut grid);

        solved_grid.print();
    }
}
