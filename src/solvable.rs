use std::{collections::HashMap, vec};

use crate::sudoku::{Field, Grid};

pub trait Solvable {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid;
}

fn update_fields_in_grid(fields: Vec<Field>, grid: &mut Grid) -> &mut Grid {
    fields.into_iter().for_each(|field| {
        grid.set_field(field);
    });

    grid
}

pub struct BySinglePossibilitiesRows {}

impl Solvable for BySinglePossibilitiesRows {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid {
        let mut fields_to_update = vec![];
        for row in 0..=8 {
            let fields = grid.get_fields_in_row(row);

            fields_to_update.append(&mut set_field_value_if_only_possibility(fields));
        }

        update_fields_in_grid(fields_to_update, grid)
    }
}

fn set_field_value_if_only_possibility(fields: Vec<&Field>) -> Vec<Field> {
    let possibilities = determine_possibilities_in_field_set(&fields);

    let mut fields_to_update = vec![];
    for field in fields.into_iter() {
        for (possibility, count) in possibilities.clone().into_iter() {
            if !count.eq(&1) {
                continue;
            }
            if field.possibilities().contains(&possibility) {
                fields_to_update.push(Field::new(possibility, field.index()));
            }
        }
    }

    fields_to_update
}
pub struct BySinglePossibilitiesColumns {}

impl Solvable for BySinglePossibilitiesColumns {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid {
        let mut fields_to_update = vec![];
        for column in 0..=8 {
            let fields = grid.get_fields_in_column(column);

            fields_to_update.append(&mut set_field_value_if_only_possibility(fields));
        }

        update_fields_in_grid(fields_to_update, grid)
    }
}
pub struct BySinglePossibilitiesBoxes {}

impl Solvable for BySinglePossibilitiesBoxes {
    fn solve<'a>(&self, grid: &'a mut Grid) -> &'a mut Grid {
        let mut fields_to_update = vec![];
        for box_id in 0..=8 {
            let fields = grid.get_fields_in_box(box_id);

            fields_to_update.append(&mut set_field_value_if_only_possibility(fields));
        }

        update_fields_in_grid(fields_to_update, grid)
    }
}

fn determine_possibilities_in_field_set(fields: &Vec<&Field>) -> HashMap<i32, i32> {
    let mut possibility_map = HashMap::new();

    for field in fields.iter() {
        for possibility in field.possibilities().clone() {
            possibility_map
                .entry(possibility)
                .and_modify(|p| *p += 1)
                .or_insert(1);
        }
    }

    possibility_map
}
