#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{env, fs, path::PathBuf};

use eframe::egui;
use solver::{SolveByX, SolveByY, SudokuSolver};

mod solver;
mod ui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };

    let mut args = env::args();

    let grid_path = PathBuf::from(args.nth(1).unwrap());

    let grid = SudokuGrid::from(grid_path.clone());
    let mut solver = SudokuSolver::new(grid);
    solver.add_solving_strategy(Box::new(SolveByX {}));
    solver.add_solving_strategy(Box::new(SolveByY {}));

    eframe::run_native("Sudoku solver", options, Box::new(|_| Box::new(solver)))
}

#[derive(Default, Clone, Debug)]
pub struct SudokuGrid {
    rows: Vec<Vec<Field>>,
}

#[derive(Default, Clone, Debug)]
pub struct Field {
    value: Option<usize>,
    possibilities: Vec<usize>,
}

impl From<PathBuf> for SudokuGrid {
    fn from(value: PathBuf) -> Self {
        let contents = &fs::read_to_string(value).unwrap();
        let lines = contents
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();

        if lines.len() != 9 {
            panic!("File has more than 9 lines, can not create valid sudoku");
        }

        let rows: Vec<Vec<Field>> = lines
            .iter()
            .map(|line| {
                let fields = line
                    .trim()
                    .split(' ')
                    .map(|c| match c {
                        "1" => Field::filled(1),
                        "2" => Field::filled(2),
                        "3" => Field::filled(3),
                        "4" => Field::filled(4),
                        "5" => Field::filled(5),
                        "6" => Field::filled(6),
                        "7" => Field::filled(7),
                        "8" => Field::filled(8),
                        "9" => Field::filled(9),
                        _ => Field::empty(),
                    })
                    .collect::<Vec<Field>>();

                if fields.len() != 9 {
                    panic!("Row has more than 9 fields, can not create valid sudoku");
                }

                fields
            })
            .collect();
        SudokuGrid { rows }
    }
}

impl SudokuGrid {
    fn get_field(&self, row_idx: usize, col_idx: usize) -> Option<&Field> {
        let row = self.rows.get(row_idx)?;

        row.get(col_idx)
    }

    fn get_field_mut(&mut self, row_idx: usize, col_idx: usize) -> Option<&mut Field> {
        let row = self.rows.get_mut(row_idx)?;

        row.get_mut(col_idx)
    }

    fn update_possibities_for_all_fields(&mut self) {
        for row_idx in 0..9 {
            for col_idx in 0..9 {
                self.update_possibility_for_field_at(row_idx, col_idx);
            }
        }
    }

