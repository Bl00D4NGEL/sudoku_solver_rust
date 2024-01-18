use crate::{Field, SudokuGrid};

pub struct SudokuSolver {
    grid: SudokuGrid,
    solving_strategies: Vec<Box<dyn SolvingStrategy>>,
}

impl SudokuSolver {
    pub fn new(grid: SudokuGrid) -> Self {
        let mut this = Self {
            grid,
            solving_strategies: vec![],
        };

        this.add_solving_strategy(Box::new(RemovePossibilitiesViaRowValues {}));
        this.add_solving_strategy(Box::new(RemovePossibilitiesViaColumnValues {}));
        this.add_solving_strategy(Box::new(RemovePossibilitiesViaBoxValues {}));
        this.add_solving_strategy(Box::new(SetValueIfOnlyOnePossibilityLeft {}));
        this.add_solving_strategy(Box::new(SetValueIfFieldIsOnlyOwnerOfPossibilityInRow {}));
        this.add_solving_strategy(Box::new(SetValueIfFieldIsOnlyOwnerOfPossibilityInColumn {}));
        this.add_solving_strategy(Box::new(SetValueIfFieldIsOnlyOwnerOfPossibilityInBox {}));

        this
    }

    pub fn grid_mut(&mut self) -> &mut SudokuGrid {
        &mut self.grid
    }

    pub fn add_solving_strategy(&mut self, strategy: Box<dyn SolvingStrategy>) {
        self.solving_strategies.push(strategy);
    }

    pub fn apply_solve_steps(&mut self, solve_steps: Vec<((usize, usize), SolveStep)>) {
        for ((row, column), solve_step) in solve_steps {
            if let Some(field) = self.grid.get_field_mut(row, column) {
                match solve_step {
                    SolveStep::SetValue(value) => field.set_value(value),
                    SolveStep::RemovePossibilities(possibilities) => {
                        for possibiliy in possibilities {
                            field.remove_possibility(possibiliy);
                        }
                    }
                }
            }
        }
    }

    pub fn solve(&mut self) {
        let mut solve_steps = vec![];
        for field in self.grid.fields() {
            if field.is_filled() {
                continue;
            }

            for strategy in self.solving_strategies.iter() {
                if let Some(x) = strategy.solve_field(field, &self.grid) {
                    solve_steps.push(((field.row, field.column), x.clone()));

                    if let SolveStep::SetValue(_) = x {
                        break;
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

pub trait SolvingStrategy {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep>;
}

pub struct RemovePossibilitiesViaRowValues {}
impl SolvingStrategy for RemovePossibilitiesViaRowValues {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let mut values: Vec<usize> = grid.get_fields_in_row(field.row).map_or(vec![], |fields| {
            fields.iter().filter_map(|f| f.value).collect()
        });

        values.dedup();

        Some(SolveStep::RemovePossibilities(values))
    }
}

pub struct RemovePossibilitiesViaColumnValues {}
impl SolvingStrategy for RemovePossibilitiesViaColumnValues {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let mut values: Vec<usize> = grid
            .get_fields_in_column(field.column)
            .iter()
            .filter_map(|f| f.value)
            .collect();

        values.dedup();

        Some(SolveStep::RemovePossibilities(values))
    }
}

pub struct RemovePossibilitiesViaBoxValues {}
impl SolvingStrategy for RemovePossibilitiesViaBoxValues {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let box_id = SudokuGrid::get_box_id_for_field(field)?;

        let mut values: Vec<usize> = grid
            .get_fields_in_box(box_id)
            .iter()
            .filter_map(|f| f.value)
            .collect();

        values.dedup();

        Some(SolveStep::RemovePossibilities(values))
    }
}

pub struct SetValueIfOnlyOnePossibilityLeft {}
impl SolvingStrategy for SetValueIfOnlyOnePossibilityLeft {
    fn solve_field(&self, field: &Field, _: &SudokuGrid) -> Option<SolveStep> {
        if field.possibilities.len() == 1 {
            let value = field.possibilities.first()?.to_owned();
            println!(
                "{value} is the only possible value for {} / {}",
                field.row, field.column
            );
            // return None;
            Some(SolveStep::SetValue(value))
        } else {
            None
        }
    }
}

fn count_possibilities_for_fields(fields: Vec<&Field>) -> [usize; 10] {
    let mut all_possibilities = [0; 10];

    for field in fields.iter() {
        for possibility in field.possibilities.iter() {
            if let Some(p) = all_possibilities.get_mut(*possibility) {
                *p += 1;
            }
        }
    }

    all_possibilities
}

pub struct SetValueIfFieldIsOnlyOwnerOfPossibilityInRow {}
impl SolvingStrategy for SetValueIfFieldIsOnlyOwnerOfPossibilityInRow {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let fields = grid.get_fields_in_row(field.row)?;

        let possibilities = count_possibilities_for_fields(fields.iter().collect());

        for possibility in field.possibilities.clone().iter() {
            if let Some(p) = possibilities.get(*possibility) {
                if *p == 1 {
                    println!("[row] {possibility} was never found, assuming {} / {} is the only place it can go", field.row, field.column);
                    return Some(SolveStep::SetValue(*possibility));
                }
            }
        }

        None
    }
}

pub struct SetValueIfFieldIsOnlyOwnerOfPossibilityInColumn {}
impl SolvingStrategy for SetValueIfFieldIsOnlyOwnerOfPossibilityInColumn {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let fields = grid.get_fields_in_column(field.column);

        let possibilities = count_possibilities_for_fields(fields);

        for possibility in field.possibilities.clone().iter() {
            if let Some(p) = possibilities.get(*possibility) {
                if *p == 1 {
                    println!("[column] {possibility} was never found, assuming {} / {} is the only place it can go", field.row, field.column);
                    return Some(SolveStep::SetValue(*possibility));
                }
            }
        }

        None
    }
}

pub struct SetValueIfFieldIsOnlyOwnerOfPossibilityInBox {}
impl SolvingStrategy for SetValueIfFieldIsOnlyOwnerOfPossibilityInBox {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let box_id = SudokuGrid::get_box_id_for_field(field)?;
        let fields = grid.get_fields_in_box(box_id);
        let possibilities = count_possibilities_for_fields(fields);

        for possibility in field.possibilities.clone().iter() {
            if let Some(p) = possibilities.get(*possibility) {
                if *p == 1 {
                    println!("[box] {possibility} was never found, assuming {} / {} is the only place it can go", field.row, field.column);
                    return Some(SolveStep::SetValue(*possibility));
                }
            }
        }

        None
    }
}
