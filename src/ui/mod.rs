use std::env::current_dir;

use crate::solver::{SolveStep, SudokuSolver};
use crate::sudoku::field::{Field, FieldPosition};
use crate::sudoku::grid::SudokuGrid;
use eframe::{egui, App};
use egui::Color32;
use egui_extras::{Size, Strip, StripBuilder};

mod export;
mod import;

pub struct SudokuUi {
    auto_solve: bool,
    solver: SudokuSolver,
    grid: Option<SudokuGridWithColoredFields>,
    solve_steps: Vec<(FieldPosition, SolveStep)>,
}

#[derive(Clone, Debug)]
pub struct SudokuGridWithColoredFields {
    grid: SudokuGrid,
    field_metadata: Vec<FieldWithMetaData>,
}

impl SudokuGridWithColoredFields {
    fn new(grid: SudokuGrid) -> Self {
        Self {
            grid: grid.clone(),
            field_metadata: grid
                .rows()
                .iter()
                .flat_map(|row| {
                    row.iter()
                        .map(|f| FieldWithMetaData::new(f.position().clone()))
                        .collect::<Vec<FieldWithMetaData>>()
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldWithMetaData {
    field_position: FieldPosition,
    metadata: FieldMetaData,
}

impl FieldWithMetaData {
    fn new(field_position: FieldPosition) -> Self {
        Self {
            field_position,
            metadata: FieldMetaData::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FieldMetaData {
    color: Option<Color32>,
}

impl SudokuUi {
    pub fn new(grid: Option<SudokuGrid>) -> Self {
        Self {
            auto_solve: false,
            solver: SudokuSolver::new(),
            grid: grid.map(SudokuGridWithColoredFields::new),
            solve_steps: vec![],
        }
    }

    pub fn grid(&self) -> Option<&SudokuGridWithColoredFields> {
        self.grid.as_ref()
    }

    pub fn grid_mut(&mut self) -> &mut Option<SudokuGridWithColoredFields> {
        &mut self.grid
    }

    pub fn add_solve_steps(&mut self, solve_steps: &Vec<(FieldPosition, SolveStep)>) {
        for (position, solve_step) in solve_steps {
            self.solve_steps
                .push((position.clone(), solve_step.clone()));
        }
    }
}

impl App for SudokuUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.auto_solve {
            if let Some(grid) = self.grid() {
                if !grid.grid.is_completed() {
                    let solve_steps = self.solver.determine_solve_steps(&grid.grid);

                    if let Some(mut_grid) = self.grid_mut() {
                        mut_grid.grid.apply_solve_steps(&solve_steps);

                        self.add_solve_steps(&solve_steps);
                    }
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
                                        self.solve_steps.clear();
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
                                horizontal_strip.cell(|ui| {
                                    let mut changes = vec![];
                                    match self.grid_mut() {
                                        None => (),
                                        Some(grid) => {
                                            changes = grid.ui(ui);
                                        }
                                    }

                                    if let Some(grid) = self.grid_mut() {
                                        grid.grid.apply_solve_steps(&changes);

                                        self.add_solve_steps(&changes);
                                    }
                                });

                                horizontal_strip.cell(|h_ui| {
                                    egui::ScrollArea::vertical().show(h_ui, |scroll_ui| {
                                        if let Some(grid) = self.grid() {
                                            if grid.grid.is_completed() {
                                                scroll_ui.label("You won!");
                                            }
                                        }

                                        for (position, solve_step) in self.solve_steps.iter().rev()
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

impl SudokuGridWithColoredFields {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<(FieldPosition, SolveStep)> {
        let mut changes = vec![];
        draw_grid(ui, 9, 9, |field_strip, position| {
            field_strip.cell(|ui| {
                let index = position.row() * 9 + position.column();
                if let Some(field_metadata) = self.field_metadata.get_mut(index) {
                    if let Some(field) = self.grid.get_field(field_metadata.field_position.clone())
                    {
                        let p = field_metadata.ui(ui, field);
                        if let Some(solve_step) = p {
                            changes.push((field_metadata.field_position.clone(), solve_step));
                        }
                    }
                }
            });
        });

        changes
    }
}

impl FieldWithMetaData {
    fn ui(&mut self, ui: &mut egui::Ui, field: &Field) -> Option<SolveStep> {
        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        let mut solve_step: Option<SolveStep> = None;
        self.metadata.color = match field.value().unwrap_or(0) {
            1 => Some(Color32::BLUE),
            2 => Some(Color32::GREEN),
            3 => Some(Color32::RED),
            4 => Some(Color32::DARK_BLUE),
            5 => Some(Color32::DARK_GREEN),
            6 => Some(Color32::DARK_RED),
            7 => Some(Color32::LIGHT_BLUE),
            8 => Some(Color32::LIGHT_GREEN),
            9 => Some(Color32::LIGHT_RED),
            _ => None,
        };

        let color = faded_color(self.metadata.color.unwrap_or(Color32::WHITE));
        match field.value() {
            None => {
                if field.possibilities().is_empty() {
                    panic!(
                        "No field value nor possibilities exist for {} / {}",
                        field.position().row(),
                        field.position().column()
                    );
                }

                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, color);

                draw_grid(ui, 3, 3, |field_strip, position| {
                    field_strip.cell(|ui| {
                        let possibility = position.row() * 3 + position.column() + 1;
                        ui.centered_and_justified(|ui| {
                            if field.possibilities().contains(&possibility) {
                                let response = ui.label(&possibility.to_string());

                                let response = response.interact(egui::Sense::click());
                                if response.clicked() {
                                    self.metadata.color = Some(match possibility {
                                        1 => Color32::BLUE,
                                        2 => Color32::GREEN,
                                        3 => Color32::RED,
                                        4 => Color32::DARK_BLUE,
                                        5 => Color32::DARK_GREEN,
                                        6 => Color32::DARK_RED,
                                        7 => Color32::LIGHT_BLUE,
                                        8 => Color32::LIGHT_GREEN,
                                        9 => Color32::LIGHT_RED,
                                        _ => Color32::BLACK,
                                    });

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
                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, color);

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
