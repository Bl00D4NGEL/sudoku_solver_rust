use std::{fs, path::PathBuf};

#[derive(Default, Clone, Debug)]
pub struct SudokuGrid {
    rows: Vec<Vec<Field>>,
}

#[derive(Default, Clone, Debug)]
pub struct Field {
    value: Option<usize>,
    possibilities: Vec<usize>,
    position: FieldPosition,
}

#[derive(Default, Clone, Debug)]
pub struct FieldPosition {
    row: usize,
    column: usize,
}

impl FieldPosition {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl TryFrom<PathBuf> for SudokuGrid {
    type Error = String;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let contents = &fs::read_to_string(value).unwrap();
        let lines = contents
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();

        if lines.len() != 9 {
            return Err(String::from(
                "File has more than 9 lines, can not create valid sudoku",
            ));
        }

        let rows: Vec<Vec<Field>> = lines
            .iter()
            .enumerate()
            .map(|(row_idx, line)| {
                let fields = line
                    .trim()
                    .split(' ')
                    .enumerate()
                    .map(|(col_idx, c)| match c.parse::<usize>() {
                        Ok(value) => match value {
                            1..=9 => Field::filled(value, FieldPosition::new(row_idx, col_idx)),
                            _ => Field::empty(FieldPosition::new(row_idx, col_idx)),
                        },
                        _ => Field::empty(FieldPosition::new(row_idx, col_idx)),
                    })
                    .collect::<Vec<Field>>();

                if fields.len() != 9 {
                    panic!("Row has more than 9 fields, can not create valid sudoku");
                }

                fields
            })
            .collect();

        Ok(SudokuGrid { rows })
    }
}

impl SudokuGrid {
    pub fn fields(&self) -> Vec<&Field> {
        self.rows.iter().flatten().collect()
    }

    pub fn rows(&self) -> &Vec<Vec<Field>> {
        &self.rows
    }

    pub fn get_field(&self, position: FieldPosition) -> Option<&Field> {
        let row = self.rows.get(position.row)?;

        row.get(position.column)
    }

    pub fn get_field_mut(&mut self, position: &FieldPosition) -> Option<&mut Field> {
        let row = self.rows.get_mut(position.row)?;

        row.get_mut(position.column)
    }

    pub fn is_completed(&self) -> bool {
        self.rows.iter().all(|row| {
            let mut row_values = row.iter().filter_map(|f| f.value).collect::<Vec<usize>>();
            row_values.sort();

            row_values.eq(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        })
    }

    pub fn get_fields_in_row(&self, row_idx: usize) -> Option<&Vec<Field>> {
        self.rows.get(row_idx)
    }

    pub fn get_fields_in_column(&self, col_idx: usize) -> Vec<&Field> {
        let mut fields = vec![];

        for i in 0..9 {
            match self.rows.get(i) {
                None => {}
                Some(row) => match row.get(col_idx) {
                    None => {}
                    Some(field) => fields.push(field),
                },
            }
        }

        (0..9)
            .filter_map(|i| self.rows.get(i)?.get(col_idx))
            .collect()
    }

    pub fn get_box_id_for_field(field: &Field) -> Option<usize> {
        match (field.position.row, field.position.column) {
            (0..=2, 0..=2) => Some(0),
            (0..=2, 3..=5) => Some(1),
            (0..=2, 6..=8) => Some(2),
            (3..=5, 0..=2) => Some(3),
            (3..=5, 3..=5) => Some(4),
            (3..=5, 6..=8) => Some(5),
            (6..=8, 0..=2) => Some(6),
            (6..=8, 3..=5) => Some(7),
            (6..=8, 6..=8) => Some(8),
            _ => None,
        }
    }

    pub fn get_fields_in_box(&self, box_id: usize) -> Vec<&Field> {
        self.get_fields_in_box_with_positions(box_id)
            .into_iter()
            .map(|(f, _)| f)
            .collect()
    }

    pub fn get_fields_in_box_with_positions(&self, box_id: usize) -> Vec<(&Field, (usize, usize))> {
        let indexes: Vec<(usize, usize)> = match box_id {
            0..=2 => vec![
                (0, box_id * 3),
                (0, box_id * 3 + 1),
                (0, box_id * 3 + 2),
                (1, box_id * 3),
                (1, box_id * 3 + 1),
                (1, box_id * 3 + 2),
                (2, box_id * 3),
                (2, box_id * 3 + 1),
                (2, box_id * 3 + 2),
            ],
            3..=5 => vec![
                (3, (box_id % 3) * 3),
                (3, (box_id % 3) * 3 + 1),
                (3, (box_id % 3) * 3 + 2),
                (4, (box_id % 3) * 3),
                (4, (box_id % 3) * 3 + 1),
                (4, (box_id % 3) * 3 + 2),
                (5, (box_id % 3) * 3),
                (5, (box_id % 3) * 3 + 1),
                (5, (box_id % 3) * 3 + 2),
            ],
            6..=8 => vec![
                (6, (box_id % 3) * 3),
                (6, (box_id % 3) * 3 + 1),
                (6, (box_id % 3) * 3 + 2),
                (7, (box_id % 3) * 3),
                (7, (box_id % 3) * 3 + 1),
                (7, (box_id % 3) * 3 + 2),
                (8, (box_id % 3) * 3),
                (8, (box_id % 3) * 3 + 1),
                (8, (box_id % 3) * 3 + 2),
            ],
            _ => vec![],
        };

        indexes
            .into_iter()
            .filter_map(|(row, col)| {
                self.get_field(FieldPosition::new(row, col))
                    .map(|field| (field, (row, col)))
            })
            .collect()
    }
}

impl Field {
    pub fn position(&self) -> &FieldPosition {
        &self.position
    }

    pub fn value(&self) -> Option<usize> {
        self.value
    }

    pub fn possibilities(&self) -> &Vec<usize> {
        &self.possibilities
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = Some(value)
    }

    pub fn remove_possibility(&mut self, possibility: usize) {
        self.possibilities.retain(|p| *p != possibility)
    }

    pub fn is_filled(&self) -> bool {
        self.value.is_some()
    }

    pub fn empty(position: FieldPosition) -> Self {
        Self {
            possibilities: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            value: None,
            position,
        }
    }

    pub fn filled(value: usize, position: FieldPosition) -> Self {
        Self {
            value: Some(value),
            possibilities: vec![],
            position,
        }
    }
}
