use crate::{Board, Heuristic, Move, Player};
use hashlru::SyncCache;
use lazy_static::lazy_static;
use rayon::prelude::*;

lazy_static! {
    // static ref CACHE: HeuristicScoreCache = HeuristicScoreCache::new();
    static ref SYNC: SyncCache<Board, (u32, f32)> = SyncCache::new(10_000);
}

#[derive(Clone)]
pub struct ExpectimaxPlayer {
    ply: Option<u8>,
}

impl Default for ExpectimaxPlayer {
    fn default() -> Self {
        ExpectimaxPlayer { ply: None }
    }
}

impl Player for ExpectimaxPlayer {
    fn next_move(&self, b: &Board, heur: &impl Heuristic) -> Option<Move> {
        let mut best_move = Move::Left;
        let mut best_score = 0.0;
        let results = Move::all()
            .iter()
            // .par_iter()
            .map(|&m| {
                let score = self.move_score(b, m, heur);
                (score, m)
            })
            .collect::<Vec<_>>();
        for (score, mv) in results {
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
        }
        if best_score == 0.0 {
            None
        } else {
            Some(best_move)
        }
    }
}

impl ExpectimaxPlayer {
    /// Dont recurse if the probability of a new tile is below this threshold.
    const CPROB_THRESH_BASE: f32 = 0.0001;
    /// Max depth of cached nodes to avoid excessive memory usage.
    const CACHE_DEPTH_LIMIT: u32 = 15;

    pub fn new(ply: Option<u8>) -> ExpectimaxPlayer {
        ExpectimaxPlayer { ply }
    }

    /// Returns the expected score of the given move.
    pub fn move_score(&self, b: &Board, m: Move, heur: &impl Heuristic) -> f32 {
        let depth_limit = if let Some(ply) = self.ply {
            ply as u32
        } else {
            (b.distinct_tiles() - 2).max(3) as u32
        };

        let mut b_copy = b.clone();
        if b_copy.make_move(m).is_some() {
            // if the move is valid, explore
            let res = self.random_player_score(heur, &b_copy, 1.0, 0, depth_limit) + 1e-6;
            // println!(
            //     "For move {:?}: moves simulated: {} cache size: {}, cache hits: {}",
            //     m,
            //     self_copy.moves_simulated,
            //     map.len(),
            //     self_copy.cache_hits
            // );
            res
        } else {
            0.0
        }
    }

    /// Computes expected value over all possible random tile placements.
    fn random_player_score(
        &self,
        heur: &impl Heuristic,
        b: &Board,
        cprob: f32,
        depth: u32,
        depth_limit: u32,
    ) -> f32 {
        if cprob < Self::CPROB_THRESH_BASE || depth >= depth_limit {
            // self.maxdepth = self.curdepth.max(self.maxdepth);
            return heur.score(b);
        }

        if let Some((cdepth, score)) = SYNC.get(b) {
            return score;
        }

        let num_empty = b.num_empty() as f32;
        let cprob = cprob / num_empty;

        let avg_score = (0..16)
            // .into_par_iter()
            .into_iter()
            .filter(|&i| b.at(i) == 0)
            .map(|i| {
                let mut b1 = b.clone();
                b1.set(i, 1);
                let mut b2 = b.clone();
                b2.set(i, 2);
                (b1, b2)
            })
            .map(|(b1, b2)| {
                let res1 =
                    self.best_move_player_score(heur, &b1, cprob * 0.9, depth + 1, depth_limit)
                        * 0.9;
                let res2 =
                    self.best_move_player_score(heur, &b2, cprob * 0.1, depth + 1, depth_limit)
                        * 0.1;
                res1 + res2
            })
            .sum::<f32>()
            / num_empty;

        SYNC.insert(b.clone(), (depth, avg_score));

        avg_score
    }

    fn best_move_player_score(
        &self,
        heur: &impl Heuristic,
        b: &Board,
        cprob: f32,
        depth: u32,
        depth_limit: u32,
    ) -> f32 {
        let mut best_score = 0.0;
        for m in Move::all() {
            let mut new_board = b.clone();
            if new_board.make_move(m).is_some() {
                // self.curdepth += 1;
                let score =
                    self.random_player_score(heur, &new_board, cprob, depth + 1, depth_limit);
                // self.curdepth -= 1;
                if score > best_score {
                    best_score = score;
                }
            }
            // self.moves_simulated += 1;
        }
        best_score
    }
}

pub struct HeuristicScoreCache {
    row_score_cache: [f32; 1 << 16],
}

impl Heuristic for HeuristicScoreCache {
    fn score(&self, b: &Board) -> f32 {
        self.get_score(b)
    }
}

impl HeuristicScoreCache {
    const SCORE_LOST_PENALTY: f32 = 200000.0;
    const SCORE_MONOTONICITY_POWER: f32 = 4.0;
    const SCORE_MONOTONICITY_WEIGHT: f32 = 47.0;
    const SCORE_SUM_POWER: f32 = 3.5;
    const SCORE_SUM_WEIGHT: f32 = 11.0;
    const SCORE_MERGES_WEIGHT: f32 = 700.0;
    const SCORE_EMPTY_WEIGHT: f32 = 270.0;

    pub fn new() -> Self {
        let mut cache = [0.0; 1 << 16];
        for i in 0..1u64 << 16 {
            let mut line = [0 as u8; 4];
            for j in 0..4 {
                line[j] = ((i >> (4 * j)) & 0xf) as u8;
            }
            cache[i as usize] = Self::compute_row_score(&line);
        }
        Self {
            row_score_cache: cache,
        }
    }

    fn compute_row_score(line: &[u8; 4]) -> f32 {
        let mut sum = 0.0;
        let mut empty = 0;
        let mut merges = 0;

        let mut prev = 0.0;
        let mut counter = 0;
        for i in 0..4 {
            let rank = line[i] as f32;
            sum += rank.powf(Self::SCORE_SUM_POWER);
            if rank == 0.0 {
                // count empty cells
                empty += 1;
            } else {
                // count number of possible merges
                if prev == rank {
                    counter += 1;
                } else if counter > 0 {
                    merges += 1 + counter;
                    counter = 0;
                }
                prev = rank;
            }
        }
        if counter > 0 {
            merges += 1 + counter;
        }

        let mut monotonicity_left = 0.0;
        let mut monotonicity_right = 0.0;
        for i in 1..4 {
            let prev = line[i - 1] as f32;
            let next = line[i] as f32;
            if prev > next {
                monotonicity_left += prev.powf(Self::SCORE_MONOTONICITY_POWER)
                    - next.powf(Self::SCORE_MONOTONICITY_POWER);
            } else {
                monotonicity_right += next.powf(Self::SCORE_MONOTONICITY_POWER)
                    - prev.powf(Self::SCORE_MONOTONICITY_POWER);
            }
        }

        return Self::SCORE_LOST_PENALTY
            + Self::SCORE_EMPTY_WEIGHT * empty as f32
            + Self::SCORE_MERGES_WEIGHT * merges as f32
            - Self::SCORE_MONOTONICITY_WEIGHT * monotonicity_left.min(monotonicity_right)
            - Self::SCORE_SUM_WEIGHT * sum;
    }

    fn get_score(&self, b: &Board) -> f32 {
        let mut b_trans = b.clone();
        b_trans.transpose();
        let b_trans = b_trans;

        let mut score = 0.0;
        for row in 0..4 {
            score += self.row_score_cache[b.get_row(row).raw as usize];
            score += self.row_score_cache[b_trans.get_row(row).raw as usize];
        }
        score
    }
}
