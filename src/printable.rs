use crate::sudoku::{Field, FieldWithIndex, Grid};

pub trait Printable {
    fn print(&self);
}

impl Printable for Grid {
    fn print(&self) {
        if self.is_solved() {
            println!("Grid solved!");
        } else {
            println!("Grid not solved");
        }
        for (i, row) in self.fields().chunks(9).enumerate() {
            println!(
                "{}: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                i,
                row[0].value().unwrap_or(0),
                row[1].value().unwrap_or(0),
                row[2].value().unwrap_or(0),
                row[3].value().unwrap_or(0),
                row[4].value().unwrap_or(0),
                row[5].value().unwrap_or(0),
                row[6].value().unwrap_or(0),
                row[7].value().unwrap_or(0),
                row[8].value().unwrap_or(0)
            )
        }
        println!();
    }
}

impl Printable for Vec<Field> {
    fn print(&self) {
        for field in self.iter() {
            print!("{:?}", field.value().unwrap_or(0))
        }
        println!()
    }
}

impl Printable for Vec<&Field> {
    fn print(&self) {
        for field in self.iter() {
            print!("{:?} ", field.value().unwrap_or(0),)
        }
        println!()
    }
}

impl Printable for Vec<&FieldWithIndex> {
    fn print(&self) {
        for field in self.iter() {
            print!("{:?} ", field.field().value().unwrap_or(0),)
        }
        println!()
    }
}

impl Printable for Vec<FieldWithIndex> {
    fn print(&self) {
        for field in self.iter() {
            println!(
                "Val: {:?}, Possibilities: {:?}",
                field.field().value().unwrap_or(0),
                field.field().possibilities()
            );
        }
    }
}

impl Printable for FieldWithIndex {
    fn print(&self) {
        if self.field().value().is_some() {
            println!(
                "Field at index {} has value {}",
                self.index(),
                self.field().value().unwrap_or(0)
            )
        } else {
            println!(
                "Field at index {} has possibilities {:?}",
                self.index(),
                self.field().possibilities()
            )
        }
    }
}
