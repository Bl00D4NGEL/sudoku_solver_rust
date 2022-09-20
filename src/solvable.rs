use std::{collections::HashMap, vec};

use crate::sudoku::{Field, Grid};

pub trait Solvable {
    fn solve<'a>(&self, grid: &'a mut Grid) -> Vec<Field>;
}

pub struct BySinglePossibilitiesRows {}

impl Solvable for BySinglePossibilitiesRows {
    fn solve<'a>(&self, grid: &'a mut Grid) -> Vec<Field> {
        let mut fields_to_update = vec![];
        for row in 0..=8 {
            let fields = grid.get_fields_in_row(row);

            fields_to_update.append(&mut set_field_value_if_only_possibility(fields));
        }

        fields_to_update
    }
}

fn set_field_value_if_only_possibility(fields: [&Field; 9]) -> Vec<Field> {
    let possibilities = determine_possibilities_in_field_set(&fields);

    let removable_possibilites: Vec<i32> = possibilities
        .iter()
        .filter(|(_, count)| count.eq(&&1))
        .map(|(possibility, _)| **possibility)
        .collect();

    if removable_possibilites.is_empty() {
        return vec![];
    }

    let mut fields_to_update = vec![];
    for possibility in removable_possibilites.iter() {
        for field in fields.iter() {
            if field.possibilities().contains(&possibility) {
                fields_to_update.push(Field::new(*possibility, field.index()));
            }
        }
    }

    fields_to_update
}

fn determine_possibilities_in_field_set<'a>(fields: &'a [&Field; 9]) -> HashMap<&'a i32, i32> {
    let mut possibility_map = HashMap::new();

    for field in fields.iter() {
        for possibility in field.possibilities() {
            possibility_map
                .entry(possibility)
                .and_modify(|p| *p += 1)
                .or_insert(1);
        }
    }

    possibility_map
}

pub struct BySinglePossibilitiesColumns {}

impl Solvable for BySinglePossibilitiesColumns {
    fn solve<'a>(&self, grid: &'a mut Grid) -> Vec<Field> {
        let mut fields_to_update = vec![];
        for column in 0..=8 {
            let fields = grid.get_fields_in_column(column);

            fields_to_update.append(&mut set_field_value_if_only_possibility(fields));
        }

        fields_to_update
    }
}
pub struct BySinglePossibilitiesBoxes {}

impl Solvable for BySinglePossibilitiesBoxes {
    fn solve<'a>(&self, grid: &'a mut Grid) -> Vec<Field> {
        let mut fields_to_update = vec![];
        for box_id in 0..=8 {
            let fields = grid.get_fields_in_box(box_id);

            fields_to_update.append(&mut set_field_value_if_only_possibility(fields));
        }

        fields_to_update
    }
}
