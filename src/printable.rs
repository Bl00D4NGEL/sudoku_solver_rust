use crate::sudoku::{Field, Grid};

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

impl Printable for Field {
    fn print(&self) {
        if self.value().is_some() {
            println!("Field has value {}", self.value().unwrap_or(0))
        } else {
            println!("Field has possibilities {:?}", self.possibilities())
        }
    }
}
