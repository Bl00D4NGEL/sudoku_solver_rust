use sudoku::Row;

pub mod sudoku;

pub fn run(row: Row) -> Row {
    let possibilities = row.get_possibilities();
    if possibilities.len() != 1 {
        return row;
    }

    let mut fields = row.fields().clone();
    for field in &mut fields {
        if field.is_empty() {
            field.update(possibilities[0])
        }
    }
    return Row::new(fields);
}
