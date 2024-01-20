#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{env, path::PathBuf};

use eframe::egui;
use solver::SudokuSolver;
use sudoku::SudokuGrid;

mod solver;
mod sudoku;
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
    let solver = SudokuSolver::new(grid);
    eframe::run_native("Sudoku solver", options, Box::new(|_| Box::new(solver)))
}
