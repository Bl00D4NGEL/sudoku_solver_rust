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
}

#[derive(Debug, Clone)]
pub struct Row {
    fields: Vec<Field>,
}

impl Row {
    pub fn new(fields: Vec<Field>) -> Row {
        if fields.len() != 9 {
            panic!("Row must have exactly 9 fields");
        }
        Row { fields }
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn empty_fields(&self) -> Vec<&Field> {
        self.fields.iter().filter(|f| f.is_empty()).collect()
    }

    pub fn update_possibilities(&mut self) {
        let used_digits: Vec<i32> = self
            .fields
            .iter()
            .map(|f| f.value.unwrap_or(0))
            .filter(|v| v.ge(&0))
            .collect();

        let mut possible_digits = vec![];

        for digit in 1..=9 {
            if !used_digits.contains(&digit) {
                possible_digits.push(digit);
            }
        }

        self.fields = self
            .fields
            .clone()
            .into_iter()
            .map(|f| {
                if !f.is_empty() {
                    return f;
                }

                let f = Field::empty();
                return f
                    .with_possibilities(possible_digits.clone())
                    .expect("Field is empty so the call to with_possibilities should not fail");
            })
            .collect();
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    pub fn new(rows: Vec<Row>) -> Grid {
        if rows.len() != 9 {
            panic!("Grid must have exactly 9 rows");
        }
        Grid { rows }
    }

    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    pub fn columns(&self) -> Vec<Row> {
        let mut rows: Vec<Row> = vec![];
        for i in 0..=8 {
            let fields: Vec<Field> = self
                .rows()
                .iter()
                .map(|r| r.fields().get(i).expect("Field must exist.").clone())
                .collect();
            rows.push(Row::new(fields));
        }

        rows
    }

    pub fn print(&self) {
        for (i, row) in self.rows().iter().enumerate() {
            println!(
                "{}: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                i,
                row.fields()[0].value.unwrap_or(0),
                row.fields()[1].value.unwrap_or(0),
                row.fields()[2].value.unwrap_or(0),
                row.fields()[3].value.unwrap_or(0),
                row.fields()[4].value.unwrap_or(0),
                row.fields()[5].value.unwrap_or(0),
                row.fields()[6].value.unwrap_or(0),
                row.fields()[7].value.unwrap_or(0),
                row.fields()[8].value.unwrap_or(0)
            )
        }
    }

    pub fn is_valid() -> bool {
        true
    }
}
