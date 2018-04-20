//extern crate rand;
#![allow(dead_code)]

extern "C" {
    fn rand() -> f64;
}

fn shuffle<T>(a : &mut [T]){
    for i in (1..a.len()).rev() {
        let r = unsafe { rand() };
        let j = (r*(i+1) as f64).floor() as usize;
        a.swap(i, j);
    }
}

pub mod reversi {
#[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Color {O, X}
    impl Color {
        pub fn opposite_color(&self) -> Color {
            match self {
                &Color::O => Color::X,
                &Color::X => Color::O,
            }
        }
    }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Move {
        Pass,
        Mv(u32, u32),
    }
    impl Move {
        fn valid(&self) -> bool {
            match self {
                &Move::Pass => true,
                &Move::Mv(i, j) => i < 8 && j < 8,
            }
        }
    }
    pub trait ReversiBoard {
        fn set(&mut self, i : u32, j : u32, c : Color);
        fn get(&self, i : u32, j : u32) -> Option<Color>;
        fn flippables(&self, i : u32, j : u32, c : Color) -> Vec<(u32, u32)> {
            let d = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
            let opc = c.opposite_color();
            let mut flip : Vec<(u32, u32)> = Vec::new();
            for &(di, dj) in d.into_iter() {
                let mut v = Vec::new();
                let mut i = (i as i32) + di;
                let mut j = (j as i32) + dj;
                while 0 <= i && i < 8 && 0 <= j && j < 8 {
                    if let Some(x) = self.get(i as u32, j as u32) {
                        if x == opc {
                            v.push((i as u32, j as u32));
                        }
                        else {
                            for x in v.iter() {
                                flip.push(*x);
                            }
                            break;
                        }
                    }
                    else {
                        break;
                    }
                    i += di;
                    j += dj;
                }
            }
            return flip;
        }
        fn do_move(&mut self, m : Move, c : Color) -> bool {
            if !m.valid() {
                return false;
            }
            match m {
                Move::Mv(i, j) => {
                    if self.get(i, j) != None {
                        return false;
                    }
                    let ms = self.flippables(i, j, c);
                    if ms.is_empty() {
                        return false;
                    }
                    self.set(i, j, c);
                    for &(i, j) in ms.iter() {
                        self.set(i, j, c);
                    }
                    return true;
                },
                Move::Pass => return true,
            }
        }
        fn valid_move(&self, i: u32, j: u32, c : Color) -> bool {
            return self.get(i, j) == None && !self.flippables(i, j, c).is_empty();
        }
        fn valid_moves(&self, c : Color) -> Vec<Move> {
            let mut v = Vec::new();
            for i in 0..8 {
                for j in 0..8 {
                    if self.valid_move(i, j, c) {
                        v.push(Move::Mv(i, j));
                    }
                }
            }
            return v;
        }
        fn result(&self) -> (u32, u32) {
            let mut o = 0;
            let mut x = 0;
            for i in 0..8 {
                for j in 0..8 {
                    match self.get(i, j) {
                        Some(Color::O) => o += 1,
                        Some(Color::X) => x += 1,
                        None => (),
                    }
                }
            }
            return (o, x);
        }
    }
#[derive(Clone)]
    pub struct U64Board {
        valid : u64,
        board : u64,
    }
    pub const U64BOARD0 : U64Board =
    U64Board {
        valid : 0x0000001818000000,
        board : 0x0000001008000000,
    };
    impl ReversiBoard for U64Board {
#[inline]
        fn set(&mut self, i : u32, j : u32, c : Color){
            self.valid |= 1 << (8*i+j);
            match c {
                Color::O => self.board |= 1 << (8*i+j),
                Color::X => self.board &= !(1u64 << (8*i+j)),
            }
        }
#[inline]
        fn get(&self, i : u32, j : u32) -> Option<Color> {
            if (self.valid & (1 << (8*i+j))) == 0 {
                return None;
            }
            else {
                if (self.board & (1 << (8*i+j))) != 0 {
                    return Some(Color::O);
                }
                else {
                    return Some(Color::X);
                }
            }
        }
    }
    impl U64Board {
        pub fn new() -> U64Board {
            U64BOARD0
        }
        pub fn print(&self) {
            println!(" 01234567");
            for i in 0..8 {
                let b = (self.board >> (8*i)) & 0xFF;
                let v = (self.valid >> (8*i)) & 0xFF;
                print!("{}", i);
                for j in 0..8 {
                    if (v >> j) & 1 == 0 {
                        print!(" ")
                    }
                    else {
                        if (b >> j) & 1 == 0 {
                            print!("x");
                        }
                        else {
                            print!("o");
                        }
                    }
                }
                println!("");
            }
        }
    }
    pub mod reversi_ai {
        use std::cmp::Ordering;
        use reversi_ai::shuffle;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct MvVal(super::Move, i32);
        const INF : i32 = 10000;
        impl PartialOrd for MvVal {
            fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
                return Some(self.1.cmp(&other.1));
            }
        }
        impl Ord for MvVal {
            fn cmp(&self, other : &Self) -> Ordering {
                return self.1.cmp(&other.1);
            }
        }
        fn eval_base0<T : super::ReversiBoard>(b : &T, c : super::Color) -> i32 {
            let mut v : i32 = 0;
            for i in 0..8 {
                for j in 0..8 {
                    if let Some(x) = b.get(i, j) {
                        if x == c {
                            v += 1;
                        }
                        else {
                            v -= 1;
                        }
                    }
                }
            }
            return v;
        }
        fn eval_base1<T : super::ReversiBoard>(b : &T, c : super::Color) -> i32 {
            let mut vp : i32 = 0;
            let mut vn : i32 = 0;
            for i in 0..8 {
                for j in 0..8 {
                    let dv = if ((i == 0) || (i == 7)) && ((j == 0) || (j == 7)) {
                        10
                    }
                    else if (i <= 1) && (j <= 1) && b.get(0, 0) == None {
                        -2
                    }
                    else if (i <= 1) && (j >= 6) && b.get(0, 7) == None {
                        -2
                    }
                    else if (i >= 6) && (j <= 1) && b.get(7, 0) == None {
                        -2
                    }
                    else if (i >= 6) && (j >= 6) && b.get(7, 7) == None {
                        -2
                    }
                    else {
                        1
                    };
                    if let Some(x) = b.get(i, j) {
                        if x == c {
                            vp += dv;
                        }
                        else {
                            vn += dv;
                        }
                    }
                }
            }
            if vn == 0 {
                return INF;
            }
            else if vp == 0 {
                return -INF;
            }
            else {
                return vp - vn;
            }
        }
        fn eval<T : super::ReversiBoard + Clone, F>(b : &T, c : super::Color, depth : u32, resource : u32, eval_base : &F) -> MvVal
         where F : Fn(&T, super::Color) -> i32 {
            let mut ms = b.valid_moves(c);
            shuffle(&mut ms);
            let ms = if ms.is_empty() {vec![super::Move::Pass]} else {ms};
            let opc = c.opposite_color();
            let v : Vec<_> = if depth == 0 || resource == 0 {
                ms.iter().map(|&m| {let mut b = (*b).clone();b.do_move(m, c);MvVal(m, -eval_base(&b, opc))}).collect()
            }
            else{
                ms.iter().map(|&m| {let mut b = (*b).clone();b.do_move(m, c);MvVal(m, -eval(&b, opc, depth - 1, resource / (ms.len() as u32), eval_base).1)}).collect()
            };
            return *v.iter().max().unwrap();
        }
        fn eval_alpha_beta<T : super::ReversiBoard + Clone, F>(b : &T, c : super::Color, upperbound : i32, depth : u32, resource : u32, eval_base : &F) -> MvVal
         where F : Fn(&T, super::Color) -> i32 {
//            let ms = b.valid_moves(c);
            let mut ms = b.valid_moves(c);
//            thread_rng().shuffle(&mut ms);
            shuffle(&mut ms);
            let ms = if ms.is_empty() {vec![super::Move::Pass]} else {ms};
            let opc = c.opposite_color();
            let ms_len = ms.len() as u32;
            if depth == 0 || resource == 0 {
                return ms.iter().map(|&m| {let mut b = (*b).clone();b.do_move(m, c);MvVal(m, -eval_base(&b, opc))}).max().unwrap();
            }
            else{
                let mut max_val = -INF;
                let mut max_mv = super::Move::Pass;
                for m in ms {
                    let mut b = (*b).clone();
                    b.do_move(m, c);
                    let val = eval_alpha_beta(&b, opc, -max_val, depth - 1, resource / ms_len, eval_base).1;
                    if -val > max_val {
                        max_val = -val;
                        max_mv = m;
                        if max_val >= upperbound {
                            break;
                        }
                    }
                }
                return MvVal(max_mv, max_val);
            }
        }
        pub fn best_move<T : super::ReversiBoard + Clone>(b : &T, c : super::Color) -> super::Move {
            return eval(b, c, 10, 50000, &eval_base1).0;
        }
        pub fn best_move_alpha_beta<T : super::ReversiBoard + Clone>(b : &T, c : super::Color) -> super::Move {
            return eval_alpha_beta(b, c, INF, 20, 10000000, &eval_base1).0;
        }
        pub fn best_move_alpha_beta2<T : super::ReversiBoard + Clone>(b : &T, c : super::Color) -> super::Move {
            let mut v : i32 = 0;
            for i in 0..8 {
                for j in 0..8 {
                    if b.get(i, j) == None {
                        v += 1;
                    }
                }
            }
            if v < 15 {
                return eval_alpha_beta(b, c, INF, 20, 1000000, &eval_base0).0;
            }
            else {
                return eval_alpha_beta(b, c, INF, 20, 1000000, &eval_base1).0;
            }
        }
        pub fn has_valid_moves<T : super::ReversiBoard>(b : &T, c : super::Color) -> bool {
            return !b.valid_moves(c).is_empty();
        }
    }
}
