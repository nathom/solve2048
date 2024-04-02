mod board;
mod expectimax;
mod monte_carlo;
mod ntuple;
mod player;
pub use board::{Board, Move};
use fastrand::Rng;
pub use monte_carlo::{MonteCarloMetric, MonteCarloPlayer};
pub use ntuple::{NTuple, NTuplePlayer};
pub use player::Player;
use wasm_bindgen::prelude::*;
use web_sys;

#[cfg(test)]
mod tests {
    use super::*;
    use board::*;

    #[ignore]
    #[test]
    fn monte_carlo() {
        play_monte_carlo(1000, 10, MonteCarloMetric::Sum);
    }

    #[test]
    fn board_add_random() {
        let mut b = Board::new();
        let mut rng = Rng::new();
        b.add_random_tile(&mut rng);
        b.add_random_tile(&mut rng);
        b.add_random_tile(&mut rng);
        b.add_random_tile(&mut rng);
        let cnt: u32 = (0..16).map(|i| (b.at(i) != 0) as u32).sum();
        assert_eq!(cnt, 4);
    }

    #[test]
    fn board_sum_zero() {
        let b = Board::new();
        assert_eq!(b.sum_tile(), 16);
    }

    #[test]
    fn board_sum() {
        let mut b = Board::new();
        b.set(0, 1);
        b.set(1, 2);
        b.set(15, 2);
        assert_eq!(b.sum_tile(), 13 + 2 + 4 + 4);
    }

    #[test]
    fn board_init_zero() {
        let b = Board::new();
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(b.get(i, j), 0);
            }
        }
    }

    #[test]
    fn board_set_row() {
        let mut b = Board::new();
        let i = 0;
        b.set_row(i, Row::from_raw(0x4321)); // little endian
        for j in 0..4 {
            assert_eq!(b.get(i, j), j + 1);
        }
    }

    #[test]
    fn board_get_row() {
        let mut b = Board::new();
        let i = 0;
        b.set_row(i, Row::from_raw(0x4321)); // little endian
        let r = b.get_row(i);
        for i in 0..4 {
            assert_eq!(r.get(i), i + 1);
        }
    }

    #[test]
    fn board_set_col() {
        let mut b = Board::from_raw(0x4312752186532731);
        let c = Row::from_raw(0x1234);
        b._set_col(2, c);
        let newcol = b._get_col(2).raw;
        assert_eq!(newcol, 0x1234);
    }

    #[test]
    fn set_row() {
        let mut r = Row::new();
        for i in 0..4 {
            r.set(i, i);
        }
        for i in 0..4 {
            assert_eq!(r.get(i), i);
        }
    }

    #[test]
    fn set_row_in_board() {
        let mut r = Row::new();
        for i in 0..4 {
            r.set(i, i);
        }
        let mut b = Board::new();
        b.set_row(2, r);
        for i in 0..4 {
            assert_eq!(b.get(2, i), i);
        }
    }

    #[test]
    fn rotate_board_counter() {
        let mut b = Board::new();
        for i in 0..16 {
            b.set(i, i);
        }
        b.counterclockwise();

        let correct = vec![3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13, 0, 4, 8, 12];
        for i in 0..16 {
            assert_eq!(b.at(i), correct[i as usize]);
        }
    }

    #[test]
    fn rotate_board_clock() {
        let mut b = Board::new();
        for i in 0..16 {
            b.set(i, i);
        }
        b.clockwise();

        let correct = vec![12, 8, 4, 0, 13, 9, 5, 1, 14, 10, 6, 2, 15, 11, 7, 3];
        for i in 0..16 {
            assert_eq!(b.at(i), correct[i as usize]);
        }
    }

    #[test]
    fn move_left() {
        let init_raw = 0x1000011011000000;
        let exp_raw = 0x1000200020000;

        let mut b = Board::from_raw(init_raw);
        b.move_left();
        assert_eq!(b.raw, exp_raw);
    }
    #[test]
    fn row_reverse() {
        let init_raw = 0x1234;
        let exp_raw = 0x4321;

        let row = Row::from_raw(init_raw);
        println!("init board: {row}");
        println!("final board: {row}");
        assert_eq!(row.reverse().raw, exp_raw);
    }

    #[test]
    fn move_right() {
        let init_raw = 0x1000011111000101;
        let exp_raw = 0x1000210020002000;

        let mut b = Board::from_raw(init_raw);
        println!("init board: {b}");
        b.move_right();
        println!("final board: {b}");
        assert_eq!(b.raw, exp_raw);
    }

    #[test]
    fn move_up() {
        let init_raw = 0x1000011111000101;
        let exp_raw = 0x1002212;

        let mut b = Board::from_raw(init_raw);
        println!("init board: {b}");
        b.move_up();
        println!("final board: {b}");
        assert_eq!(b.raw, exp_raw);
    }

    #[test]
    fn move_down() {
        let init_raw = 0x1000011111000101;
        let exp_raw = 0x2212010000000000;
        let mut b = Board::from_raw(init_raw);
        println!("init board: {b}");
        b.move_down();
        println!("final board: {b}");
        assert_eq!(b.raw, exp_raw);
    }
}

pub fn play_game<P: Player>(player: &P) -> (u32, u32) {
    let mut b = Board::new();
    let mut rng = Rng::new();
    b.add_random_tile(&mut rng);
    b.add_random_tile(&mut rng);

    let mut score = 0;
    loop {
        let m = match player.next_move(&b) {
            Some(mv) => mv,
            None => break,
        };

        if let Some(s) = b.make_move(m) {
            score += s;
            b.add_random_tile(&mut rng);
        }
    }
    let max_tile = b.max_tile();
    return (score, max_tile);
}

pub fn play_monte_carlo(niter: u32, ngames: u32, metric: MonteCarloMetric) {
    let player = MonteCarloPlayer::new(niter, metric);

    for i in 0..ngames {
        let (score, max) = play_game(&player);
        println!("Game {i}: Score: {score} Max: {max}");
    }
}

#[wasm_bindgen]
pub fn get_ntuple_net_from_js_blob(blob: web_sys::Blob) -> NTuple {
    let reader = web_sys::FileReader::new().unwrap();
    reader.read_as_array_buffer(&blob).unwrap();
    let ntuple = NTuple::load(reader.result().unwrap().as_ref().unwrap());
    ntuple
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn monte_carlo(arr: &[i32]) -> i32 {
    let b = Board::from_arr(arr);
    let next_move = MonteCarloPlayer::default().next_move(&b);
    next_move.unwrap().to_int()
}

#[wasm_bindgen]
pub fn play_game_ntuple() {
    let weights_url = "https://huggingface.co/nathom/ntuple-2048/resolve/main/tuplenet_4M_lr.bin";
    let network = NTuple::load(weights_url);
    let player = NTuplePlayer::new();
    let (score, max) = play_game(&player);
    alert(&format!("Score: {} Max: {}", score, max));
}
