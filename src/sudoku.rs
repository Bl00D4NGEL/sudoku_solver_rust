#[derive(Debug, Clone)]
pub struct Field {
    value: Option<i32>,
}

impl Field {
    pub fn new(val: i32) -> Field {
        Field {
            value: Option::Some(val),
        }
    }

    pub fn empty() -> Field {
        Field {
            value: Option::None,
        }
    }

    pub fn update(&mut self, new_val: i32) {
        self.value = Option::Some(new_val);
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none()
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

    pub fn get_possibilities(&self) -> Vec<i32> {
        let filled_fields: Vec<i32> = self
            .fields
            .iter()
            .map(|f| f.value.unwrap_or(0))
            .filter(|i| i.gt(&0))
            .collect();

        let mut possibilities = vec![];

        for i in 1..9 {
            if !filled_fields.contains(&i) {
                possibilities.push(i)
            }
        }

        possibilities
    }

    pub fn fields(&self) -> &[Field; 9] {
        &self.fields
    }
}
