use criterion::{criterion_group, criterion_main, Criterion};
use fastrand::Rng;
use solve2048::{Board, ExpectimaxPlayer, HeuristicScoreCache, MonteCarloPlayer, Move, Player};

fn move_left_benchmark(c: &mut Criterion) {
    let mut brd = Board::new();
    let mut rng = Rng::new();
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
    let mut rng = Rng::new();
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
    let mut rng = Rng::new();
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
    let mut rng = Rng::new();
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
    let mut rng = Rng::new();
    c.bench_function("generate random move", |b| {
        b.iter(|| {
            Move::rand(&mut rng);
        })
    });
}

fn random_run_benchmark(c: &mut Criterion) {
    let mut brd = Board::new();
    let mut rng = Rng::new();
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

fn expectimax_calculation(c: &mut Criterion) {
    let mut brd = Board::new();
    /* load board with the following values:
     * 128 4 2 0
     * 256 8 2 0
     * 512 32 8 2
     * 16384 64 8 2
     */
    brd.set(0, 7);
    brd.set(1, 2);
    brd.set(2, 1);
    brd.set(3, 0);
    brd.set(4, 8);
    brd.set(5, 3);
    brd.set(6, 1);
    brd.set(7, 0);
    brd.set(8, 9);
    brd.set(9, 5);
    brd.set(10, 3);
    brd.set(11, 1);
    brd.set(12, 14);
    brd.set(13, 6);
    brd.set(14, 3);
    brd.set(15, 1);
    let player = ExpectimaxPlayer::default();
    let cache = HeuristicScoreCache::new();
    c.bench_function("expectimax calculation", |b| {
        b.iter(|| player.next_move(&brd, &cache))
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
    expectimax_calculation,
);
criterion_main!(benches);
