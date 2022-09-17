use std::{fs, io::Error};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku_solver::{
    solver::Solver,
    sudoku::{Field, Grid},
};

fn do_main(i: i32) {
    let mut grid = match create_grid() {
        Ok(grid) => grid,
        Err(err) => panic!("Cannot create grid: {}.", err),
    };

    let solver = Solver::new(false, false);
    solver.solve(&mut grid);
}

fn create_grid() -> Result<Grid, Error> {
    let file_content = fs::read_to_string("./grid8.txt")?;

    let mut grid = Grid::create_empty();
    for (row, line) in file_content.lines().into_iter().enumerate() {
        for (col, s) in line.split_whitespace().into_iter().enumerate() {
            grid.set_field(
                row,
                col,
                match s.parse() {
                    Ok(v) => Field::new(v),
                    Err(_) => Field::empty(),
                },
            );
        }
    }

    return Ok(grid);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("main", |b| b.iter(|| do_main(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
