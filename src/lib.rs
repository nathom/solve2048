mod board;
mod expectimax;
mod heuristic;
mod monte_carlo;
mod ntuple;
mod player;

use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::sync::Mutex;

use fastrand::Rng;
use lazy_static::lazy_static;
use rayon::prelude::*;
use wasm_bindgen::prelude::*;

pub use board::{Board, Move};
pub use expectimax::{ExpectimaxPlayer, HeuristicScoreCache};
pub use heuristic::{Heuristic, NullHeuristic};
pub use monte_carlo::{MonteCarloMetric, MonteCarloPlayer};
pub use ntuple::{Feature, MoveRecord, NTuple};
pub use player::Player;
// pub use wasm_bindgen_rayon::init_thread_pool;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

use std::time::{Duration, Instant};

pub fn play_game<P: Player>(
    player: &P,
    max_moves: u32,
    show_moves: bool,
    heur: &impl Heuristic,
) -> (u32, u32) {
    let mut b = Board::new();
    let mut rng = Rng::new();
    b.add_random_tile(&mut rng);
    b.add_random_tile(&mut rng);

    let mut score = 0;
    let mut total_moves = 0;
    let mut total_time = Duration::new(0, 0);

    loop {
        let start_time = Instant::now();

        let m = match player.next_move(&b, heur) {
            Some(mv) => mv,
            None => break,
        };

        let move_time = start_time.elapsed();
        total_time += move_time;
        total_moves += 1;

        if let Some(s) = b.make_move(m) {
            score += s;
            b.add_random_tile(&mut rng);
        }

        if show_moves {
            let move_time = move_time.as_secs_f64();
            println!("{b}\n{move_time:.2} s");
        }

        if total_moves >= max_moves {
            break;
        }
    }
    let max_tile = b.max_tile();
    let average_time_per_move = total_time.as_secs_f64() / total_moves as f64;

    (score, max_tile)
}

pub fn play_monte_carlo(niter: u32, ngames: u32, metric: MonteCarloMetric) {
    let player = MonteCarloPlayer::new(niter, metric.clone());

    for i in 0..ngames {
        let (score, max) = play_game(&player, u32::max_value(), false, &NullHeuristic {});
        println!("Game {i}: Score: {score} Max: {max}");
    }
}

#[derive(Clone, Default)]
struct Stats {
    score_total: u32,
    max_tile_total: u32,
    moves_total: u32,
    tile_counter: [u32; 16],
}

pub fn tdl_fine_tune_expectimax(load_path: &str, save_path: &str, alpha: f32, ngames: u32) {
    let mut net = NTuple::default();
    let mut weight_file = File::open(load_path).unwrap();
    let mut reader = BufReader::new(&mut weight_file);
    net.load_weights(&mut reader);
    let player = ExpectimaxPlayer::new(Some(3));

    let mut stats: Mutex<Stats> = Default::default();
    let batch_size = 20;
    for n in (1..=ngames).step_by(batch_size) {
        (0..batch_size).into_par_iter().for_each(|_| {
            tdl_learn_iter(&player, &net, alpha, &stats);
        });
        {
            let stats_data = stats.lock().unwrap();
            let max_tile_avg = stats_data.max_tile_total as f32 / batch_size as f32;
            let score_avg = stats_data.score_total as f32 / batch_size as f32;
            let moves_avg = stats_data.moves_total as f32 / batch_size as f32;
            println!("Batch {n}: Score: {score_avg} Max: {max_tile_avg} Moves: {moves_avg}");
            print_tile_freq(&stats_data.tile_counter);
        }
        stats = Default::default();
    }

    // write bytes to filepath
    let mut file = File::create(save_path).unwrap();
    let mut writer = BufWriter::new(&mut file);
    net.save_weights(&mut writer);
}

fn tdl_learn_iter(player: &impl Player, net: &NTuple, alpha: f32, stats: &Mutex<Stats>) {
    let mut path = Vec::with_capacity(20000);
    let mut rng = Rng::new();
    let mut b = Board::new();
    b.add_random_tile(&mut rng);
    b.add_random_tile(&mut rng);
    let mut score = 0;
    let mut moves = 0;

    loop {
        let mv = player.next_move(&b, net);
        if mv.is_none() {
            break;
        }
        let mv = mv.unwrap();
        moves += 1;

        let rec = b.make_move_and_record(mv).unwrap();
        b.add_random_tile(&mut rng);
        score += rec.score;
        path.push(rec);
    }

    net.backward(&mut path, alpha);
    path.clear();

    let mut stats = stats.lock().unwrap();
    stats.score_total += score;
    stats.max_tile_total += b.max_tile();
    stats.moves_total += moves;
    stats.tile_counter[b.log_max_tile() as usize] += 1;
}

