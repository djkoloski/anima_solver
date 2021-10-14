use anima_solver::{solve, State};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn solve_free_radical(c: &mut Criterion) {
    const SQUARE_DANCE: &str = " ....\n.r.r.\n.. ..\n.r.r.\n.... \n\nR 2 1\nR 1 2\nR 3 2\nR 2 3";

    let (initial_state, data) = State::parse(SQUARE_DANCE).unwrap();

    c.bench_function("solve_square_dance", |b| {
        b.iter(|| solve(black_box(&initial_state).clone(), &data))
    });

    const FRACTAL: &str =
        "  r  \n ... \n. b .\n.....\n. r .\n ... \n  b  \n\nR 1 3\nR 3 3\nB 2 2\nB 2 4";

    let (initial_state, data) = State::parse(FRACTAL).unwrap();

    c.bench_function("solve_fractal", |b| {
        b.iter(|| solve(black_box(&initial_state).clone(), &data))
    });

    const ANTIPARTICLE: &str =
        ".....\n.r.r.\n.....\n.r.r.\n.....\n\nR 1 2\nR 2 1\nB 2 2\nR 2 3\nR 3 2";

    let (initial_state, data) = State::parse(ANTIPARTICLE).unwrap();

    c.bench_function("solve_antiparticle", |b| {
        b.iter(|| solve(black_box(&initial_state).clone(), &data))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    solve_free_radical(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
