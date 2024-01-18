use crate::{Field, SudokuGrid};

pub struct SudokuSolver {
    grid: SudokuGrid,
    solving_strategies: Vec<Box<dyn SolvingStrategy>>,
}

impl SudokuSolver {
    pub fn new(grid: SudokuGrid) -> Self {
        Self {
            grid,
            solving_strategies: vec![],
        }
    }

    pub fn grid(&self) -> &SudokuGrid {
        &self.grid
    }

    pub fn grid_mut(&mut self) -> &mut SudokuGrid {
        &mut self.grid
    }

    pub fn add_solving_strategy(&mut self, strategy: Box<dyn SolvingStrategy>) {
        self.solving_strategies.push(strategy);
    }

    pub fn solve(&mut self) {
        let mut changes = vec![];
        for (row_idx, row) in self.grid.rows.iter().enumerate() {
            for (col_idx, field) in row.iter().enumerate() {
                for strategy in self.solving_strategies.iter() {
                    if let Some(x) = strategy.solve_field(field, &self.grid) {
                        changes.push(((row_idx, col_idx), x));
                    }
                }
            }
        }

        for ((row_idx, col_idx), solve_step) in changes {
            if let Some(field) = self.grid.get_field_mut(row_idx, col_idx) {
                match solve_step {
                    SolveStep::SetValue(value) => *field = Field::filled(value),
                    SolveStep::RemovePossibilities(possibilities) => {
                        for possibiliy in possibilities {
                            field.remove_possibility(possibiliy);
                        }
                    }
                }
            }
        }
    }
}

pub enum SolveStep {
    SetValue(usize),
    RemovePossibilities(Vec<usize>),
}

pub trait SolvingStrategy {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep>;
}

pub struct SolveByX {}

impl SolvingStrategy for SolveByX {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        // Some(SolveStep::SetValue(1))
        None
    }
}

pub struct SolveByY {}

impl SolvingStrategy for SolveByY {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        Some(SolveStep::RemovePossibilities(vec![1]))
    }
}
