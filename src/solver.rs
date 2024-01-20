use crate::sudoku::{Field, SudokuGrid};

pub struct SudokuSolver {
    grid: SudokuGrid,
    solving_strategies: Vec<Box<dyn SolvingStrategy>>,
    solve_steps: Vec<((usize, usize), SolveStep)>,
}

impl SudokuSolver {
    pub fn new(grid: SudokuGrid) -> Self {
        let mut this = Self {
            grid,
            solving_strategies: vec![],
            solve_steps: vec![],
        };

        this.add_solving_strategy(Box::new(RemovePossibilitiesViaRowValues {}));
        this.add_solving_strategy(Box::new(RemovePossibilitiesViaColumnValues {}));
        this.add_solving_strategy(Box::new(RemovePossibilitiesViaBoxValues {}));
        this.add_solving_strategy(Box::new(SetValueIfOnlyOnePossibilityLeft {}));
        this.add_solving_strategy(Box::new(SetValueIfFieldIsOnlyOwnerOfPossibilityInRow {}));
        this.add_solving_strategy(Box::new(SetValueIfFieldIsOnlyOwnerOfPossibilityInColumn {}));
        this.add_solving_strategy(Box::new(SetValueIfFieldIsOnlyOwnerOfPossibilityInBox {}));
        this.add_solving_strategy(Box::new(RemovePossibilitiesByPairsOfSizeNInRow {}));
        this.add_solving_strategy(Box::new(RemovePossibilitiesByPairsOfSizeNInColummn {}));
        this.add_solving_strategy(Box::new(RemovePossibilitiesByPairsOfSizeNInBox {}));

        this
    }

    pub fn solve_steps(&self) -> &Vec<((usize, usize), SolveStep)> {
        &self.solve_steps
    }

    pub fn grid(&mut self) -> &SudokuGrid {
        &self.grid
    }

    pub fn grid_mut(&mut self) -> &mut SudokuGrid {
        &mut self.grid
    }

    pub fn add_solving_strategy(&mut self, strategy: Box<dyn SolvingStrategy>) {
        self.solving_strategies.push(strategy);
    }

