use criterion::{criterion_group, criterion_main, Criterion};
use solve2048::{Board, MonteCarloPlayer, Move};

fn move_left_benchmark(c: &mut Criterion) {
    let mut brd = Board::new();
    let mut rng = rand::thread_rng();
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);

    c.bench_function("board move left", |b| {
        b.iter(|| {
            brd.clone().move_left();
        })
    });
}

fn move_right_benchmark(c: &mut Criterion) {
    let mut brd = Board::new();
    let mut rng = rand::thread_rng();
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);

    c.bench_function("board move right", |b| {
        b.iter(|| {
            brd.clone().move_right();
        })
    });
}

fn move_up_benchmark(c: &mut Criterion) {
    let mut brd = Board::new();
    let mut rng = rand::thread_rng();
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);

    c.bench_function("board move up", |b| {
        b.iter(|| {
            brd.clone().move_up();
        })
    });
}

fn move_down_benchmark(c: &mut Criterion) {
    let mut brd = Board::new();
    let mut rng = rand::thread_rng();
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);

    c.bench_function("board move down", |b| {
        b.iter(|| {
            brd.clone().move_down();
        })
    });
}

fn random_move_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("generate random move", |b| {
        b.iter(|| {
            Move::rand(&mut rng);
        })
    });
}

fn random_run_benchmark(c: &mut Criterion) {
    let mut brd = Board::new();
    let mut rng = rand::thread_rng();
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    brd.add_random_tile(&mut rng);
    let player = MonteCarloPlayer::default();
    c.bench_function("random game run", |b| {
        b.iter(|| {
            player.random_run(&brd);
        })
    });
}

// fn mc_benchmark(c: &mut Criterion) {
//     let player = MonteCarloPlayer::default();
//     c.bench_function("monte carlo single game", |b| {
//         b.iter(|| monte_carlo_single_game(&player))
//     });
// }
criterion_group!(
    benches,
    move_left_benchmark,
    move_right_benchmark,
    move_down_benchmark,
    move_up_benchmark,
    random_move_benchmark,
    random_run_benchmark,
);
criterion_main!(benches);
