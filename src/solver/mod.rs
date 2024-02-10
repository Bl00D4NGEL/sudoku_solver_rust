use crate::sudoku::{Field, FieldPosition, SudokuGrid};

mod export;
mod import;
mod strategies;

type SolveFn = dyn Fn(&Field, &SudokuGrid) -> Option<SolveStep>;

pub struct SudokuSolver {
    grid: Option<SudokuGrid>,
    solving_strategies: Vec<Box<SolveFn>>,
    solve_steps: Vec<(FieldPosition, SolveStep)>,
}

impl SudokuSolver {
    pub fn new(grid: Option<SudokuGrid>) -> Self {
        let mut this = Self {
            grid,
            solving_strategies: vec![],
            solve_steps: vec![],
        };

        this.add_solving_strategy(Box::new(strategies::remove_possibilities_by_row_values));
        this.add_solving_strategy(Box::new(strategies::remove_possibilities_by_column_values));
        this.add_solving_strategy(Box::new(strategies::remove_possibilities_via_box_values));
        this.add_solving_strategy(Box::new(strategies::set_value_if_only_one_possibility_left));
        this.add_solving_strategy(Box::new(
            strategies::set_value_if_field_is_only_owner_of_possibility_in_row,
        ));
        this.add_solving_strategy(Box::new(
            strategies::set_value_if_field_is_only_owner_of_possibility_in_column,
        ));
        this.add_solving_strategy(Box::new(
            strategies::set_value_if_field_is_only_owner_of_possibility_in_box,
        ));
        this.add_solving_strategy(Box::new(
            strategies::remove_possibilities_by_pairs_of_size_nin_row,
        ));
        this.add_solving_strategy(Box::new(
            strategies::remove_possibilities_by_pairs_of_size_nin_colummn,
        ));
        this.add_solving_strategy(Box::new(
            strategies::remove_possibilities_by_pairs_of_size_nin_box,
        ));

        this
    }

    pub fn solve_steps(&self) -> &Vec<(FieldPosition, SolveStep)> {
        &self.solve_steps
    }

    pub fn solve_steps_mut(&mut self) -> &mut Vec<(FieldPosition, SolveStep)> {
        &mut self.solve_steps
    }

    pub fn grid(&mut self) -> &Option<SudokuGrid> {
        &self.grid
    }

    pub fn add_solving_strategy(&mut self, strategy: Box<SolveFn>) {
        self.solving_strategies.push(strategy);
    }

    pub fn apply_solve_steps(&mut self, solve_steps: Vec<(FieldPosition, SolveStep)>) {
        for (position, solve_step) in solve_steps {
            if let Some(grid) = &mut self.grid {
                if let Some(field) = grid.get_field_mut(&position) {
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
            self.solve_steps.push((position, solve_step.clone()));
        }
    }

    pub fn solve(&mut self) {
        let mut solve_steps = vec![];
        if let Some(grid) = &self.grid {
            for field in grid.fields().iter().filter(|f| !f.is_filled()) {
                for strategy in self.solving_strategies.iter() {
                    match strategy(field, grid) {
                        None => {}
                        Some(SolveStep::SetValue(value)) => {
                            solve_steps
                                .push((field.position().clone(), SolveStep::SetValue(value)));
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
        }

        self.apply_solve_steps(solve_steps);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SolveStep {
    SetValue(usize),
    RemovePossibilities(Vec<usize>),
}
