use crate::sudoku::grid::SudokuGrid;
use crate::ui::SudokuUi;
use std::path::Path;

use super::SudokuGridWithColoredFields;

impl SudokuUi {
    pub fn import_from(&mut self, target: &Path) -> Result<(), String> {
        let grid = SudokuGrid::try_from(target.to_path_buf())?;
        *self.grid_mut() = Some(SudokuGridWithColoredFields::new(grid));

        Ok(())
    }
}
