use macroquad::{
    color::{colors::*, Color},
    math::Vec2,
};
use crate::game::{Game, Coord, Cell};
use crate::widget::*;

const SCALE: f32 = 2.;

const MARGIN: f32 = SCALE * 1.;
const MARGIN2: f32 = MARGIN * 2.;
const MARGIN3: f32 = MARGIN * 3.;
const MARGIN6: f32 = MARGIN * 6.;
const MARGIN9: f32 = MARGIN * 9.;
const MARGIN15: f32 = MARGIN * 15.;
const MARGIN18: f32 = MARGIN * 18.;

const fn pix(n: f32) -> f32 { SCALE * n }

const HUDHEIGHT: f32 = SCALE * 37.;
const CELLSIZE: f32 = SCALE * 16.;

const NUM_SIZE: Vec2 = Vec2{x: 39. * SCALE, y: 23. * SCALE};
const GAMEOFFSET: f32 = 43. * SCALE;
const NUM_MINES: usize = 10;

#[derive(Default)]
pub struct Rsweeper {
    pub widget: Widget,
    pub game: InnerGame,
}


#[derive(PartialEq, Default)]
pub enum State { 
    #[default]
    Wait,
    Dead(Coord),
    Won,
}

#[derive(Default)]
pub struct InnerGame {
    pub widget: Widget, 
    pub state: State,
    pub notfirst: bool,
    pub game: Game,
}

impl WidgetImpl for Rsweeper {
    fn draw(&mut self) {
        // draw window
        let screen = self.screen();
        //outer
        self.draw_raised_rect(Vec2::ZERO, screen, MARGIN3, RAISED);
       
        // HUD
        self.draw_raised_rect(Vec2::splat(MARGIN9), 
            screen.with_y(HUDHEIGHT).add_x(-MARGIN18), MARGIN2, RECESSED);
        self.draw_raised_rect(Vec2::splat(MARGIN15), NUM_SIZE + Vec2::ONE, MARGIN, RECESSED);
        self.draw_raised_rect(screen.with_y(MARGIN15).add_x(-NUM_SIZE.x-MARGIN15),
            NUM_SIZE + Vec2::ONE, MARGIN, RECESSED);
        self.draw_raised_rect((screen / 2. - pix(13.)).with_y(pix(14.)), Vec2::splat(pix(26.)), MARGIN, RAISED);
        
        // game frame
        self.draw_raised_rect(Vec2::splat(MARGIN9).add_y(GAMEOFFSET), 
            screen.add_y(-GAMEOFFSET) - MARGIN18, MARGIN3, RECESSED);

        self.game.draw();
    }

    fn update(&mut self) {
        *self.game.mouse_mut() = self.widget.mouse;
        *self.game.screen_mut() = self.screen().add_y(-GAMEOFFSET) - MARGIN18 - MARGIN6;
        *self.game.offset_mut() = self.offset().add_y(GAMEOFFSET) + MARGIN9 + MARGIN3;
        self.game.update();
    }
    fn widget(&self) -> &Widget { &self.widget }
    fn widget_mut(&mut self) -> &mut Widget { &mut self.widget }
}

impl State {
    fn gameover(&self) -> bool {
        match *self {
            State::Dead(_) => true,
            State::Won => true,
            _ => false,
        }
    }
}

impl InnerGame {
    fn to_pix(&self, n: usize) -> f32 { n as f32 * CELLSIZE }
    fn from_pix(&self, p: f32) -> isize { (p / CELLSIZE) as isize }
    fn co_to_pix(&self, c: Coord) -> Vec2 { Vec2{ x: self.to_pix(c.0), y: self.to_pix(c.1)} }
    fn pix_to_co(&self, p: Vec2) -> Coord<isize> { 
        Coord(self.from_pix(p.x), self.from_pix(p.y)) 
    }
    
    fn draw_tile(&self, pos: Vec2, cell: &Cell) {
        let cen = pos + Vec2::splat(CELLSIZE / 2.);
        if cell.dug() {
            let n = cell.get_num();
            self.draw_ctext(&format!("{n}",), cen, 50, map_num(n));
            return;
        }
        if cell.is_mine() && self.state.gameover() && !cell.flagged() {
            self.draw_ctext("*", cen, 50, BLACK);
            return;
        }
        self.draw_raised_rect(pos, Vec2::splat(CELLSIZE), MARGIN, RAISED);
        if cell.flagged() {
            if self.state.gameover() && !cell.is_mine() { // if flag is wrong tint tile red
                self.draw_rect(pos, Vec2::splat(CELLSIZE), RED.with_alpha(0.5)); 
            }
            self.draw_ctext("P", cen, 50, FLAG_COLOR);
        }
    }
}

impl WidgetImpl for InnerGame {
    fn draw(&mut self) {
        let screen = self.widget.screen;

        self.draw_rect(Vec2::ZERO, screen, BASE);

        if let State::Dead(spot) = self.state {
            let pos = self.co_to_pix(spot);
            self.draw_rect(pos, Vec2::splat(CELLSIZE), RED);
        }

        // draw horizontal lines
        for x in 0..=self.game.get_size().x() {
            let x = self.to_pix(x);
            self.draw_line(Vec2::ZERO.with_x(x), screen.with_x(x), pix(1.), GRID);
        }


        // draw vertical lines
        for y in 0..=self.game.get_size().y() {
            let y = self.to_pix(y);
            self.draw_line(Vec2::ZERO.with_y(y), screen.with_y(y), pix(1.), GRID);
        }
        
        // draw tiles
        for (co, cell) in self.game.grid.enumerate() {
            self.draw_tile(self.co_to_pix(co), &cell);
        }
    }

    fn update(&mut self) {
        if let Mouse::Released(p, b) = self.mouse() {
            //println!("aoeu");
            if !self.in_rect(p, Vec2::ZERO, self.screen()) { return; }
            if self.state.gameover() {
                self.game.reset();
                self.notfirst = false;
                self.state = State::Wait;
            } else {
                let spot = self.pix_to_co(self.local_mouse(p));

                //println!("{:?} - {:?} / {:?} = {:?}", p, self.offset, self.grid, spot);

                if !self.notfirst {
                    // if generate mines fails, returns false, still first, go again
                    self.notfirst = self.game.generate_mines(NUM_MINES, spot.into());
                }
                if self.game.action(spot, b) { 
                    self.state = State::Dead(spot.into()); 
                }
                self.game.print_game();
                if self.game.get_score() == 0 {
                    println!("gamewon!");
                    self.state = State::Won;
                }
            }
            self.widget.mouse = Mouse::None;
        }
    }
    fn widget(&self) -> &Widget { &self.widget }
    fn widget_mut(&mut self) -> &mut Widget { &mut self.widget }
}

fn map_num(n: u8) -> Color {
    match n {
        0 => BLANK,
        1 => Color::from_hex(0x0000fe),
        2 => Color::from_hex(0x008001),
        3 => Color::from_hex(0xfe0000),
        4 => Color::from_hex(0x00007e),
        5 => Color::from_hex(0x800000),
        6 => Color::from_hex(0x017f7e),
        7 => Color::from_hex(0x000000),
        8 => Color::from_hex(0x808080),
        _ => RED,
    }
}
