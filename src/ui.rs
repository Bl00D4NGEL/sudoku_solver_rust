use crate::{solver::SudokuSolver, Field, SudokuGrid};
use eframe::{egui, App};
use egui::Color32;
use egui_extras::{Size, Strip, StripBuilder};

impl App for SudokuSolver {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.solve();
            self.grid_mut().ui(ui);
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
