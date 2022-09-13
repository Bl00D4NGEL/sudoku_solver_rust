use std::collections::HashMap;

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

pub struct BySinglePossibilitiesRows {}

impl Solvable for BySinglePossibilitiesRows {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid {
        let mut fields_to_update = vec![];
        for row in 0..=8 {
            let fields = match grid.get_fields_in_row(row) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let possibilities = determine_single_possibility_in_field_set(&fields);

            for (col, field) in possibilities {
                fields_to_update.push((row, col, field));
            }
        }

        for (row, col, field) in fields_to_update {
            grid.set_field(row, col, field);
        }

        grid
    }
}

pub struct BySinglePossibilitiesColumns {}

impl Solvable for BySinglePossibilitiesColumns {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid {
        let mut fields_to_update = vec![];
        for col in 0..=8 {
            let fields = match grid.get_fields_in_column(col) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let possibilities = determine_single_possibility_in_field_set(&fields);

            for (row, field) in possibilities {
                fields_to_update.push((row, col, field));
            }
        }

        for (row, col, field) in fields_to_update {
            grid.set_field(row, col, field);
        }

        grid
    }
}

fn determine_single_possibility_in_field_set(fields: &Vec<&Field>) -> Vec<(usize, Field)> {
    let mut fields_to_update = vec![];
    let mut possibility_map = HashMap::new();

    for field in fields {
        for possibility in field.possibilities() {
            possibility_map
                .entry(possibility)
                .and_modify(|p| *p += 1)
                .or_insert(1);
        }
    }

    for (possibility, count) in possibility_map.iter() {
        if !count.eq(&1) {
            continue;
        }
        for (index, field) in (&fields).into_iter().enumerate() {
            if field.possibilities().contains(possibility) {
                fields_to_update.push((index, Field::new((*possibility).clone())));
            }
        }
    }

    fields_to_update
}
