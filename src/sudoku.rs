#[derive(Debug, Clone)]
pub struct Field {
    value: Option<i32>,
    possibilities: Vec<i32>,
}

impl Field {
    pub fn new(val: i32) -> Field {
        Field {
            value: Option::Some(val),
            possibilities: vec![],
        }
    }

    pub fn empty(possibilities: Option<Vec<i32>>) -> Field {
        Field {
            value: Option::None,
            possibilities: possibilities.unwrap_or(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
        }
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
    fields: [Field; 9],
}

impl Row {
    pub fn new(fields: [Field; 9]) -> Row {
        Row { fields }
    }

    pub fn fields(&self) -> &[Field; 9] {
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

        self.fields = self.fields().clone().map(|f| {
            if !f.is_empty() {
                return f;
            }

            return Field::empty(Option::Some(possible_digits.clone()));
        });
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    rows: [Row; 9],
}

impl Grid {
    pub fn new(rows: [Row; 9]) -> Grid {
        Grid { rows }
    }

    pub fn rows(&self) -> &[Row; 9] {
        &self.rows
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
}
