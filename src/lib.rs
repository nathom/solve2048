use lazy_static::lazy_static;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monte_carlo() {
        play_monte_carlo(50, 1, MonteCarloMetric::Sum);
        assert!(false);
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
        b.set_row(0, r);
        for i in 0..4 {
            assert_eq!(b.get(0, i), i);
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

fn randint() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen::<usize>()
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

    pub fn game_ended(&self) -> bool {
        (0..4)
            .map(|i| {
                let mut b: Board = *self;
                b.make_move(Move::from_int(i))
            })
            .all(|x| x.is_none())
    }

    pub fn sum_tile(&self) -> u32 {
        (0..16).map(|i| 1 << self.at(i)).sum()
    }

    pub fn max_tile(&self) -> u32 {
        (0..16).map(|i| 1 << self.at(i)).max().unwrap()
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
            let (newrow, score_delta) = get_cached_move(&oldrow);
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
        self.flip_horizontal();
        let res = self.move_left();
        self.flip_horizontal();
        res
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

    fn reverse(&mut self) {
        self.flip_horizontal();
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

    pub fn add_random_tile(&mut self) {
        let empty_spaces: Vec<u8> = (0..16 as u8).filter(|&i| self.at(i) == 0).collect();
        if empty_spaces.len() > 0 {
            // chose random free square to put tile
            self.set(
                empty_spaces[randint() % empty_spaces.len()],
                if randint() % 10 != 0 { 1 } else { 2 },
            );
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Row {
    pub raw: u16,
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
}

#[derive(Clone, Copy)]
struct MoveCacheElem {
    after: Row,
    score: u16,
}

struct MoveCache {
    cache: [MoveCacheElem; 1 << 16],
}

impl MoveCache {
    fn new() -> Self {
        println!("Init cache");
        let default = MoveCacheElem {
            after: Row::new(),
            score: 0,
        };
        let mut cache = [default; 1 << 16];
        for i in 0x0000..=0xffff as u16 {
            let row = Row::from_raw(i);
            let (score, after) = row.shift_left();
            // Assume that we're not getting > 65k points in one shift
            // Won't happen given that I've rarely gotten a single 32k tile
            let score = score as u16;
            cache[i as usize] = MoveCacheElem { after, score };
        }
        println!("Done Init cache");

        Self { cache }
    }

    fn get(&self, i: u16) -> MoveCacheElem {
        self.cache[i as usize]
    }
}

fn get_cached_move(row: &Row) -> (Row, u16) {
    lazy_static! {
        static ref CACHE: MoveCache = MoveCache::new();
    }
    let res = CACHE.get(row.raw);
    return (res.after, res.score);
}

pub enum MonteCarloMetric {
    Sum,
    MaxTile,
    Score,
    Moves,
}
pub struct MonteCarloPlayer {
    niter: u32,
    metric: MonteCarloMetric,
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

    pub fn next_move(&self, b: &Board) -> Move {
        let (mv, _) = Move::all()
            .iter()
            .map(|&m| (m, self.explore_move(b, m)))
            .max_by_key(|&(_, s)| s)
            .unwrap();
        mv
    }

    pub fn explore_move(&self, b: &Board, m: Move) -> u32 {
        let mut b: Board = *b;
        let mut score = match b.make_move(m) {
            Some(s) => s,
            None => return 0,
        };
        let mut rng = rand::thread_rng();
        for _ in 0..self.niter {
            score += self.random_run(&b, &mut rng);
        }
        score
    }
    pub fn random_run(&self, b: &Board, rng: &mut ThreadRng) -> u32 {
        let mut b: Board = *b;
        let mut nmoves = 0;
        let mut score = 0;
        while !b.game_ended() {
            if let Some(delta) = b.make_move(Move::rand(rng)) {
                score += delta;
                nmoves += 1;
                b.add_random_tile();
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

pub fn monte_carlo_single_game(player: &MonteCarloPlayer) -> (u32, u32) {
    let mut b = Board::new();
    b.add_random_tile();
    b.add_random_tile();

    let mut score = 0;
    while !b.game_ended() {
        let m = player.next_move(&b);
        if let Some(s) = b.make_move(m) {
            score += s;
            b.add_random_tile();
        }
    }
    let max_tile = b.max_tile();
    return (score, max_tile);
}

pub fn play_monte_carlo(niter: u32, ngames: u32, metric: MonteCarloMetric) {
    let player = MonteCarloPlayer::new(niter, metric);
    for _ in 0..ngames {
        let (score, max) = monte_carlo_single_game(&player);
        println!("Score {score} Max: {max}");
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
