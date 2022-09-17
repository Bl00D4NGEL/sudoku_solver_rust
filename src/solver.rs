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
    should_sleep: bool,
    should_print: bool,
}

impl Solver {
    pub fn new(should_sleep: bool, should_print: bool) -> Solver {
        Solver {
            should_print,
            should_sleep,
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
            println!("Grid not solved");
            grid.print();
            println!();
        }
        if self.should_sleep {
            std::thread::sleep(Duration::new(1, 0));
        }

        return self.solve(grid);
    }
}

fn solve_with<'a>(grid: &'a mut Grid, solver_solvable: impl Solvable) -> &mut Grid {
    grid.update_possibilities_in_rows();
    grid.update_possibilities_in_columns();
    grid.update_possibilities_in_boxes();
    solver_solvable.solve(grid);

    grid
}
