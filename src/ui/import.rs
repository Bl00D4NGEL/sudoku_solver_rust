use crate::sudoku::grid::SudokuGrid;
use crate::ui::SudokuUi;
use std::path::Path;

impl SudokuUi {
    pub fn import_from(&mut self, target: &Path) -> Result<(), String> {
        *self.grid_mut() = Some(SudokuGrid::try_from(target.to_path_buf())?);

        Ok(())
    }
}
