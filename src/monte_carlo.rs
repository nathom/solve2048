use crate::{Board, Heuristic, Move, Player};
use fastrand::Rng;
// use rayon::prelude::*;

#[derive(Clone)]
pub enum MonteCarloMetric {
    Sum,
    MaxTile,
    Score,
    Moves,
}

#[derive(Clone)]
pub struct MonteCarloPlayer {
    niter: u32,
    metric: MonteCarloMetric,
}

impl Player for MonteCarloPlayer {
    fn next_move(&self, b: &Board, _: &impl Heuristic) -> Option<Move> {
        let res = Move::all()
            .iter()
            .map(|&m| (m, self.explore_move(b, m)))
            .filter(|&(_, s)| s > 0)
            .max_by_key(|&(_, s)| s)?;

        let (mv, _) = res;
        Some(mv)
    }
}

impl MonteCarloPlayer {
    pub fn new(niter: u32, metric: MonteCarloMetric) -> Self {
        Self { niter, metric }
    }

    pub fn default() -> Self {
        Self {
            niter: 200,
            metric: MonteCarloMetric::Sum,
        }
    }

    pub fn explore_move(&self, b: &Board, m: Move) -> u32 {
        let mut b: Board = *b;
        let mut score: u32 = match b.make_move(m) {
            Some(s) => s,
            None => return 0,
        };
        let mut rng = Rng::new();
        b.add_random_tile(&mut rng);

        score += (0..self.niter)
            // .into_par_iter()
            .map(|_| self.random_run(&b))
            .sum::<u32>();

        score
    }
    pub fn random_run(&self, b: &Board) -> u32 {
        let mut b: Board = *b;
        let mut nmoves = 0;
        let mut score = 0;
        let mut fails = 0;
        let mut rng = Rng::new();
        // approximation of !game_ended
        while fails < 4 {
            if let Some(delta) = b.make_move(Move::rand(&mut rng)) {
                score += delta;
                nmoves += 1;
                b.add_random_tile(&mut rng);
                fails = 0;
            } else {
                fails += 1;
            }
        }
        match self.metric {
            MonteCarloMetric::Sum => b.sum_tile(),
            MonteCarloMetric::MaxTile => b.max_tile(),
            MonteCarloMetric::Score => score,
            MonteCarloMetric::Moves => nmoves,
        }
    }
}
