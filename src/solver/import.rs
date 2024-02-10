use std::path::Path;

use crate::sudoku::SudokuGrid;

use super::SudokuSolver;

impl SudokuSolver {
    pub fn import_from(&mut self, target: &Path) -> Result<(), String> {
        self.grid = Some(SudokuGrid::try_from(target.to_path_buf())?);

        Ok(())
    }
}
