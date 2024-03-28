use crate::{Board, Move};
pub trait Player {
    fn next_move(&self, b: &Board) -> Option<Move>;
}
