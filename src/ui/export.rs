use crate::ui::SudokuUi;
use std::{fs, path::PathBuf};

impl SudokuUi {
    pub fn export_to(&self, target: &PathBuf) {
        if let Some(grid) = self.grid() {
            let export = grid
                .grid
                .rows()
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|f| match f.value() {
                            None => "x".to_string(),
                            Some(v) => v.to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>()
                .join("\r\n");
            let _ = fs::write(target, export.as_bytes());
        }
    }
}
