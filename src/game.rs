pub use crate::array2d::Coord;
use crate::array2d::Array2d;
use rand::prelude::*;

//base type, 1-9 are precomped numbers, 10 is mine, 11 is flagged
// <0 means undug, >0 means dug
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Cell(pub u8, bool, bool);

impl Cell {
    pub fn dug(&self) -> bool { self.1 }
    pub fn flagged(&self) -> bool { self.2 }
    pub fn is_mine(&self) -> bool { self.0 == 10 }
    pub fn get_num(&self) -> u8 { self.0 }
    pub fn is_zero(&self) -> bool { self.0 == 0 }


    pub fn force_mine(&mut self) { self.0 = 10 }
    pub fn toggle_flag(&mut self) { self.2 = !self.2 }
    pub fn dig(&mut self) { if self.dug() { panic!(); } self.1 = true; }
    pub fn assign(&mut self, i: u8) { self.0 = i; }
}

#[derive(Default)]
pub struct Game {
    pub grid: Array2d<Cell>,
    score: usize,
}

impl Game {
    pub fn print_game(&self) {
        for i in 0..self.grid.width() {
            print!("{} ", to_ascii(i));
        }
        println!("");  
        println!("");  

        for (y, row) in self.grid.get_rows() {
            for cell in row {
                //print!("{} ", cell.print());
                if !cell.dug() {
                    if cell.flagged() { print!("p "); 
                    } else { print!("o "); }
                } else if cell.is_zero() {
                    print!("  ");
                } else {
                    print!("{} ", cell.get_num());
                }
            }
            println!(" {}", to_ascii(y));  
        }
        println!("{}", self.score);  
    }

    pub fn get_score(&self) -> usize { self.score }

    pub fn resize(&mut self, w: usize, h: usize) {
        self.grid.resize(Coord(w, h));
        self.score = w * h;
        println!("{}", self.score);  
    }

    pub fn generate_mines(&mut self, nums: usize, safe: Coord) {
        if nums > self.grid.len() { return; }
        self.score -= nums;
            let mut num = nums;
            while num > 0 {
                let coord = self.get_random_spot();
                if safe == coord || self.grid.access(coord).dug() { continue; } 
                self.grid.access_mut(coord).force_mine();
                num -= 1;
            }

            let data = self.grid.clone();
            //precompute numbers
            for (coord, cell) in self.grid.enumerate_mut() {
                if cell.is_mine() { continue; }
                let result = data.get_surround_match(Coord::from(coord), Cell::is_mine);
                cell.assign(result);
            }
    }

    fn get_random_spot(&self) -> Coord {
        let mut rng = rand::rng();
        Coord(rng.random_range(0..self.grid.width()), rng.random_range(0..self.grid.height()))
    }

    pub fn action(&mut self, co: Coord<isize>, flag: bool) -> bool {
        let Some(spot) = self.grid.safe_access(co) else { return false; };
        if spot.dug() { return false; }

        if flag { 
            self.grid.access_mut(Coord::from(co)).toggle_flag();
            return false;
        }
        if spot.flagged() { return false; }
        if spot.is_mine() { return true; }
        self.flood_dig(co);
        false
    }

    fn flood_dig(&mut self, co: Coord<isize>) {
        let Some(spot) = self.grid.safe_access(co) else { return; };
        if spot.dug() || spot.is_mine() { return; }
        self.grid.access_mut(Coord::from(co)).dig();
        self.score -= 1;
        if self.grid.access(Coord::from(co)).get_num() != 0 { return; }

        for xoff in -1..=1 {
            for yoff in -1..=1 {
                self.flood_dig(Coord(co.x() + xoff, co.y() + yoff));
            }
        }
    }

    pub fn get_size(&self) -> Coord { self.grid.get_size() }

    pub fn reset(&mut self) {
        let size = self.grid.get_size();
        self.grid.clear();
        self.grid.resize(size);
        self.score = size.mul();
    }
}

fn to_ascii(_i: usize) -> char { 
    '-'
}

pub fn from_ascii(c: char) -> usize {
    match c {
        '0'..='9' => c as usize - 48,
        _ => 0,
    }
}
