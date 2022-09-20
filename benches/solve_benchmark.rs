use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku_solver::{solver::Solver, sudoku::Grid};

fn do_main(grid: &mut Grid) {
    let solver = Solver::new(Duration::ZERO, false);
    solver.solve(grid);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("main", |b| {
        b.iter(|| {
            let grid_content = "x 8 4 x x x x 1 3
2 x x x 3 x 6 x x 
6 x x 5 x 9 x x 2
x x 2 x x x 4 6 9
7 x x x x x x x x
x x x 2 8 x x x x
x 2 x 7 x x x x x
x x 8 x x 5 9 x 6
5 x x x 2 x 3 x 7";
            let mut grid = Grid::create_from_string(black_box(grid_content)).unwrap();
            do_main(&mut grid);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
