use std::vec;

#[derive(Debug, Clone)]
pub struct Field {
    value: Option<i32>,
    possibilities: Vec<i32>,
}

impl Field {
    pub fn new(val: i32) -> Field {
        if val < 1 || val > 9 {
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

    pub fn with_possibilities(&self, possibilities: Vec<i32>) -> Result<Field, String> {
        if self.value.is_some() {
            return Err(String::from(
                "Field already has value. Setting possitibilities is forbidden",
            ));
        }

        Ok(Field {
            value: Option::None,
            possibilities,
        })
    }

    pub fn update(&mut self, new_val: i32) {
        self.value = Option::Some(new_val);
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    pub fn possibilities(&self) -> &Vec<i32> {
        return &self.possibilities;
    }

    pub fn value(&self) -> Option<i32> {
        return self.value;
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

    pub fn get_field(&self, row: usize, column: usize) -> Option<&Field> {
        return self
            .fields
            .get(Grid::grid_index_by_row_and_column(row, column));
    }

    pub fn set_field(&mut self, row: usize, column: usize, field: Field) {
        let index = Grid::grid_index_by_row_and_column(row, column);
        self.fields.splice(index..index + 1, vec![field]);
    }

    fn grid_index_by_row_and_column(row: usize, column: usize) -> usize {
        return row * 9 + column;
    }

    pub fn is_solved(&self) -> bool {
        for field in &self.fields {
            if field.is_empty() {
                return false;
            }
        }

        return true;
    }

    pub fn get_fields_in_row(&self, row: usize) -> Result<Vec<&Field>, String> {
        let mut fields = vec![];

        for i in 0..9 {
            fields.push(match self.get_field(row, i) {
                Some(field) => field,
                None => return Err(String::from("Cannot find field")),
            });
        }

        Ok(fields)
    }

    pub fn get_fields_in_column(&self, column: usize) -> Result<Vec<&Field>, String> {
        let mut fields = vec![];
        for i in 0..9 {
            fields.push(match self.get_field(i, column) {
                Some(field) => field,
                None => return Err(String::from("Cannot find field")),
            });
        }

        Ok(fields)
    }
}

pub trait Printable {
    fn print(&self);
}

impl Printable for Grid {
    fn print(&self) {
        let rows = self.fields.chunks(9);
        for (i, row) in rows.enumerate() {
            println!(
                "{}: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                i,
                row[0].value.unwrap_or(0),
                row[1].value.unwrap_or(0),
                row[2].value.unwrap_or(0),
                row[3].value.unwrap_or(0),
                row[4].value.unwrap_or(0),
                row[5].value.unwrap_or(0),
                row[6].value.unwrap_or(0),
                row[7].value.unwrap_or(0),
                row[8].value.unwrap_or(0)
            )
        }
    }
}
