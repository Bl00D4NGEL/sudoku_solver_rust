use crate::{
    solver::{SolveStep, SudokuSolver},
    sudoku::{Field, SudokuGrid},
};
use eframe::{egui, App};
use egui::Color32;
use egui_extras::{Size, Strip, StripBuilder};

impl App for SudokuSolver {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.solve();

        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.8))
                .size(Size::relative(0.2))
                .horizontal(|mut upper_strip| {
                    upper_strip.cell(|ui| {
                        let changes = self.grid_mut().ui(ui);
                        self.apply_solve_steps(changes);
                    });

                    upper_strip.cell(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if self.grid().is_completed() {
                                ui.label("You won!");
                            }
                            for ((row, col), solve_step) in self.solve_steps().iter().rev() {
                                ui.label(format!(
                                    "{row} / {col} => {}",
                                    match &solve_step {
                                        SolveStep::SetValue(value) => format!("Set {value}"),
                                        SolveStep::RemovePossibilities(p) => {
                                            format!("Remove {p:?}")
                                        }
                                    },
                                ));
                            }
                        });
                    })
                });

            ctx.request_repaint();
        });
    }
}

impl SudokuGrid {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<((usize, usize), SolveStep)> {
        let mut changes = vec![];
        draw_grid(ui, 9, 9, |field_strip, row_idx, col_idx| {
            field_strip.cell(|ui| {
                if let Some(field) = self.get_field(row_idx, col_idx) {
                    let p = field.ui(ui);
                    if let Some(solve_step) = p {
                        changes.push(((field.row(), field.column()), solve_step));
                    }
                }
            });
        });

        changes
    }
}

impl Field {
    fn ui(&self, ui: &mut egui::Ui) -> Option<SolveStep> {
        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        let mut solve_step: Option<SolveStep> = None;

        match self.value() {
            None => {
                let color = if self.possibilities().len() == 1 {
                    faded_color(Color32::BLUE)
                } else {
                    faded_color(Color32::RED)
                };

                if self.possibilities().is_empty() {
                    panic!(
                        "No field value nor possibilities exist for {} / {}",
                        self.row(),
                        self.column()
                    );
                }

                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, color);

                draw_grid(ui, 3, 3, |field_strip, row, col| {
                    field_strip.cell(|ui| {
                        let possibility = row * 3 + col + 1;
                        ui.centered_and_justified(|ui| {
                            if self.possibilities().contains(&possibility) {
                                let response = ui.label(&possibility.to_string());

                                let response = response.interact(egui::Sense::click());
                                if response.clicked() {
                                    solve_step = Some(SolveStep::SetValue(possibility));
                                }
                                response.context_menu(|ui| {
                                    solve_step =
                                        Some(SolveStep::RemovePossibilities(vec![possibility]));
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
                    ui.label(format!("{}", value));
                });
            }
        }

        solve_step
    }
}

fn draw_grid<F>(ui: &mut egui::Ui, row_count: usize, column_count: usize, mut field_fn: F)
where
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
