use crate::Board;
pub trait Heuristic {
    fn score(&self, b: &Board) -> f32;
}

pub struct NullHeuristic;
impl Heuristic for NullHeuristic {
    fn score(&self, _b: &Board) -> f32 {
        0.0
    }
}
