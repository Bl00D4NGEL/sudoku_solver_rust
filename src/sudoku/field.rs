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
