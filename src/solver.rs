use std::time::Duration;

use crate::{
    printable::Printable,
    solvable::{
        BySinglePossibilitiesBoxes, BySinglePossibilitiesColumns, BySinglePossibilitiesRows,
        Solvable,
    },
    sudoku::{Field, Grid},
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
        grid.update_possibilities();

        let mut changed_fields = vec![];
        changed_fields.append(&mut solve_with(grid, BySinglePossibilitiesRows {}));
        changed_fields.append(&mut solve_with(grid, BySinglePossibilitiesColumns {}));
        changed_fields.append(&mut solve_with(grid, BySinglePossibilitiesBoxes {}));

        if changed_fields.is_empty() {
            println!("No changes in grid detected. Stopping solving.");

            return grid;
        }

        grid.set_fields(changed_fields);

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

fn solve_with(grid: &mut Grid, solver_solvable: impl Solvable) -> Vec<Field> {
    solver_solvable.solve(grid)
}
