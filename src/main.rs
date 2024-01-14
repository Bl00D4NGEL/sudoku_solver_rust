#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::Color32;
use egui_extras::{Size, Strip, StripBuilder};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sudoku solver",
        options,
        Box::new(|_| {
            Box::new(SudokuGrid {
                rows: vec![
                    vec![
                        Field::empty(),
                        Field::filled(2),
                        Field::filled(9),
                        Field::filled(6),
                        Field::filled(8),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                    ],
                    vec![
                        Field::filled(6),
                        Field::filled(3),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::filled(9),
                        Field::empty(),
                    ],
                    vec![
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::filled(3),
                        Field::filled(5),
                        Field::empty(),
                        Field::empty(),
                    ],
                    vec![
                        Field::empty(),
                        Field::empty(),
                        Field::filled(8),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::filled(5),
                        Field::filled(1),
                    ],
                    vec![
                        Field::filled(2),
                        Field::filled(7),
                        Field::filled(5),
                        Field::empty(),
                        Field::filled(6),
                        Field::filled(9),
                        Field::empty(),
                        Field::filled(4),
                        Field::filled(3),
                    ],
                    vec![
                        Field::filled(1),
                        Field::filled(6),
                        Field::empty(),
                        Field::filled(5),
                        Field::filled(4),
                        Field::filled(8),
                        Field::empty(),
                        Field::filled(7),
                        Field::filled(9),
                    ],
                    vec![
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::filled(1),
                        Field::filled(6),
                        Field::filled(7),
                        Field::filled(8),
                        Field::empty(),
                    ],
                    vec![
                        Field::empty(),
                        Field::empty(),
                        Field::filled(2),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::empty(),
                        Field::filled(6),
                        Field::empty(),
                    ],
                    vec![
                        Field::filled(7),
                        Field::empty(),
                        Field::empty(),
                        Field::filled(3),
                        Field::filled(5),
                        Field::empty(),
                        Field::filled(9),
                        Field::empty(),
                        Field::empty(),
                    ],
                ],
            })
        }),
    )
}

#[derive(Default)]
pub struct SudokuGrid {
    rows: Vec<Vec<Field>>,
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

    fn update_possibility_for_field_at(&mut self, row_idx: usize, col_idx: usize) {
        if self.get_field_mut(row_idx, col_idx).is_none() {
            return;
        }

        // get fields of row_idx
        let row_field_values: Vec<usize> = self.rows.get(row_idx).map_or(vec![], |fields| {
            fields.iter().filter_map(|f| f.value).collect()
        });

        // get fields of col_idx
        let column_field_values: Vec<usize> = (0..9)
            .filter_map(|i| {
                self.rows
                    .get(i)
                    .and_then(|fields| match fields.get(col_idx) {
                        Some(f) => f.value,
                        None => None,
                    })
            })
            .collect();

        // get fields of box #
        let box_id: usize = match (row_idx, col_idx) {
            (0..=2, 0..=2) => 0,
            (0..=2, 3..=5) => 1,
            (0..=2, 6..=8) => 2,
            (3..=5, 0..=2) => 3,
            (3..=5, 3..=5) => 4,
            (3..=5, 6..=8) => 5,
            (6..=8, 0..=2) => 6,
            (6..=8, 3..=5) => 7,
            (6..=8, 6..=8) => 8,
            _ => panic!("Invalid row {row_idx} and column {col_idx} given"),
        };

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

        let box_values: Vec<usize> = indexes
            .into_iter()
            .filter_map(|(row, col)| self.get_field(row, col)?.value)
            .collect();

        let mut values = [box_values, row_field_values, column_field_values]
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<usize>>();

        values.dedup();

        if let Some(field) = self.get_field_mut(row_idx, col_idx) {
            field.possibilities.retain(|p| !values.contains(p));
            if field.possibilities.len() == 1 {
                *field = Field::filled(*field.possibilities.first().unwrap());
            }
        }
    }
}

pub struct Field {
    value: Option<usize>,
    possibilities: Vec<usize>,
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

impl eframe::App for SudokuGrid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

impl View for SudokuGrid {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut changes = vec![];
        self.update_possibities_for_all_fields();

        StripBuilder::new(ui)
            .size(Size::relative(1.0))
            .vertical(|mut upper_strip| {
                upper_strip.cell(|ui| {
                    if self.is_completed() {
                        ui.centered_and_justified(|ui| {
                            ui.heading("You won!");
                        });
                        return;
                    }

                    self.draw_grid(ui, 9, 9, |field_strip, row_idx, col_idx| {
                        field_strip.cell(|ui| {
                            if let Some(field) = self.get_field(row_idx, col_idx) {
                                let p = self.draw_field(field, ui);
                                if let Some(new_value) = p {
                                    changes.push((row_idx, col_idx, new_value));
                                }
                            }
                        });
                    });
                });
            });

        for (row_idx, col_idx, changed_field) in changes {
            if let Some(field) = self.get_field_mut(row_idx, col_idx) {
                *field = changed_field;
            }
        }
    }
}

impl SudokuGrid {
    fn draw_field(&self, field: &Field, ui: &mut egui::Ui) -> Option<Field> {
        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        let mut new_value: Option<Field> = None;

        match field.value {
            None => {
                let color = if field.possibilities.len() == 1 {
                    faded_color(Color32::BLUE)
                } else {
                    faded_color(Color32::RED)
                };
                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, color);

                self.draw_grid(ui, 3, 3, |field_strip, row, col| {
                    field_strip.cell(|ui| {
                        let possibility = row * 3 + col + 1;
                        ui.centered_and_justified(|ui| {
                            if field.possibilities.contains(&possibility) {
                                let response = ui.label(&possibility.to_string());

                                let response = response.interact(egui::Sense::click());
                                if response.clicked() {
                                    new_value = Some(Field::filled(possibility));
                                }
                                response.context_menu(|ui| {
                                    new_value = Some(field.without_possibility(possibility));
                                    ui.close_menu();
                                });
                            } else {
                                ui.label("");
                            }
                        });
                    });
                });
            }
            Some(value) => {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    0.0,
                    faded_color(Color32::GREEN),
                );

                ui.centered_and_justified(|ui| {
                    let response = ui.label(format!("{}", value));

                    let response = response.interact(egui::Sense::click());
                    response.context_menu(|ui| {
                        new_value = Some(Field::empty());
                        ui.close_menu();
                    });
                });
            }
        }

        new_value
    }

    fn draw_grid<F>(
        &self,
        ui: &mut egui::Ui,
        row_count: usize,
        column_count: usize,
        mut field_fn: F,
    ) where
        F: FnMut(&mut Strip, usize, usize),
    {
        StripBuilder::new(ui)
            .sizes(Size::remainder(), row_count)
            .vertical(|mut strip| {
                for row in 0..row_count {
                    strip.strip(|row_builder| {
                        row_builder
                            .sizes(Size::remainder(), column_count)
                            .horizontal(|mut field_strip| {
                                for column in 0..column_count {
                                    field_fn(&mut field_strip, row, column);
                                }
                            });
                    });
                }
            });
    }
}