    pub fn apply_solve_steps(&mut self, solve_steps: Vec<((usize, usize), SolveStep)>) {
        for ((row, column), solve_step) in solve_steps {
            self.solve_steps.push(((row, column), solve_step.clone()));
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
        for field in self.grid.fields().iter().filter(|f| !f.is_filled()) {
            for strategy in self.solving_strategies.iter() {
                match strategy.solve_field(field, &self.grid) {
                    None => {}
                    Some(SolveStep::SetValue(value)) => {
                        solve_steps
                            .push(((field.row(), field.column()), SolveStep::SetValue(value)));
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
                                (field.row(), field.column()),
                                SolveStep::RemovePossibilities(x),
                            ));
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

pub trait SolvingStrategy {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep>;
}

pub struct RemovePossibilitiesViaRowValues {}
impl SolvingStrategy for RemovePossibilitiesViaRowValues {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let mut values: Vec<usize> = grid
            .get_fields_in_row(field.row())
            .map_or(vec![], |fields| {
                fields.iter().filter_map(|f| f.value()).collect()
            });

        values.dedup();

        Some(SolveStep::RemovePossibilities(values))
    }
}

pub struct RemovePossibilitiesViaColumnValues {}
impl SolvingStrategy for RemovePossibilitiesViaColumnValues {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let mut values: Vec<usize> = grid
            .get_fields_in_column(field.column())
            .iter()
            .filter_map(|f| f.value())
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
            .filter_map(|f| f.value())
            .collect();

        values.dedup();

        Some(SolveStep::RemovePossibilities(values))
    }
}

pub struct SetValueIfOnlyOnePossibilityLeft {}
impl SolvingStrategy for SetValueIfOnlyOnePossibilityLeft {
    fn solve_field(&self, field: &Field, _: &SudokuGrid) -> Option<SolveStep> {
        if field.possibilities().len() == 1 {
            let value = field.possibilities().first()?.to_owned();
            println!(
                "{value} is the only possible value for {} / {}",
                field.row(),
                field.column()
            );
            Some(SolveStep::SetValue(value))
        } else {
            None
        }
    }
}

fn count_possibilities_for_fields(fields: Vec<&Field>) -> [usize; 10] {
    let mut all_possibilities = [0; 10];

    for field in fields.iter() {
        for possibility in field.possibilities().iter() {
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
        let fields = grid.get_fields_in_row(field.row())?;

        let possibilities = count_possibilities_for_fields(fields.iter().collect());

        for possibility in field.possibilities().clone().iter() {
            if let Some(p) = possibilities.get(*possibility) {
                if *p == 1 {
                    println!("[row] {possibility} was never found, assuming {} / {} is the only place it can go", field.row(), field.column());
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
        let fields = grid.get_fields_in_column(field.column());

        let possibilities = count_possibilities_for_fields(fields);

        for possibility in field.possibilities().clone().iter() {
            if let Some(p) = possibilities.get(*possibility) {
                if *p == 1 {
                    println!("[column] {possibility} was never found, assuming {} / {} is the only place it can go", field.row(), field.column());
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

        for possibility in field.possibilities().clone().iter() {
            if let Some(p) = possibilities.get(*possibility) {
                if *p == 1 {
                    println!("[box] {possibility} was never found, assuming {} / {} is the only place it can go", field.row(), field.column());
                    return Some(SolveStep::SetValue(*possibility));
                }
            }
        }

        None
    }
}

pub struct RemovePossibilitiesByPairsOfSizeNInRow {}
impl SolvingStrategy for RemovePossibilitiesByPairsOfSizeNInRow {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let fields = grid.get_fields_in_row(field.row())?;

        let fields_possibilities = fields
            .iter()
            .filter(|f| f.column() != field.column())
            .map(|field| field.possibilities().clone())
            .collect::<Vec<Vec<usize>>>();

        let possibilities_to_remove = find_grouped_possibilities(fields_possibilities);
        if possibilities_to_remove.is_empty() {
            None
        } else {
            Some(SolveStep::RemovePossibilities(possibilities_to_remove))
        }
    }
}
fn find_grouped_possibilities(fields_possibilities: Vec<Vec<usize>>) -> Vec<usize> {
    let mut possibilties_to_remove = vec![];

    let fields_possibilities = fields_possibilities
        .into_iter()
        .filter(|fp| fp.len() > 1)
        .collect::<Vec<Vec<usize>>>();

    for field_possibilities in fields_possibilities.iter() {
        let len = field_possibilities.len();
        let matches = fields_possibilities
            .iter()
            .filter(|possibilities| {
                // more possibilities than the one we are looking for => ignore this field
                if possibilities.len() > len {
                    return false;
                }

                possibilities
                    .iter()
                    .all(|p| field_possibilities.contains(p))
            })
            .collect::<Vec<&Vec<usize>>>();

        if matches.len() == len {
            possibilties_to_remove.append(&mut field_possibilities.clone());
        }
    }

    possibilties_to_remove.sort();
    possibilties_to_remove.dedup();

    possibilties_to_remove
}

pub struct RemovePossibilitiesByPairsOfSizeNInColummn {}
impl SolvingStrategy for RemovePossibilitiesByPairsOfSizeNInColummn {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let fields = grid.get_fields_in_column(field.column());

        let fields_possibilities = fields
            .iter()
            .filter(|f| f.row() != field.row())
            .map(|field| field.possibilities().clone())
            .collect::<Vec<Vec<usize>>>();

        let possibilities_to_remove = find_grouped_possibilities(fields_possibilities);
        if possibilities_to_remove.is_empty() {
            None
        } else {
            Some(SolveStep::RemovePossibilities(possibilities_to_remove))
        }
    }
}
pub struct RemovePossibilitiesByPairsOfSizeNInBox {}
impl SolvingStrategy for RemovePossibilitiesByPairsOfSizeNInBox {
    fn solve_field(&self, field: &Field, grid: &SudokuGrid) -> Option<SolveStep> {
        let box_id = SudokuGrid::get_box_id_for_field(field)?;
        let fields = grid.get_fields_in_box(box_id);

        let fields_possibilities = fields
            .iter()
            .filter(|f| f.column() != field.column() && f.row() != field.row())
            .map(|field| field.possibilities().clone())
            .collect::<Vec<Vec<usize>>>();

        let possibilities_to_remove = find_grouped_possibilities(fields_possibilities);
        if possibilities_to_remove.is_empty() {
            None
        } else {
            Some(SolveStep::RemovePossibilities(possibilities_to_remove))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_for_pairs() {
        let possibilities = vec![vec![1, 2], vec![1, 2], vec![2, 3], vec![1, 2, 4, 5]];
        let result = vec![1, 2];

        assert_eq!(result, super::find_grouped_possibilities(possibilities));
    }

    #[test]
    fn it_works_for_triples() {
        let possibilities = vec![
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 4, 5],
        ];
        let result = vec![1, 2, 3];

        assert_eq!(result, super::find_grouped_possibilities(possibilities));
    }

    #[test]
    fn it_works_for_triples_with_missing_possibilities() {
        let possibilities = vec![vec![1, 2, 3], vec![1, 3], vec![1, 2, 3], vec![1, 2, 4, 5]];
        let result = vec![1, 2, 3];

        assert_eq!(result, super::find_grouped_possibilities(possibilities));

        let possibilities = vec![vec![1, 2, 3], vec![1, 3], vec![1, 2], vec![1, 2, 4, 5]];
        let result = vec![1, 2, 3];

        assert_eq!(result, super::find_grouped_possibilities(possibilities));

        let possibilities = vec![
            vec![1, 2, 3],
            vec![1, 3],
            vec![1, 2, 3],
            vec![1, 2, 3, 4, 5],
        ];
        let result = vec![1, 2, 3];

        assert_eq!(result, super::find_grouped_possibilities(possibilities));
        let possibilities = vec![
            vec![1, 2, 3],
            vec![1, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3, 4, 5],
        ];
        let result = vec![1, 2, 3, 4, 5];

        assert_eq!(result, super::find_grouped_possibilities(possibilities));
        let possibilities = vec![vec![1, 5, 8], vec![1, 4, 5, 8], vec![], vec![]];
        let result: Vec<usize> = vec![];

        assert_eq!(result, super::find_grouped_possibilities(possibilities));
        let possibilities = vec![vec![1, 5, 8], vec![1, 4, 5, 8], vec![2], vec![3]];
        let result: Vec<usize> = vec![];

        assert_eq!(result, super::find_grouped_possibilities(possibilities));
    }
}
