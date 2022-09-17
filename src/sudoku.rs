use std::{collections::HashMap, fs, io::Error, vec};

#[derive(Debug)]
pub struct FieldWithIndex {
    field: Field,
    index: usize,
}

impl FieldWithIndex {
    pub fn new(field: Field, index: usize) -> FieldWithIndex {
        FieldWithIndex { field, index }
    }

    pub fn field(&self) -> Field {
        self.field.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    value: Option<i32>,
    possibilities: Vec<i32>,
}

impl Field {
    pub fn new(val: i32) -> Field {
        if !(1..=9).contains(&val) {
            panic!("Value for field must be between 1 and 9, got {}", val)
        }

        Field {
            value: Option::Some(val),
            possibilities: vec![],
        }
    }

    pub fn empty() -> Field {
        Field {
            value: Option::None,
            possibilities: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }
    pub fn empty_with_possibilities(possibilities: Vec<i32>) -> Field {
        if possibilities.len().eq(&0) {
            panic!("Possibilities must have at least one entry");
        }
        Field {
            value: Option::None,
            possibilities,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    pub fn possibilities(&self) -> &Vec<i32> {
        &self.possibilities
    }

    pub fn value(&self) -> Option<i32> {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    fields: Vec<Field>,
}

impl Grid {
    pub fn create_empty() -> Grid {
        let mut fields: Vec<Field> = vec![];

        for _ in 0..81 {
            fields.push(Field::empty());
        }

        Grid { fields }
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn get_field(&self, index: usize) -> Option<FieldWithIndex> {
        self.fields
            .get(index)
            .map(|field| FieldWithIndex::new(field.clone(), index))
    }

    pub fn set_field(&mut self, index: usize, field: Field) {
        self.fields.splice(index..index + 1, vec![field]);
    }

    pub fn is_solved(&self) -> bool {
        for field in &self.fields {
            if field.is_empty() {
                return false;
            }
        }

        true
    }

    pub fn get_fields_in_row(&self, row: usize) -> Result<Vec<FieldWithIndex>, String> {
        let mut fields = vec![];

        for i in 0..9 {
            let index = Grid::row_and_col_to_index(row, i);
            fields.push(match self.get_field(index) {
                Some(field) => field,
                None => return Err(String::from("Cannot find field")),
            });
        }

        Ok(fields)
    }

    pub fn get_fields_in_column(&self, column: usize) -> Result<Vec<FieldWithIndex>, String> {
        let mut fields = vec![];

        for i in 0..9 {
            let index = Grid::row_and_col_to_index(i, column);
            fields.push(match self.get_field(index) {
                Some(field) => field,
                None => return Err(String::from("Cannot find field")),
            });
        }

        Ok(fields)
    }

    pub fn get_fields_in_box(&self, box_id: usize) -> Vec<FieldWithIndex> {
        // Box 0  0, 1, 2, 9,10,11,18,19,20
        // Box 1  3, 4, 5,12,13,14,21,22,23
        // Box 2  6, 7, 8,15,16,17,24,25,26
        // For Box 3-5 the same applies as for box 0-2 but we need to add 18 to everything:
        // 3 * 3 + 18 = 9  + 18 = 27
        // 4 * 3 + 18 = 12 + 18 = 30
        // 5 * 3 + 18 = 15 + 18 = 33
        // Box 3 27,28,29,36,37,38,45,46,47
        // Box 4 30,31,32,39,40,41,48,49,50
        // Box 5 33,34,35,42,43,44,51,52,53
        // For Box 6-8 the same applies as for box 0-2 but we need to add 36 to everything:
        // 6 * 3 + 36 = 18 + 36 = 54
        // 7 * 3 + 18 = 21 + 36 = 57
        // 8 * 3 + 18 = 24 + 36 = 60
        // Box 6 54,55,56,63,64,65,72,73,74
        // Box 7 57,58,59,66,67,68,75,76,77
        // Box 8 60,61,62,69,70,71,78,79,80
        let extra_index = match box_id {
            3 => 18,
            4 => 18,
            5 => 18,
            6 => 36,
            7 => 36,
            8 => 36,
            _ => 0,
        };
        let mut fields = vec![];
        for i in [0, 1, 2, 9, 10, 11, 18, 19, 20] {
            fields.push(FieldWithIndex::new(
                self.fields[box_id * 3 + i + extra_index].clone(),
                box_id * 3 + i + extra_index,
            ));
        }

        fields
    }

    pub fn update_possibilities(&mut self) {
        self.update_possibilities_in_rows();
        self.update_possibilities_in_columns();
        self.update_possibilities_in_boxes();
    }

    fn update_possibilities_in_rows(&mut self) {
        let mut to_update_fields = vec![];
        for row in 0..=8 {
            let fields = match self.get_fields_in_row(row) {
                Ok(r) => r,
                Err(_) => continue,
            };

            to_update_fields.append(&mut calculate_new_possibilities_for_field_set(fields));
        }

        self.update_fields(to_update_fields);
    }

    fn update_fields(&mut self, fields: Vec<FieldWithIndex>) {
        for field in fields {
            self.set_field(field.index(), field.field());
        }
    }

    fn update_possibilities_in_columns(&mut self) {
        let mut to_update_fields = vec![];
        for column in 0..=8 {
            let fields = match self.get_fields_in_column(column) {
                Ok(r) => r,
                Err(_) => continue,
            };

            to_update_fields.append(&mut calculate_new_possibilities_for_field_set(fields));
        }

        self.update_fields(to_update_fields);
    }

    fn update_possibilities_in_boxes(&mut self) {
        let mut to_update_fields = vec![];
        for box_id in 0..=8 {
            let fields = self.get_fields_in_box(box_id);

            to_update_fields.append(&mut calculate_new_possibilities_for_field_set(fields));
        }

        for field in to_update_fields {
            self.set_field(field.index(), field.field());
        }
    }

    fn row_and_col_to_index(row: usize, column: usize) -> usize {
        row * 9 + column
    }
}

fn calculate_new_possibilities_for_field_set(fields: Vec<FieldWithIndex>) -> Vec<FieldWithIndex> {
    let mut non_empty_fields = vec![];
    let mut empty_fields = vec![];

    for field in fields.into_iter() {
        if field.field().is_empty() {
            empty_fields.push(field);
        } else {
            non_empty_fields.push(field);
        }
    }

    if empty_fields.is_empty() {
        return vec![];
    }

    let used_digits: Vec<i32> = non_empty_fields
        .iter()
        .map(|f| f.field().value().unwrap_or(0))
        .filter(|v| v.gt(&0))
        .collect();

    let mut to_update_fields = vec![];
    for empty_field in empty_fields {
        let possibilities = empty_field
            .field()
            .possibilities()
            .iter()
            .filter(|p| !used_digits.contains(p))
            .copied()
            .collect();

        to_update_fields.push(FieldWithIndex::new(
            Field::empty_with_possibilities(possibilities),
            empty_field.index(),
        ));
    }

    advanced_possibility_removal(to_update_fields)
}

// This function increases execution time by ~75%. We might be able to get away with executing this only when a solving attempt results in no changes
fn advanced_possibility_removal(fields: Vec<FieldWithIndex>) -> Vec<FieldWithIndex> {
    let possibilities_map = &mut HashMap::new();

    for field in fields.iter() {
        let key = field.field().possibilities().clone();
        possibilities_map
            .entry(key)
            .and_modify(|p| *p += 1)
            .or_insert(1);
    }

    let mut fields_to_update = vec![];
    for field in fields.into_iter() {
        // Only the possibilities where the pair size matches the possibility size should be removed
        // This prevents that for example a 2,3,4 triplet is removed because 2,3,4 is only found twice instead of the
        // required three times
        let possibility_pairs_to_remove: Vec<&Vec<i32>> = possibilities_map
            .iter()
            .filter(|(possibilities, count)| possibilities.len().eq(count))
            .map(|(possibilities, _)| possibilities)
            .collect();

        if possibility_pairs_to_remove.contains(&&field.field().possibilities().clone()) {
            fields_to_update.push(field);
        } else {
            /*
            Since the possibility pairs are vectors in vectors the new possibilities are basically all digits that are in are not in any of the
            possibility pairs to be removed. Example:
            If the pairs to remove are [1,2] and [3,4] the possibilities [1,5,6] would get the 1 removed as it is contained in the first pair
            */
            fields_to_update.push(FieldWithIndex::new(
                Field::empty_with_possibilities(
                    field
                        .field()
                        .possibilities()
                        .into_iter()
                        .filter(|p| {
                            possibility_pairs_to_remove
                                .iter()
                                .all(|possibility_pair_to_remove| {
                                    !possibility_pair_to_remove.contains(p)
                                })
                        })
                        .map(|p| *p)
                        .collect(),
                ),
                field.index(),
            ));
        }
    }

    fields_to_update
}

impl Grid {
    pub fn create_from_file(file_name: &str) -> Result<Grid, Error> {
        let file_content = fs::read_to_string(file_name)?;

        let mut grid = Grid::create_empty();
        let mut index: usize = 0;
        for line in file_content.lines() {
            for s in line.split_whitespace() {
                grid.set_field(
                    index,
                    match s.parse() {
                        Ok(v) => Field::new(v),
                        Err(_) => Field::empty(),
                    },
                );
                index += 1;
            }
        }

        Ok(grid)
    }
}
