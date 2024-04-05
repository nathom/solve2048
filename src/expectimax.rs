use crate::{Board, Move, Player};
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::collections::HashMap;

lazy_static! {
    static ref CACHE: HeuristicScoreCache = HeuristicScoreCache::new();
}

#[derive(Clone, Copy, Default)]
pub struct ExpectimaxPlayer {
    depth_limit: u32,
    maxdepth: u32,
    curdepth: u32,
    cache_hits: u32,
    moves_simulated: u32,
}

impl Player for ExpectimaxPlayer {
    fn next_move(&self, b: &Board) -> Option<Move> {
        let mut best_move = Move::Left;
        let mut best_score = 0.0;
        let results = Move::all()
            .par_iter()
            .map(|&m| {
                let score = self.move_score(b, m);
                (score, m)
            })
            .collect::<Vec<_>>();
        for (score, mv) in results {
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
        }
        // for m in Move::all() {
        //     let score = self.move_score(b, m);
        //     if score > best_score {
        //         best_score = score;
        //         best_move = m;
        //     }
        // }
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

    /// Returns the expected score of the given move.
    fn move_score(&self, b: &Board, m: Move) -> f32 {
        let mut self_copy = Self::default();
        self_copy.depth_limit = (b.distinct_tiles() - 2).max(3) as u32;
        // self_copy.depth_limit = 3;
        let mut map = HashMap::new();

        let mut b_copy = b.clone();
        if b_copy.make_move(m).is_some() {
            // if the move is valid, explore
            let res = self_copy.random_player_score(&b_copy, 1.0, &mut map) + 1e-6;
            println!(
                "For move {:?}: moves simulated: {} cache size: {}, cache hits: {}",
                m,
                self_copy.moves_simulated,
                map.len(),
                self_copy.cache_hits
            );
            res
        } else {
            0.0
        }
    }

    /// Computes expected value over all possible random tile placements.
    fn random_player_score(
        &mut self,
        b: &Board,
        cprob: f32,
        map: &mut HashMap<Board, (u32, f32)>,
    ) -> f32 {
        if cprob < Self::CPROB_THRESH_BASE || self.curdepth >= self.depth_limit {
            self.maxdepth = self.curdepth.max(self.maxdepth);
            return CACHE.get_score(b);
        }

        if self.curdepth < Self::CACHE_DEPTH_LIMIT {
            if let Some(&(depth, score)) = map.get(b) {
                if depth <= self.curdepth {
                    self.cache_hits += 1;
                    return score;
                }
            }
        }

        let num_empty = b.num_empty() as f32;
        let cprob = cprob / num_empty;

        // let mut res = 0.0;

        let res = (0..16)
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
                let res1 = self.best_move_player_score(&b1, cprob * 0.9, map) * 0.9;
                let res2 = self.best_move_player_score(&b2, cprob * 0.1, map) * 0.1;
                res1 + res2
            })
            .sum::<f32>()
            / num_empty;

        if self.curdepth < Self::CACHE_DEPTH_LIMIT {
            map.insert(b.clone(), (self.curdepth, res));
        }

        res
    }

    fn best_move_player_score(
        &mut self,
        b: &Board,
        cprob: f32,
        map: &mut HashMap<Board, (u32, f32)>,
    ) -> f32 {
        let mut best_score = 0.0;
        for m in Move::all() {
            let mut new_board = b.clone();
            if new_board.make_move(m).is_some() {
                self.curdepth += 1;
                let score = self.random_player_score(&new_board, cprob, map);
                self.curdepth -= 1;
                if score > best_score {
                    best_score = score;
                }
            }
            self.moves_simulated += 1;
        }
        best_score
    }
}

struct HeuristicScoreCache {
    row_score_cache: [f32; 1 << 16],
}

impl HeuristicScoreCache {
    const SCORE_LOST_PENALTY: f32 = 200000.0;
    const SCORE_MONOTONICITY_POWER: f32 = 4.0;
    const SCORE_MONOTONICITY_WEIGHT: f32 = 47.0;
    const SCORE_SUM_POWER: f32 = 3.5;
    const SCORE_SUM_WEIGHT: f32 = 11.0;
    const SCORE_MERGES_WEIGHT: f32 = 700.0;
    const SCORE_EMPTY_WEIGHT: f32 = 270.0;

    fn new() -> Self {
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
