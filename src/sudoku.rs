use std::{collections::HashMap, fs, io::Error, vec};

#[derive(Debug)]
pub struct Field {
    value: Option<i32>,
    possibilities: Vec<i32>,
    index: usize,
}

impl Field {
    pub fn new(val: i32, index: usize) -> Field {
        if !(1..=9).contains(&val) {
            panic!("Value for field must be between 1 and 9, got {}", val)
        }

        Field {
            value: Option::Some(val),
            possibilities: vec![],
            index,
        }
    }

    pub fn empty(index: usize) -> Field {
        Field::empty_with_possibilities(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], index)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn empty_with_possibilities(possibilities: Vec<i32>, index: usize) -> Field {
        if possibilities.len().eq(&0) {
            panic!("Possibilities must have at least one entry");
        }
        Field {
            value: Option::None,
            possibilities,
            index,
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

#[derive(Debug)]
pub struct Grid {
    fields: [Field; 81],
}

impl Grid {
    pub fn create_empty() -> Grid {
        let fields: [Field; 81] = [
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
            Field::empty(0),
        ];

        Grid { fields }
    }

    pub fn fields(&self) -> &[Field; 81] {
        &self.fields
    }

    pub fn set_field(&mut self, field: Field) {
        let index = field.index();
        self.fields[index] = field;
    }

    pub fn set_fields(&mut self, fields: Vec<Field>) {
        fields.into_iter().for_each(|field| {
            self.set_field(field);
        });
    }

    pub fn is_solved(&self) -> bool {
        for field in &self.fields {
            if field.is_empty() {
                return false;
            }
        }

        true
    }

    pub fn get_fields_in_row(&self, row: usize) -> [&Field; 9] {
        return [
            self.fields.get(row * 9 + 0).unwrap(),
            self.fields.get(row * 9 + 1).unwrap(),
            self.fields.get(row * 9 + 2).unwrap(),
            self.fields.get(row * 9 + 3).unwrap(),
            self.fields.get(row * 9 + 4).unwrap(),
            self.fields.get(row * 9 + 5).unwrap(),
            self.fields.get(row * 9 + 6).unwrap(),
            self.fields.get(row * 9 + 7).unwrap(),
            self.fields.get(row * 9 + 8).unwrap(),
        ];
    }

    pub fn get_fields_in_column(&self, column: usize) -> [&Field; 9] {
        return [
            self.fields.get(0 + column).unwrap(),
            self.fields.get(9 + column).unwrap(),
            self.fields.get(18 + column).unwrap(),
            self.fields.get(27 + column).unwrap(),
            self.fields.get(36 + column).unwrap(),
            self.fields.get(45 + column).unwrap(),
            self.fields.get(54 + column).unwrap(),
            self.fields.get(63 + column).unwrap(),
            self.fields.get(72 + column).unwrap(),
        ];
    }

    pub fn get_fields_in_box(&self, box_id: usize) -> [&Field; 9] {
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

        return [
            self.fields.get(box_id * 3 + 0 + extra_index).unwrap(),
            self.fields.get(box_id * 3 + 1 + extra_index).unwrap(),
            self.fields.get(box_id * 3 + 2 + extra_index).unwrap(),
            self.fields.get(box_id * 3 + 9 + extra_index).unwrap(),
            self.fields.get(box_id * 3 + 10 + extra_index).unwrap(),
            self.fields.get(box_id * 3 + 11 + extra_index).unwrap(),
            self.fields.get(box_id * 3 + 18 + extra_index).unwrap(),
            self.fields.get(box_id * 3 + 19 + extra_index).unwrap(),
            self.fields.get(box_id * 3 + 20 + extra_index).unwrap(),
        ];
    }

    pub fn update_possibilities(&mut self) {
        self.update_possibilities_in_rows();
        self.update_possibilities_in_columns();
        self.update_possibilities_in_boxes();
    }

    fn update_possibilities_in_rows(&mut self) {
        let mut to_update_fields = vec![];
        for row in 0..=8 {
            let fields = self.get_fields_in_row(row);

            to_update_fields.append(&mut calculate_new_possibilities_for_field_set(fields));
        }

        self.set_fields(to_update_fields);
    }

    fn update_possibilities_in_columns(&mut self) {
        let mut to_update_fields = vec![];
        for column in 0..=8 {
            let fields = self.get_fields_in_column(column);

            to_update_fields.append(&mut calculate_new_possibilities_for_field_set(fields));
        }

        self.set_fields(to_update_fields);
    }

    fn update_possibilities_in_boxes(&mut self) {
        let mut to_update_fields = vec![];
        for box_id in 0..=8 {
            let fields = self.get_fields_in_box(box_id);

            to_update_fields.append(&mut calculate_new_possibilities_for_field_set(fields));
        }

        self.set_fields(to_update_fields);
    }
}

fn calculate_new_possibilities_for_field_set(fields: [&Field; 9]) -> Vec<Field> {
    let mut non_empty_fields = vec![];
    let mut empty_fields = vec![];

    for field in fields.into_iter() {
        if field.is_empty() {
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
        .map(|f| f.value().unwrap_or(0))
        .filter(|v| v.gt(&0))
        .collect();

    let mut to_update_fields = vec![];
    for empty_field in empty_fields {
        let possibilities: Vec<i32> = empty_field
            .possibilities()
            .iter()
            .filter(|p| !used_digits.contains(p))
            .copied()
            .collect();

        if possibilities.is_empty() {
            continue;
        }

        to_update_fields.push(Field::empty_with_possibilities(
            possibilities,
            empty_field.index(),
        ));
    }

    advanced_possibility_removal(to_update_fields)
}

fn advanced_possibility_removal(fields: Vec<Field>) -> Vec<Field> {
    let possibilities_map = &mut HashMap::new();

    for field in fields.iter() {
        let key = field.possibilities().clone();
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
            .filter(|(possibilities, count)| possibilities.len().eq(*count))
            .map(|(possibilities, _)| possibilities)
            .collect();

        if possibility_pairs_to_remove.contains(&&field.possibilities().clone()) {
            fields_to_update.push(field);
        } else {
            /*
            Since the possibility pairs are vectors in vectors the new possibilities are basically all digits that are in are not in any of the
            possibility pairs to be removed. Example:
            If the pairs to remove are [1,2] and [3,4] the possibilities [1,5,6] would get the 1 removed as it is contained in the first pair
            */
            fields_to_update.push(Field::empty_with_possibilities(
                field
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
                field.index(),
            ));
        }
    }

    fields_to_update
}

impl Grid {
    pub fn create_from_file(file_name: &str) -> Result<Grid, Error> {
        Grid::create_from_string(&fs::read_to_string(file_name)?[..])
    }

    pub fn create_from_string(grid_str: &str) -> Result<Grid, Error> {
        let mut grid = Grid::create_empty();
        let mut index: usize = 0;
        for line in grid_str.lines() {
            for s in line.split_whitespace() {
                grid.set_field(match s.parse() {
                    Ok(v) => Field::new(v, index),
                    Err(_) => Field::empty(index),
                });
                index += 1;
            }
        }

        Ok(grid)
    }
}
