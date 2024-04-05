use crate::{Board, Move, Player};
use std::io::{Read, Write};
use std::mem::size_of;
use wasm_bindgen::prelude::*;

pub struct Feature {
    weights: Vec<f32>,
    iso: [Vec<u8>; 8],
}

pub struct MoveRecord {
    pub m: Move,
    pub board_after: Board,
    pub score: u32,
}

impl Feature {
    /// Create a new feature with the given pattern and zero weights
    pub fn new(pattern: &[u8]) -> Self {
        let weights = vec![0.0; 1 << (pattern.len() * 4)];
        let iso: [Vec<u8>; 8] = Self::isometries(pattern);
        Feature { weights, iso }
    }

    pub fn with_weights(pattern: &[u8], bytes: &mut impl Read) -> Self {
        let mut feat = Self::new(pattern);
        feat.load_weights(bytes);
        feat
    }

    pub fn save_weights(&self, bytes: &mut impl Write) {
        let name = self.name();
        let name_size = name.len() as i32;
        bytes.write(&name_size.to_le_bytes()).unwrap();
        bytes.write(name.as_bytes()).unwrap();
        let num_weights = self.weights.len() as u64;
        bytes.write(&num_weights.to_le_bytes()).unwrap();
        for w in &self.weights {
            bytes.write(&w.to_le_bytes()).unwrap();
        }
    }

    pub fn load_weights(&mut self, bytes: &mut impl Read) {
        // 1 name size
        let mut buf = [0; size_of::<i32>()];
        bytes.read_exact(&mut buf).unwrap();
        let name_size = i32::from_le_bytes(buf);
        // read `name_size` bytes to buf
        let mut buf = vec![0 as u8; name_size as usize];
        bytes.read_exact(&mut buf).unwrap();
        // convert bytes to string
        let name = String::from_utf8(buf).unwrap();
        if name != self.name() {
            panic!(
                "Invalid feature name (len={}): {} != {}",
                name_size,
                name,
                self.name()
            );
        }
        // read number of weights
        let mut buf = [0; size_of::<u64>()];
        // weight size
        bytes.read_exact(&mut buf).unwrap();
        let length = u64::from_le_bytes(buf) as usize;
        assert!(length == self.weights.len());
        // read f32 weights
        let mut buf = [0; size_of::<f32>()];
        for i in 0..length {
            bytes.read_exact(&mut buf).unwrap();
            self.weights[i] = f32::from_le_bytes(buf);
        }
    }

    fn isometries(pattern: &[u8]) -> [Vec<u8>; 8] {
        let mut iso: [Vec<u8>; 8] = Default::default();
        for i in 0..8 {
            iso[i].reserve(pattern.len());
        }
        // 8 isometries: 4 rotated states * 2 flipped states
        for i in 0..8 {
            // board with tiles the same as index
            let mut b = Board::from_raw(0xfedcba9876543210);
            if i >= 4 {
                b.flip_horizontal();
            }
            b.rotate(i as u32);
            for &t in pattern {
                iso[i].push(b.at(t));
            }
        }
        iso
    }

    fn estimate(&self, b: &Board) -> f32 {
        return self
            .iso
            .iter()
            .map(|p| self.weights[self.indexof(p, b)])
            .sum::<f32>();
    }

    fn indexof(&self, pattern: &Vec<u8>, b: &Board) -> usize {
        let mut index = 0;
        for i in 0..pattern.len() {
            index |= (b.at(pattern[i]) as usize) << (4 * i);
        }
        return index as usize;
    }

    fn update(&mut self, b: &Board, delta: f32) -> f32 {
        let delta = delta / self.iso.len() as f32;
        let mut value = 0.0;
        for i in 0..8 {
            let index = self.indexof(&self.iso[i], b);
            self.weights[index] += delta;
            value += self.weights[index];
        }
        return value;
    }

    fn name(&self) -> String {
        let mut s = String::new();
        for i in &self.iso[0] {
            s.push_str(&format!("{:x}", i));
        }
        format!("{}-tuple pattern {}", self.iso[0].len(), s)
    }
}

#[wasm_bindgen]
pub struct NTuple {
    feats: Vec<Feature>,
}

impl Default for NTuple {
    fn default() -> Self {
        let mut feats = Vec::with_capacity(4);
        for pattern in [
            &[0, 1, 2, 3, 4, 5],
            &[4, 5, 6, 7, 8, 9],
            &[0, 1, 2, 4, 5, 6],
            &[4, 5, 6, 8, 9, 10],
        ] {
            feats.push(Feature::new(pattern));
        }
        NTuple::new(feats)
    }
}

impl Player for NTuple {
    fn next_move(&self, b: &Board) -> Option<Move> {
        return self.select_best_move(b);
    }
}

impl NTuple {
    pub fn load(patterns: &[&[u8]], bytes: &mut impl Read) -> Self {
        let mut buf = [0; size_of::<u64>()];
        bytes.read_exact(&mut buf).unwrap();
        let size = u64::from_le_bytes(buf) as usize;
        if size != patterns.len() {
            panic!("Invalid size: {} != {}", size, patterns.len() as u32);
        }
        let mut feats = Vec::with_capacity(size as usize);
        for i in 0..size {
            feats.push(Feature::with_weights(patterns[i], bytes));
        }
        NTuple::new(feats)
    }

    pub fn save_weights(&self, out: &mut impl Write) {
        let size = self.feats.len();
        out.write(&size.to_le_bytes()).unwrap();
        for feat in &self.feats {
            feat.save_weights(out);
        }
        out.flush().unwrap();
    }

    pub fn new(feats: Vec<Feature>) -> Self {
        NTuple { feats }
    }

    pub fn load_weights(&mut self, bytes: &mut impl Read) {
        let mut buf = [0; size_of::<u64>()];
        bytes.read(&mut buf).unwrap();
        let size = u64::from_le_bytes(buf) as usize;
        println!("Size of network {size}");

        for feat in &mut self.feats {
            feat.load_weights(bytes);
        }
    }

    pub fn estimate(&self, b: &Board) -> f32 {
        // sum of all feature estimates
        return self.feats.iter().map(|f| f.estimate(b)).sum::<f32>();
    }

    pub fn update(&mut self, b: &Board, delta: f32) -> f32 {
        let delta = delta / self.feats.len() as f32;
        let value = self
            .feats
            .iter_mut()
            .map(|f| f.update(b, delta))
            .sum::<f32>();
        return value;
    }

    fn select_best_move(&self, b: &Board) -> Option<Move> {
        let moves = Move::all();
        let mut best_move = Move::Up;
        // minimum f32 value
        let mut best_score = f32::MIN;

        for mv in moves {
            let mut b_copy = *b;
            if let Some(score) = b_copy.make_move(mv) {
                let score = score as f32 + self.estimate(&b_copy);
                if score > best_score {
                    best_score = score;
                    best_move = mv;
                }
            }
        }

        if best_score == f32::MIN {
            return None;
        }
        return Some(best_move);
    }

    pub fn backward(&mut self, path: &mut Vec<MoveRecord>, alpha: f32) {
        let mut target = 0.0;
        path.pop();
        for mv in path.iter().rev() {
            let err = target - self.estimate(&mv.board_after);
            target = mv.score as f32 + self.update(&mv.board_after, alpha * err);
        }
    }
}
