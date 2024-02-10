use std::path::PathBuf;

use crate::sudoku::SudokuGrid;

use super::SudokuSolver;

impl SudokuSolver {
    pub fn import_from(&mut self, target: &PathBuf) -> Result<(), String> {
        self.grid = SudokuGrid::try_from(target.clone())?;

        Ok(())
    }
}