    fn is_completed(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.iter().all(|f| f.value.is_some()))
    }

    fn get_fields_in_row(&self, row_idx: usize) -> Option<&Vec<Field>> {
        self.rows.get(row_idx)
    }

    fn get_fields_in_column(&self, col_idx: usize) -> Vec<&Field> {
        let mut fields = vec![];

        for i in 0..9 {
            match self.rows.get(i) {
                None => {}
                Some(row) => match row.get(col_idx) {
                    None => {}
                    Some(field) => fields.push(field),
                },
            }
        }

        (0..9)
            .filter_map(|i| self.rows.get(i)?.get(col_idx))
            .collect()
    }

    fn get_box_id_for_row_and_column(row: usize, column: usize) -> Option<usize> {
        match (row, column) {
            (0..=2, 0..=2) => Some(0),
            (0..=2, 3..=5) => Some(1),
            (0..=2, 6..=8) => Some(2),
            (3..=5, 0..=2) => Some(3),
            (3..=5, 3..=5) => Some(4),
            (3..=5, 6..=8) => Some(5),
            (6..=8, 0..=2) => Some(6),
            (6..=8, 3..=5) => Some(7),
            (6..=8, 6..=8) => Some(8),
            _ => None,
        }
    }

    fn get_fields_in_box(&self, box_id: usize) -> Vec<&Field> {
        self.get_fields_in_box_with_positions(box_id)
            .into_iter()
            .map(|(f, _)| f)
            .collect()
    }

    fn get_fields_in_box_with_positions(&self, box_id: usize) -> Vec<(&Field, (usize, usize))> {
        let indexes: Vec<(usize, usize)> = match box_id {
            0..=2 => vec![
                (0, box_id * 3),
                (0, box_id * 3 + 1),
                (0, box_id * 3 + 2),
                (1, box_id * 3),
                (1, box_id * 3 + 1),
                (1, box_id * 3 + 2),
                (2, box_id * 3),
                (2, box_id * 3 + 1),
                (2, box_id * 3 + 2),
            ],
            3..=5 => vec![
                (3, (box_id % 3) * 3),
                (3, (box_id % 3) * 3 + 1),
                (3, (box_id % 3) * 3 + 2),
                (4, (box_id % 3) * 3),
                (4, (box_id % 3) * 3 + 1),
                (4, (box_id % 3) * 3 + 2),
                (5, (box_id % 3) * 3),
                (5, (box_id % 3) * 3 + 1),
                (5, (box_id % 3) * 3 + 2),
            ],
            6..=8 => vec![
                (6, (box_id % 3) * 3),
                (6, (box_id % 3) * 3 + 1),
                (6, (box_id % 3) * 3 + 2),
                (7, (box_id % 3) * 3),
                (7, (box_id % 3) * 3 + 1),
                (7, (box_id % 3) * 3 + 2),
                (8, (box_id % 3) * 3),
                (8, (box_id % 3) * 3 + 1),
                (8, (box_id % 3) * 3 + 2),
            ],
            _ => vec![],
        };

        indexes
            .into_iter()
            .filter_map(|(row, col)| self.get_field(row, col).map(|field| (field, (row, col))))
            .collect()
    }

    fn update_possibility_for_field_at(&mut self, row_idx: usize, col_idx: usize) {
        if self.get_field_mut(row_idx, col_idx).is_none() {
            return;
        }

        let row_field_values: Vec<usize> =
            self.get_fields_in_row(row_idx).map_or(vec![], |fields| {
                fields.iter().filter_map(|f| f.value).collect()
            });

        let column_field_values: Vec<usize> = self
            .get_fields_in_column(col_idx)
            .iter()
            .filter_map(|f| f.value)
            .collect();

        let box_id = Self::get_box_id_for_row_and_column(row_idx, col_idx);

        let box_values: Vec<usize> = match box_id {
            None => vec![],
            Some(id) => self
                .get_fields_in_box(id)
                .iter()
                .filter_map(|f| f.value)
                .collect(),
        };

        let mut values = [box_values, row_field_values, column_field_values]
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<usize>>();

        values.dedup();

        let all_row_possibilities = if let Some(row_fields) = self.get_fields_in_row(row_idx) {
            Self::count_possibilities_for_fields(row_fields.iter().collect())
        } else {
            [0; 10]
        };

        let all_column_possibilities =
            Self::count_possibilities_for_fields(self.get_fields_in_column(col_idx));

        let all_box_possibilities =
            if let Some(box_id) = Self::get_box_id_for_row_and_column(row_idx, col_idx) {
                Self::count_possibilities_for_fields(self.get_fields_in_box(box_id))
            } else {
                [0; 10]
            };

        if let Some(field) = self.get_field_mut(row_idx, col_idx) {
            field.possibilities.retain(|p| !values.contains(p));
            if field.possibilities.len() == 1 {
                *field = Field::filled(*field.possibilities.first().unwrap());
                return;
            }

            for possibility in field.possibilities.clone().iter() {
                if let Some(p) = all_row_possibilities.get(*possibility) {
                    if *p == 1 {
                        *field = Field::filled(*possibility);
                        println!("[row] {possibility} was never found, assuming {row_idx} / {col_idx} is the only place it can go");
                    }
                }
                if let Some(p) = all_column_possibilities.get(*possibility) {
                    if *p == 1 {
                        *field = Field::filled(*possibility);
                        println!("[column] {possibility} was never found, assuming {row_idx} / {col_idx} is the only place it can go");
                    }
                }
                if let Some(p) = all_box_possibilities.get(*possibility) {
                    if *p == 1 {
                        *field = Field::filled(*possibility);
                        println!("[box] {possibility} was never found, assuming {row_idx} / {col_idx} is the only place it can go");
                    }
                }
            }
        }
    }

    fn count_possibilities_for_fields(fields: Vec<&Field>) -> [usize; 10] {
        let mut all_possibilities = [0; 10];

        for field in fields.iter() {
            for possibility in field.possibilities.iter() {
                if let Some(p) = all_possibilities.get_mut(*possibility) {
                    *p += 1;
                }
            }
        }

        all_possibilities
    }
}

impl Field {
    fn empty() -> Self {
        Self {
            value: None,
            possibilities: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }

    fn filled(value: usize) -> Self {
        Self {
            value: Some(value),
            possibilities: vec![],
        }
    }

    fn remove_possibility(&mut self, possibility: usize) {
        self.possibilities.retain(|p| *p != possibility)
    }

    fn without_possibility(&self, possibility: usize) -> Self {
        Self {
            value: None,
            possibilities: self
                .possibilities
                .iter()
                .filter(|p| **p != possibility)
                .copied()
                .collect(),
        }
    }
}
