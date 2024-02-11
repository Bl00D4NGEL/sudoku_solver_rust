mod strategies;
use crate::sudoku::field::{Field, FieldPosition};
use crate::sudoku::grid::SudokuGrid;

type SolveFn = dyn Fn(&Field, &SudokuGrid) -> Option<SolveStep>;

pub struct SudokuSolver {
    solving_strategies: Vec<Box<SolveFn>>,
}

impl SudokuSolver {
    pub fn new() -> Self {
        Self {
            solving_strategies: vec![
                Box::new(strategies::remove_possibilities_by_row_values),
                Box::new(strategies::remove_possibilities_by_column_values),
                Box::new(strategies::remove_possibilities_via_box_values),
                Box::new(strategies::set_value_if_only_one_possibility_left),
                Box::new(strategies::set_value_if_field_is_only_owner_of_possibility_in_row),
                Box::new(strategies::set_value_if_field_is_only_owner_of_possibility_in_column),
                Box::new(strategies::set_value_if_field_is_only_owner_of_possibility_in_box),
                Box::new(strategies::remove_possibilities_by_pairs_of_size_nin_row),
                Box::new(strategies::remove_possibilities_by_pairs_of_size_nin_colummn),
                Box::new(strategies::remove_possibilities_by_pairs_of_size_nin_box),
            ],
        }
    }

    pub fn determine_solve_steps(&self, grid: &SudokuGrid) -> Vec<(FieldPosition, SolveStep)> {
        let mut solve_steps = vec![];
        for field in grid.fields().iter().filter(|f| !f.is_filled()) {
            for strategy in self.solving_strategies.iter() {
                match strategy(field, grid) {
                    None => {}
                    Some(SolveStep::SetValue(value)) => {
                        solve_steps.push((field.position().clone(), SolveStep::SetValue(value)));
                        break;
                    }
                    Some(SolveStep::RemovePossibilities(possibilities_to_remove)) => {
                        let x = possibilities_to_remove
                            .iter()
                            .filter(|p| field.possibilities().contains(p))
                            .copied()
                            .collect::<Vec<usize>>();
                        if !x.is_empty() {
                            solve_steps.push((
                                field.position().clone(),
                                SolveStep::RemovePossibilities(x),
                            ));
                        }
                    }
                }
            }
        }

        solve_steps
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SolveStep {
    SetValue(usize),
    RemovePossibilities(Vec<usize>),
}

impl SudokuGrid {
    pub fn apply_solve_steps(&mut self, solve_steps: &Vec<(FieldPosition, SolveStep)>) {
        for (position, solve_step) in solve_steps {
            if let Some(field) = self.get_field_mut(position) {
                match &solve_step {
                    SolveStep::SetValue(value) => field.set_value(*value),
                    SolveStep::RemovePossibilities(possibilities) => {
                        for possibiliy in possibilities {
                            field.remove_possibility(*possibiliy);
                        }
                    }
                }
            }
        }
    }
}
