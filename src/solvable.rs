use crate::sudoku::{Field, Grid};

pub trait Solvable {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid;
}

pub struct ByRows {}

impl Solvable for ByRows {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid {
        for row in 0..=8 {
            let cloned_grid = grid.clone();
            let fields = match cloned_grid.get_fields_in_row(row) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let missing_digit = match determine_missing_digit_in_fields(&fields) {
                Some(d) => d,
                None => continue,
            };

            for (column, field) in fields.iter().enumerate() {
                if field.is_empty() {
                    grid.set_field(row, column, Field::new(missing_digit));
                }
            }
        }

        grid
    }
}

fn determine_missing_digit_in_fields(fields: &Vec<&Field>) -> Option<i32> {
    let used_digits: Vec<i32> = fields
        .iter()
        .map(|f| f.value().unwrap_or(0))
        .filter(|f| f.gt(&0))
        .collect();

    if used_digits.len() != 8 {
        return Option::None;
    }

    for digit in 1..=9 {
        if !used_digits.contains(&digit) {
            return Option::Some(digit);
        }
    }

    return Option::None;
}
pub struct ByColumns {}

impl Solvable for ByColumns {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid {
        for column in 0..=8 {
            let cloned_grid = grid.clone();
            let fields = match cloned_grid.get_fields_in_column(column) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let missing_digit = match determine_missing_digit_in_fields(&fields) {
                Some(d) => d,
                None => continue,
            };

            for (row, field) in fields.iter().enumerate() {
                if field.is_empty() {
                    grid.set_field(row, column, Field::new(missing_digit));
                }
            }
        }

        grid
    }
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
