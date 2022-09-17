use criterion::{criterion_group, criterion_main, Criterion};
use sudoku_solver::{solver::Solver, sudoku::Grid};

fn do_main(grid: &mut Grid) {
    let solver = Solver::new(false, false);
    solver.solve(grid);
}

fn criterion_benchmark(c: &mut Criterion) {
    let grid = match Grid::create_from_file("./grid6.txt") {
        Ok(grid) => grid,
        Err(err) => panic!("Cannot create grid: {}.", err),
    };

    c.bench_function("main", |b| b.iter(|| do_main(&mut grid.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
