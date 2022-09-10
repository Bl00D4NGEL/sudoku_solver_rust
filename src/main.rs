fn main() {
    let fields = [
        Field::new(2),
        Field::new(3),
        Field::new(4),
        Field::new(5),
        Field::new(6),
        Field::new(7),
        Field::empty(),
        Field::new(8),
        Field::new(9),
    ];
    let row = Row::new(fields);
    let mutated_row = run(row);

    dbg!(mutated_row);
}

#[derive(Debug, Clone)]
struct Field {
    value: Option<i32>,
}

impl Field {
    fn new(val: i32) -> Field {
        Field {
            value: Option::Some(val),
        }
    }

    fn empty() -> Field {
        Field {
            value: Option::None,
        }
    }

    fn update(&mut self, new_val: i32) {
        self.value = Option::Some(new_val);
    }

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

#[derive(Debug, Clone)]
struct Row {
    fields: [Field; 9],
}

impl Row {
    fn new(fields: [Field; 9]) -> Row {
        Row { fields }
    }

    fn get_possibilities(&self) -> Vec<i32> {
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
}

fn run(row: Row) -> Row {
    let possibilities = row.get_possibilities();
    if possibilities.len() != 1 {
        return row;
    }

    let mut fields = row.fields;
    for field in &mut fields {
        if field.is_empty() {
            field.update(possibilities[0])
        }
    }
    return Row::new(fields);
}
