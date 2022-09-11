use sudoku::{Field, Row};

pub mod sudoku;

pub fn run(row: &Row) -> Row {
    if row.empty_fields().len() != 1 {
        return row.clone();
    }

    let mut mutable_row = row.clone();

    mutable_row.update_possibilities();

    let new_fields = mutable_row.fields().clone().map(|f| {
        if !f.is_empty() {
            return f;
        }

        if f.possibilities().len() == 1 {
            return Field::new(f.possibilities()[0]);
        }

        return f;
    });

    return Row::new(new_fields);
}