pub fn tdl_learn(save_path: &str, alpha: f32, ngames: u32) {
    let mut net = NTuple::default();

    let mut rng = Rng::new();
    let mut path = Vec::with_capacity(20000);

    let print_stats_every = 1000;

    let mut score_total = 0;
    let mut max_tile_total = 0;
    let mut moves_total = 0;
    let mut tile_counter = [0; 16];

    let threads = 10;
    for n in (1..=ngames).step_by(threads) {
        let mut b = Board::new();
        b.add_random_tile(&mut rng);
        b.add_random_tile(&mut rng);
        let mut score = 0;
        let mut moves = 0;

        loop {
            let mv = net.next_move(&b, &NullHeuristic {});
            if mv.is_none() {
                break;
            }
            let mv = mv.unwrap();
            moves += 1;

            let rec = b.make_move_and_record(mv).unwrap();
            b.add_random_tile(&mut rng);
            score += rec.score;
            path.push(rec);
        }

        net.backward(&mut path, alpha);
        path.clear();

        if n % print_stats_every == 0 {
            let max_tile_avg = max_tile_total as f32 / print_stats_every as f32;
            let score_avg = score_total as f32 / print_stats_every as f32;
            let moves_avg = moves_total as f32 / print_stats_every as f32;
            println!("Game {n}: Score: {score_avg} Max: {max_tile_avg} Moves: {moves_avg}");
            print_tile_freq(&tile_counter);
            score_total = 0;
            max_tile_total = 0;
            moves_total = 0;
            tile_counter = [0; 16];
        }
        score_total += score;
        max_tile_total += b.max_tile();
        moves_total += moves;
        tile_counter[b.log_max_tile() as usize] += 1;
    }

    // write bytes to filepath
    let mut file = File::create(save_path).unwrap();
    let mut writer = BufWriter::new(&mut file);
    net.save_weights(&mut writer);
}

fn print_tile_freq(tile_counter: &[u32; 16]) {
    let tile_sum: u32 = tile_counter.iter().sum();
    let mut cumulative = [0; 16];
    let mut sum = 0;
    for i in (0..16).rev() {
        sum += tile_counter[i];
        cumulative[i] = sum;
    }
    let cum_max = cumulative[0];
    println!("==== Tile Frequency ====");
    for i in 0..16 {
        let freq = tile_counter[i] as f32 / tile_sum as f32;
        let cum_freq = cumulative[i] as f32 / cum_max as f32;
        if freq > 0.0 {
            let percent = freq * 100.0;
            let cum_percent = cum_freq * 100.0;
            let tile = 1u32 << i;
            println!("{tile}: {percent:.2}% ({cum_percent:.2}%)");
            // pad the output as if it were a table
        }
    }
    println!("========================\n");
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn monte_carlo(arr: &[i32]) -> i32 {
    let b = Board::from_arr(arr);
    let next_move = MonteCarloPlayer::default().next_move(&b, &NullHeuristic {});
    match next_move {
        Some(m) => m.to_int(),
        None => -1,
    }
}

#[wasm_bindgen]
pub fn expectimax(arr: &[i32]) -> i32 {
    lazy_static! {
        static ref CACHE: HeuristicScoreCache = HeuristicScoreCache::new();
    }
    let b = Board::from_arr(arr);
    let next_move = ExpectimaxPlayer::default().next_move(&b, &*CACHE);
    match next_move {
        Some(m) => m.to_int(),
        None => -1,
    }
}

#[wasm_bindgen]
pub fn build_ntuple(weights: &[u8]) -> NTuple {
    console_error_panic_hook::set_once();
    let mut net = NTuple::default();
    net.load_weights(&mut weights.as_ref());
    net
}

#[wasm_bindgen]
pub fn ntuple(net: &NTuple, arr: &[i32]) -> i32 {
    // statically load the network weights
    let b = Board::from_arr(arr);
    let next_move = net.next_move(&b, &NullHeuristic {});
    match next_move {
        Some(m) => m.to_int(),
        None => -1,
    }
}
