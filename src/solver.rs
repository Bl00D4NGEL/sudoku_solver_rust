use crate::sudoku::{Field, Grid, Row};

pub trait Solvable {
    fn solve<'a>(&self, grid: &'a Grid) -> Grid;
}

pub struct ByRows {}

impl Solvable for ByRows {
    fn solve<'a>(&self, grid: &'a Grid) -> Grid {
        Grid::new(grid.rows().into_iter().map(|r| solve_by_row(r)).collect())
    }
}
pub struct ByColumns {}

impl Solvable for ByColumns {
    fn solve<'a>(&self, grid: &'a Grid) -> Grid {
        Grid::new(
            grid.columns()
                .into_iter()
                .map(|r| solve_by_row(&r))
                .collect(),
        )
    }
}

fn solve_by_row(row: &Row) -> Row {
    if row.empty_fields().len() != 1 {
        return row.clone();
    }

    let mut mutable_row = row.clone();

    mutable_row.update_possibilities();

    let new_fields = mutable_row
        .fields()
        .clone()
        .into_iter()
        .map(|f| {
            if !f.is_empty() {
                return f;
            }

            if f.possibilities().len() == 1 {
                return Field::new(f.possibilities()[0]);
            }

            return f;
        })
        .collect();

    return Row::new(new_fields);
}
