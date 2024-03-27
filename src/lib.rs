use lazy_static::lazy_static;
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::fmt;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref CACHE: MoveCache = MoveCache::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn monte_carlo() {
        play_monte_carlo(1000, 10, MonteCarloMetric::Sum);
    }

    #[test]
    fn board_add_random() {
        let mut b = Board::new();
        let mut rng = rand::thread_rng();
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

#[derive(Clone, Copy)]
pub struct Board {
    raw: u64,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_str = String::new();
        for y in 0..4 {
            for x in 0..4 {
                let value = self.get(x, y);
                board_str.push_str(&format!("{} ", value));
            }
            board_str.pop(); // Remove the last space
            board_str.push('\n'); // Newline after each row
        }
        write!(f, "{}", board_str)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl Move {
    pub fn rand(rng: &mut ThreadRng) -> Self {
        Self::from_int(rng.gen::<u32>() % 4)
    }

    fn from_int(i: u32) -> Self {
        match i {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Right,
            _ => panic!(),
        }
    }

    fn all() -> [Move; 4] {
        return [Move::Up, Move::Down, Move::Left, Move::Right];
    }
}

impl Board {
    pub fn new() -> Self {
        Self { raw: 0 }
    }

    pub fn from_raw(raw: u64) -> Self {
        Self { raw }
    }

    pub fn game_ended(&self) -> bool {
        (0..4)
            .map(|i| {
                let mut b: Board = *self;
                b.make_move(Move::from_int(i))
            })
            .all(|x| x.is_none())
    }

    pub fn sum_tile(&self) -> u32 {
        (0..16).map(|i| 1u32 << self.at(i)).sum()
    }

    pub fn max_tile(&self) -> u32 {
        1u32 << (0..16).map(|i| self.at(i)).max().unwrap()
    }

    pub fn make_move(&mut self, m: Move) -> Option<u32> {
        match m {
            Move::Up => self.move_up(),
            Move::Down => self.move_down(),
            Move::Left => self.move_left(),
            Move::Right => self.move_right(),
        }
    }

    pub fn move_left(&mut self) -> Option<u32> {
        let mut score = 0;
        let mut did_move = false;
        for row in 0..4 {
            let oldrow = self.get_row(row);
            // let (score_delta, newrow) = oldrow.shift_left();
            let (score_delta, newrow) = CACHE.get_left(oldrow);
            if newrow != oldrow {
                did_move = true;
            }
            self.set_row(row, newrow);
            score += score_delta;
        }

        if did_move {
            Some(score as u32)
        } else {
            None
        }
    }

    pub fn move_right(&mut self) -> Option<u32> {
        let mut score = 0;
        let mut did_move = false;
        for row in 0..4 {
            let oldrow = self.get_row(row);
            let (score_delta, newrow) = CACHE.get_left(oldrow.reverse());
            let newrow = newrow.reverse();
            if newrow != oldrow {
                did_move = true;
            }
            self.set_row(row, newrow);
            score += score_delta;
        }

        if did_move {
            Some(score as u32)
        } else {
            None
        }
    }

    fn _get_col(&self, i: u8) -> Row {
        let selector: u64 = 0x000f_000f_000f_000f << (i * 4);
        let raw = self.raw;
        let selected_raw = raw & selector;
        let shifted_raw = selected_raw >> (i * 4);
        let col = (shifted_raw & 0xf)
            | ((shifted_raw & 0x000f_0000) >> (4 * 3))
            | ((shifted_raw & 0x000f_0000_0000) >> (4 * 6))
            | (shifted_raw & 0x000f_0000_0000_0000) >> (4 * 9);
        return Row::from_raw(col as u16);
    }

    fn _set_col(&mut self, i: u8, val: Row) {
        let raw = self.raw;
        let val = val.raw;
        let selector = 0x000f_000f_000f_000f << (i * 4);
        let deletor = !selector;
        // set insert positions to 0
        let masked_raw = raw & deletor;
        // Select 4 bits from each pos
        let v1 = ((val & 0xf000) >> 3 * 4) as u64;
        let v2 = ((val & 0x0f00) >> 2 * 4) as u64;
        let v3 = ((val & 0x00f0) >> 1 * 4) as u64;
        let v4 = ((val & 0x000f) >> 0 * 4) as u64;
        let val_placed = (v1 << (16 * 3)) | (v2 << (16 * 2)) | (v3 << (16 * 1)) | (v4 << (16 * 0));
        // insert into raw
        let final_raw = masked_raw | (val_placed << (i * 4));
        self.raw = final_raw;
    }

    pub fn move_up(&mut self) -> Option<u32> {
        self.counterclockwise();
        let res = self.move_left();
        self.clockwise();
        res
    }

    pub fn move_down(&mut self) -> Option<u32> {
        self.clockwise();
        let res = self.move_left();
        self.counterclockwise();
        res
    }

    fn get(&self, i: u8, j: u8) -> u8 {
        self.at(i * 4 + j)
    }

    fn get_row(&self, i: u8) -> Row {
        let raw = ((self.raw >> (i << 4)) & 0xffff) as u16;
        Row::from_raw(raw)
    }

    fn at(&self, i: u8) -> u8 {
        ((self.raw >> (i << 2)) & 0x0f) as u8
    }

    fn set(&mut self, i: u8, val: u8) {
        self.raw = (self.raw & !(0x0f << (i << 2))) | (((val & 0x0f) as u64) << (i << 2));
    }

    fn set_row(&mut self, i: u8, row: Row) {
        let row = row.raw;
        self.raw = (self.raw & !((0xffff as u64) << (i << 4))) | ((row as u64) << (i << 4));
    }

    fn transpose(&mut self) {
        let step1 = (self.raw & 0xf0f00f0ff0f00f0f)
            | ((self.raw & 0x0000f0f00000f0f0) << 12)
            | ((self.raw & 0x0f0f00000f0f0000) >> 12);
        let step2 = (step1 & 0xff00ff0000ff00ff)
            | ((step1 & 0x00000000ff00ff00) << 24)
            | ((step1 & 0x00ff00ff00000000) >> 24);
        self.raw = step2;
    }

    fn clockwise(&mut self) {
        self.transpose();
        self.flip_horizontal();
    }
    fn counterclockwise(&mut self) {
        self.transpose();
        self.flip_vertical();
    }

    fn flip_horizontal(&mut self) {
        self.raw = ((self.raw & 0x000f000f000f000f) << 12)
            | ((self.raw & 0x00f000f000f000f0) << 4)
            | ((self.raw & 0x0f000f000f000f00) >> 4)
            | ((self.raw & 0xf000f000f000f000) >> 12);
    }

    fn flip_vertical(&mut self) {
        self.raw = ((self.raw & 0x000000000000ffff) << 48)
            | ((self.raw & 0x00000000ffff0000) << 16)
            | ((self.raw & 0x0000ffff00000000) >> 16)
            | ((self.raw & 0xffff000000000000) >> 48);
    }

    pub fn add_random_tile(&mut self, rng: &mut ThreadRng) {
        let empty_spaces: Vec<u8> = (0..16 as u8).filter(|&i| self.at(i) == 0).collect();
        if empty_spaces.len() > 0 {
            self.set(
                empty_spaces[rng.gen_range(0..empty_spaces.len())],
                if rng.gen_range(0..10) != 0 { 1 } else { 2 },
            );
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Row {
    pub raw: u16,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_str = String::new();
        for x in 0..4 {
            let value = self.get(x);
            board_str.push_str(&format!("{} ", value));
        }
        board_str.pop(); // Remove the last space
        board_str.push('\n'); // Remove the last space
        write!(f, "{}", board_str)
    }
}

impl Row {
    pub fn new() -> Self {
        Self { raw: 0 }
    }
    pub fn from_raw(r: u16) -> Self {
        Self { raw: r }
    }
    fn get(&self, i: u8) -> u8 {
        return ((self.raw >> (i * 4)) & 0x0f) as u8;
    }
    fn set(&mut self, i: u8, r: u8) {
        let pos = i * 4;
        let masked_raw = self.raw & !(0xf << pos);
        let mod_raw = masked_raw | ((r as u16) << pos);
        self.raw = mod_raw;
    }
    pub fn shift_left(&self) -> (u32, Self) {
        let mut score: u32 = 0;
        let mut top = 0;
        let mut tmp = 0;
        let mut row: Self = *self;

        for i in 0..4 {
            let mut tile = self.get(i);
            if tile == 0 {
                continue;
            }
            row.set(i, 0);
            if tmp != 0 {
                if tile == tmp {
                    tile = tile + 1;
                    row.set(top, tile);
                    top += 1;
                    score += 1 << tile;
                    tmp = 0;
                } else {
                    row.set(top, tmp);
                    top += 1;
                    tmp = tile;
                }
            } else {
                tmp = tile;
            }
        }
        if tmp != 0 {
            row.set(top, tmp);
        }
        return (score, row);
    }
    fn reverse(&self) -> Self {
        let raw = self.raw;
        let p1 = (raw & 0x000f) << 3 * 4;
        let p2 = (raw & 0x00f0) << 1 * 4;
        let p3 = (raw & 0x0f00) >> 1 * 4;
        let p4 = (raw & 0xf000) >> 3 * 4;
        return Self::from_raw(p1 | p2 | p3 | p4);
    }
}

#[derive(Clone, Copy)]
struct MoveCacheElem {
    after: Row,
    score: u16,
}

struct MoveCache {
    left_cache: [MoveCacheElem; 1 << 16],
}

impl MoveCache {
    fn new() -> Self {
        let default = MoveCacheElem {
            after: Row::new(),
            score: 0,
        };
        let mut left_cache = [default; 1 << 16];
        for i in 0x0000..=0xffff as u16 {
            let row = Row::from_raw(i);
            // Assume that we're not getting > 65k points in one shift
            // Won't happen given that I've rarely gotten a single 32k tile
            let (score, after) = row.shift_left();
            let score = score as u16;
            left_cache[i as usize] = MoveCacheElem { after, score };

            // let (score, after) = row.shift_right();
            // // Assume that we're not getting > 65k points in one shift
            // // Won't happen given that I've rarely gotten a single 32k tile
            // let score = score as u16;
            // right_cache[i as usize] = MoveCacheElem { after, score };
        }

        Self {
            left_cache,
            // right_cache,
        }
    }

    fn get_left(&self, i: Row) -> (u16, Row) {
        let res = self.left_cache[i.raw as usize];
        return (res.score, res.after);
    }
}

#[derive(Clone)]
pub enum MonteCarloMetric {
    Sum,
    MaxTile,
    Score,
    Moves,
}

pub trait Player {
    fn next_move(&self, b: &Board) -> Option<Move>;
}
#[derive(Clone)]
pub struct MonteCarloPlayer {
    niter: u32,
    metric: MonteCarloMetric,
}

impl Player for MonteCarloPlayer {
    fn next_move(&self, b: &Board) -> Option<Move> {
        let res = Move::all()
            .iter()
            .map(|&m| (m, self.explore_move(b, m)))
            .filter(|&(_, s)| s > 0)
            .max_by_key(|&(_, s)| s);

        if let Some((mv, _)) = res {
            return Some(mv);
        } else {
            // no moves possible
            return None;
        }
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
        let mut rng = rand::thread_rng();
        b.add_random_tile(&mut rng);

        score += (0..self.niter)
            .into_par_iter()
            .map(|_| self.random_run(&b))
            .sum::<u32>();

        score
    }
    pub fn random_run(&self, b: &Board) -> u32 {
        let mut b: Board = *b;
        let mut nmoves = 0;
        let mut score = 0;
        let mut fails = 0;
        let mut rng = rand::thread_rng();
        // approximation of !game_ended
        while fails < 5 {
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

pub fn play_game<P: Player>(player: &P) -> (u32, u32) {
    let mut b = Board::new();
    let mut rng = rand::thread_rng();
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
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
