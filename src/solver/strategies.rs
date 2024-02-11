use crate::sudoku::field::Field;
use crate::sudoku::grid::SudokuGrid;

use super::SolveStep;

pub(crate) fn remove_possibilities_by_row_values(
    field: &Field,
    grid: &SudokuGrid,
) -> Option<SolveStep> {
    let mut values: Vec<usize> = grid
        .get_fields_in_row(field.position().row())
        .map_or(vec![], |fields| {
            fields.iter().filter_map(|f| f.value()).collect()
        });

    values.dedup();

    Some(SolveStep::RemovePossibilities(values))
}

pub(crate) fn remove_possibilities_by_column_values(
    field: &Field,
    grid: &SudokuGrid,
) -> Option<SolveStep> {
    let mut values: Vec<usize> = grid
        .get_fields_in_column(field.position().column())
        .iter()
        .filter_map(|f| f.value())
        .collect();

    values.dedup();

    Some(SolveStep::RemovePossibilities(values))
}

pub(crate) fn remove_possibilities_via_box_values(
    field: &Field,
    grid: &SudokuGrid,
) -> Option<SolveStep> {
    let box_id = SudokuGrid::get_box_id_for_field(field)?;

    let mut values: Vec<usize> = grid
        .get_fields_in_box(box_id)
        .iter()
        .filter_map(|f| f.value())
        .collect();

    values.dedup();

    Some(SolveStep::RemovePossibilities(values))
}

pub(crate) fn set_value_if_only_one_possibility_left(
    field: &Field,
    _: &SudokuGrid,
) -> Option<SolveStep> {
    if field.possibilities().len() == 1 {
        let value = field.possibilities().first()?.to_owned();
        println!(
            "{value} is the only possible value for {} / {}",
            field.position().row(),
            field.position().column()
        );
        Some(SolveStep::SetValue(value))
    } else {
        None
    }
}

pub(crate) fn count_possibilities_for_fields(fields: Vec<&Field>) -> [usize; 10] {
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

pub(crate) fn set_value_if_field_is_only_owner_of_possibility_in_row(
    field: &Field,
    grid: &SudokuGrid,
) -> Option<SolveStep> {
    let fields = grid.get_fields_in_row(field.position().row())?;

    let possibilities = count_possibilities_for_fields(fields.iter().collect());

    for possibility in field.possibilities().clone().iter() {
        if let Some(p) = possibilities.get(*possibility) {
            if *p == 1 {
                println!("[row] {possibility} was never found, assuming {} / {} is the only place it can go", field.position().row(), field.position().column());
                return Some(SolveStep::SetValue(*possibility));
            }
        }
    }

    None
}

pub(crate) fn set_value_if_field_is_only_owner_of_possibility_in_column(
    field: &Field,
    grid: &SudokuGrid,
) -> Option<SolveStep> {
    let fields = grid.get_fields_in_column(field.position().column());

    let possibilities = count_possibilities_for_fields(fields);

    for possibility in field.possibilities().clone().iter() {
        if let Some(p) = possibilities.get(*possibility) {
            if *p == 1 {
                println!("[column] {possibility} was never found, assuming {} / {} is the only place it can go", field.position().row(), field.position().column());
                return Some(SolveStep::SetValue(*possibility));
            }
        }
    }

    None
}

pub(crate) fn set_value_if_field_is_only_owner_of_possibility_in_box(
    field: &Field,
    grid: &SudokuGrid,
) -> Option<SolveStep> {
    let box_id = SudokuGrid::get_box_id_for_field(field)?;
    let fields = grid.get_fields_in_box(box_id);
    let possibilities = count_possibilities_for_fields(fields);

    for possibility in field.possibilities().clone().iter() {
        if let Some(p) = possibilities.get(*possibility) {
            if *p == 1 {
                println!("[box] {possibility} was never found, assuming {} / {} is the only place it can go", field.position().row(), field.position().column());
                return Some(SolveStep::SetValue(*possibility));
            }
        }
    }

    None
}

pub(crate) fn remove_possibilities_by_pairs_of_size_nin_row(
    field: &Field,
    grid: &SudokuGrid,
) -> Option<SolveStep> {
    let fields = grid.get_fields_in_row(field.position().row())?;

    let fields_possibilities = fields
        .iter()
        .filter(|f| f.position().column() != field.position().column())
        .map(|field| field.possibilities().clone())
        .collect::<Vec<Vec<usize>>>();

    let possibilities_to_remove = find_grouped_possibilities(fields_possibilities);
    if possibilities_to_remove.is_empty() {
        None
    } else {
        Some(SolveStep::RemovePossibilities(possibilities_to_remove))
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

pub(crate) fn remove_possibilities_by_pairs_of_size_nin_colummn(
    field: &Field,
    grid: &SudokuGrid,
) -> Option<SolveStep> {
    let fields = grid.get_fields_in_column(field.position().column());

    let fields_possibilities = fields
        .iter()
        .filter(|f| f.position().row() != field.position().row())
        .map(|field| field.possibilities().clone())
        .collect::<Vec<Vec<usize>>>();

    let possibilities_to_remove = find_grouped_possibilities(fields_possibilities);
    if possibilities_to_remove.is_empty() {
        None
    } else {
        Some(SolveStep::RemovePossibilities(possibilities_to_remove))
    }
}
pub(crate) fn remove_possibilities_by_pairs_of_size_nin_box(
    field: &Field,
    grid: &SudokuGrid,
) -> Option<SolveStep> {
    let box_id = SudokuGrid::get_box_id_for_field(field)?;
    let fields = grid.get_fields_in_box(box_id);

    let fields_possibilities = fields
        .iter()
        .filter(|f| {
            f.position().column() != field.position().column()
                && f.position().row() != field.position().row()
        })
        .map(|field| field.possibilities().clone())
        .collect::<Vec<Vec<usize>>>();

    let possibilities_to_remove = find_grouped_possibilities(fields_possibilities);
    if possibilities_to_remove.is_empty() {
        None
    } else {
        Some(SolveStep::RemovePossibilities(possibilities_to_remove))
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
