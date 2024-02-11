use std::env::current_dir;

use crate::{
    solver::{SolveStep, SudokuSolver},
    sudoku::{Field, FieldPosition, SudokuGrid},
};
use eframe::{egui, App};
use egui::Color32;
use egui_extras::{Size, Strip, StripBuilder};

mod export;
mod import;

pub struct SudokuUi {
    auto_solve: bool,
    solver: SudokuSolver,
}

impl SudokuUi {
    pub fn new(auto_solve: bool, solver: SudokuSolver) -> Self {
        Self { auto_solve, solver }
    }

    pub fn solver(&self) -> &SudokuSolver {
        &self.solver
    }

    pub fn solver_mut(&mut self) -> &mut SudokuSolver {
        &mut self.solver
    }
}

impl App for SudokuUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.auto_solve {
            if let Some(grid) = self.solver.grid() {
                if !grid.is_completed() {
                    self.solver.solve();
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::at_most(Size::initial(20.0), 200.0))
                .size(Size::remainder())
                .vertical(|mut vertical_strip| {
                    vertical_strip.cell(|menu_ui| {
                        menu_ui.horizontal(|menu_ui| {
                            if menu_ui.button("Export").clicked() {
                                if let Ok(cwd) = current_dir() {
                                    let fd = rfd::FileDialog::new();
                                    if let Some(path) = fd.set_directory(cwd).pick_file() {
                                        self.export_to(&path);
                                    }
                                }
                            }

                            if menu_ui.button("Import").clicked() {
                                if let Ok(cwd) = current_dir() {
                                    let fd = rfd::FileDialog::new();
                                    if let Some(path) = fd.set_directory(cwd).pick_file() {
                                        self.solver.solve_steps_mut().clear();
                                        let result = self.import_from(&path);
                                        if result.is_err() {
                                            menu_ui.label("That didn't work");
                                        }
                                    }
                                }
                            }

                            menu_ui.checkbox(&mut self.auto_solve, "Auto solve");
                        });
                    });

                    vertical_strip.cell(|v_ui| {
                        StripBuilder::new(v_ui)
                            .size(Size::relative(0.8))
                            .size(Size::relative(0.2))
                            .horizontal(|mut horizontal_strip| {
                                horizontal_strip.cell(|ui| match self.solver.grid() {
                                    None => (),
                                    Some(grid) => {
                                        let changes = grid.ui(ui);
                                        self.solver.apply_solve_steps(changes);
                                    }
                                });

                                horizontal_strip.cell(|h_ui| {
                                    egui::ScrollArea::vertical().show(h_ui, |scroll_ui| {
                                        if let Some(grid) = self.solver.grid() {
                                            if grid.is_completed() {
                                                scroll_ui.label("You won!");
                                            }
                                        }

                                        for (position, solve_step) in
                                            self.solver.solve_steps().iter().rev()
                                        {
                                            scroll_ui.label(format!(
                                                "{} / {} => {}",
                                                position.row(),
                                                position.column(),
                                                match &solve_step {
                                                    SolveStep::SetValue(value) =>
                                                        format!("Set {value}"),
                                                    SolveStep::RemovePossibilities(p) => {
                                                        format!("Remove {p:?}")
                                                    }
                                                },
                                            ));
                                        }
                                    });
                                });
                            });
                    });
                })
        });

        ctx.request_repaint();
    }
}

impl SudokuGrid {
    fn ui(&self, ui: &mut egui::Ui) -> Vec<(FieldPosition, SolveStep)> {
        let mut changes = vec![];
        draw_grid(ui, 9, 9, |field_strip, position| {
            field_strip.cell(|ui| {
                if let Some(field) = self.get_field(position) {
                    let p = field.ui(ui);
                    if let Some(solve_step) = p {
                        changes.push((field.position().clone(), solve_step));
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
                        self.position().row(),
                        self.position().column()
                    );
                }

                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, color);

                draw_grid(ui, 3, 3, |field_strip, position| {
                    field_strip.cell(|ui| {
                        let possibility = position.row() * 3 + position.column() + 1;
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
                    ui.heading(format!("{}", value));
                });
            }
        }

        solve_step
    }
}

fn draw_grid<F>(ui: &mut egui::Ui, row_count: usize, column_count: usize, mut field_fn: F)
where
    F: FnMut(&mut Strip, FieldPosition),
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
                                field_fn(&mut field_strip, FieldPosition::new(row, column));
                            }
                        });
                });
            }
        });
}
