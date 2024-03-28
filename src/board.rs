use fastrand::Rng;
use lazy_static::lazy_static;
use std::fmt;

lazy_static! {
    static ref CACHE: MoveCache = MoveCache::new();
}

#[derive(Clone, Copy)]
pub struct Board {
    pub raw: u64,
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
    pub fn rand(rng: &mut Rng) -> Self {
        Self::from_int(rng.u32(0..4))
    }

    pub fn from_int(i: u32) -> Self {
        match i {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Right,
            _ => panic!(),
        }
    }

    pub fn all() -> [Move; 4] {
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
            let (score_delta, newrow) = CACHE.get_right(oldrow);
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

    pub fn _get_col(&self, i: u8) -> Row {
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

    pub fn _set_col(&mut self, i: u8, val: Row) {
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

    pub fn get(&self, i: u8, j: u8) -> u8 {
        self.at(i * 4 + j)
    }

    pub fn get_row(&self, i: u8) -> Row {
        let raw = ((self.raw >> (i << 4)) & 0xffff) as u16;
        Row::from_raw(raw)
    }

    pub fn at(&self, i: u8) -> u8 {
        ((self.raw >> (i << 2)) & 0x0f) as u8
    }

    pub fn set(&mut self, i: u8, val: u8) {
        self.raw = (self.raw & !(0x0f << (i << 2))) | (((val & 0x0f) as u64) << (i << 2));
    }

    pub fn set_row(&mut self, i: u8, row: Row) {
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

    pub fn clockwise(&mut self) {
        self.transpose();
        self.flip_horizontal();
    }
    pub fn counterclockwise(&mut self) {
        self.transpose();
        self.flip_vertical();
    }

    fn flip_horizontal(&mut self) {
        let raw = self.raw;
        self.raw = ((raw & 0x000f000f000f000f) << 12)
            | ((raw & 0x00f000f000f000f0) << 4)
            | ((raw & 0x0f000f000f000f00) >> 4)
            | ((raw & 0xf000f000f000f000) >> 12);
    }

    fn flip_vertical(&mut self) {
        let raw = self.raw;
        self.raw = ((raw & 0x000000000000ffff) << 48)
            | ((raw & 0x00000000ffff0000) << 16)
            | ((raw & 0x0000ffff00000000) >> 16)
            | ((raw & 0xffff000000000000) >> 48);
    }

    pub fn add_random_tile(&mut self, rng: &mut Rng) {
        // let empty_spaces: [u8; 16] = (0..16 as u8).filter(|&i| self.at(i) == 0).collect();
        let mut len = 0;
        let mut empty_spaces: [u8; 16] = [0; 16];
        for i in 0..16 {
            if self.at(i) == 0 {
                empty_spaces[len] = i;
                len += 1;
            }
        }
        if len > 0 {
            self.set(
                empty_spaces[rng.usize(0..len)],
                if rng.u8(0..10) != 0 { 1 } else { 2 },
            );
        }
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
        }

        Self { left_cache }
    }

    fn get_left(&self, i: Row) -> (u16, Row) {
        let res = self.left_cache[i.raw as usize];
        return (res.score, res.after);
    }

    fn get_right(&self, i: Row) -> (u16, Row) {
        let res = self.left_cache[i.reverse().raw as usize];
        return (res.score, res.after.reverse());
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Row {
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
    pub fn get(&self, i: u8) -> u8 {
        return ((self.raw >> (i * 4)) & 0x0f) as u8;
    }
    pub fn set(&mut self, i: u8, r: u8) {
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
    pub fn reverse(&self) -> Self {
        let raw = self.raw;
        let p1 = (raw & 0x000f) << 3 * 4;
        let p2 = (raw & 0x00f0) << 1 * 4;
        let p3 = (raw & 0x0f00) >> 1 * 4;
        let p4 = (raw & 0xf000) >> 3 * 4;
        return Self::from_raw(p1 | p2 | p3 | p4);
    }
    pub fn shift_right(&self) -> (u32, Self) {
        let rev = self.reverse();
        let (score, shifted) = rev.shift_left();
        return (score, shifted.reverse());
    }
}
