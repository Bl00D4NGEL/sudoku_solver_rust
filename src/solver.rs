use std::time::Duration;

use crate::{
    printable::Printable,
    solvable::{
        ByBoxes, ByColumns, ByPossibilities, ByRows, BySinglePossibilitiesBoxes,
        BySinglePossibilitiesColumns, BySinglePossibilitiesRows, Solvable,
    },
    sudoku::Grid,
};

pub struct Solver {
    sleep_duration: Duration,
    should_print: bool,
}

impl Solver {
    pub fn new(sleep_duration: Duration, should_print: bool) -> Solver {
        Solver {
            should_print,
            sleep_duration,
        }
    }

    pub fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid {
        solve_with(grid, ByPossibilities {});
        solve_with(grid, ByColumns {});
        solve_with(grid, ByRows {});
        solve_with(grid, ByBoxes {});
        solve_with(grid, BySinglePossibilitiesRows {});
        solve_with(grid, BySinglePossibilitiesColumns {});
        solve_with(grid, BySinglePossibilitiesBoxes {});

        if grid.is_solved() {
            return grid;
        }

        if self.should_print {
            grid.print();
        }
        if !self.sleep_duration.is_zero() {
            std::thread::sleep(self.sleep_duration);
        }

        self.solve(grid)
    }
}

fn solve_with(grid: &mut Grid, solver_solvable: impl Solvable) -> &mut Grid {
    grid.update_possibilities();
    solver_solvable.solve(grid);

    grid
}
