use crate::{Board, Heuristic, Move};
pub trait Player {
    fn next_move(&self, b: &Board, heur: &impl Heuristic) -> Option<Move>;
}
