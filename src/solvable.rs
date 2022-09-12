use crate::sudoku::{Field, Grid};

pub trait Solvable {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid;
}

pub struct ByPossibilities {}

impl Solvable for ByPossibilities {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid {
        for (i, field) in grid.fields().clone().iter().enumerate() {
            if field.possibilities().len() == 1 {
                grid.set_field_by_index(i, Field::new(field.possibilities()[0]));
            }
        }

        grid
    }
}
