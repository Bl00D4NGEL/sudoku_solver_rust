use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku_solver::{solver::Solver, sudoku::Grid};

fn criterion_benchmark(c: &mut Criterion) {
    let solver = Solver::new(Duration::ZERO, false);
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
            solver.solve(&mut grid);
        })
    });

    c.bench_function("update_possibilities", |b| {
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

            grid.update_possibilities();
        });
    });
    c.bench_function("is_solved", |b| {
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
            let grid = Grid::create_from_string(black_box(grid_content)).unwrap();

            grid.is_solved();
        });
    });
    c.bench_function("get_fields_in_box", |b| {
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
            let grid = Grid::create_from_string(black_box(grid_content)).unwrap();

            grid.get_fields_in_box(black_box(0));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
