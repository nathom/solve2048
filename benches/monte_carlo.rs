use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solve2048::{monte_carlo_single_game, MonteCarloPlayer};

fn mc_benchmark(c: &mut Criterion) {
    let player = MonteCarloPlayer::default();
    c.bench_function("monte carlo single game", |b| {
        b.iter(|| monte_carlo_single_game(&player))
    });
}
criterion_group!(benches, mc_benchmark);
criterion_main!(benches);
