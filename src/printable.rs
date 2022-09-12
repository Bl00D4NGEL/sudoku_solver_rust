use crate::sudoku::Grid;

pub trait Printable {
    fn print(&self);
}

impl Printable for Grid {
    fn print(&self) {
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
    }
}
