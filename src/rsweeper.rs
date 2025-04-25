use macroquad::{
    color::{
        colors::*, 
        Color},
    text::{
        get_text_center,
        draw_text},
};
use crate::game::{Game, Coord, Cell};
use crate::widget::{Vec2, Widget, WidgetImpl};

const WINSIZE: f32 = 500.;
const MARGIN: f32 = 5.;

const BASE: Color = Color::from_hex(0xc0c0c0);
const GRID: Color = Color::from_hex(0x808080);
const FLAG_COLOR: Color = Color::from_hex(0xfe0000);

const RAISED: [Color; 3] = [WHITE, BASE, GRID];
const RECESSED: [Color; 3] = [GRID, BASE, WHITE];

const GAME_SIZE: [usize; 2] = [15, 10];
const NUM_MINES: usize = 10;

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
    pub grid: f32,
    pub state: State,
    pub first: bool,
    pub game: Game,
}

impl Default for Rsweeper {
    fn default() -> Self {
        let mut ret = Rsweeper{
            widget: Widget::default(),
            game: InnerGame::default(),
        };
        ret.game.game.resize(GAME_SIZE[0], GAME_SIZE[1]);
        ret.game.first = true;
        let height = (WINSIZE - MARGIN * 6.) / ret.game.game.get_size().x() as f32 
            * ret.game.game.get_size().y() as f32;
        ret.widget.screen = (WINSIZE, height + 105.); 
        ret.game.widget.screen = (WINSIZE - MARGIN * 6., height);
        ret.game.widget.offset = (MARGIN * 3., 90.);
        ret
    }
}

impl WidgetImpl for Rsweeper {
    fn draw(&mut self) {
        // draw window
        let screen = self.screen();
        self.draw_raised_rect((0., 0.), screen, MARGIN, RAISED);
        self.draw_raised_rect((MARGIN * 2., MARGIN * 2.), 
            (screen.0 - MARGIN * 4., 70.), MARGIN, RECESSED);
        self.draw_raised_rect((MARGIN * 4., MARGIN * 4.), (70., 50.), MARGIN, RECESSED);
        self.draw_raised_rect((screen.0 - 70. - MARGIN * 4., MARGIN * 4.), 
            (70., 50.), MARGIN, RECESSED);
        self.draw_raised_rect(((WINSIZE - 50.) / 2., MARGIN * 4.), 
            (50., 50.), MARGIN, RAISED);
        self.draw_raised_rect(((MARGIN * 2.), 70. + MARGIN * 3.), 
            (WINSIZE - MARGIN * 4., self.game.screen().1 + MARGIN * 2.), MARGIN, RECESSED);

        self.game.draw();
    }

    fn update(&mut self, m: Option<(Vec2, bool)>) {
        self.game.update(m);
    }
    fn screen(&self) -> Vec2 { self.widget.screen }
    fn offset(&self) -> Vec2 { self.widget.offset }
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
    fn to_pix(&self, n: usize) -> f32 { n as f32 * self.grid }
    fn from_pix(&self, p: f32) -> isize { (p / self.grid) as isize }
    fn co_to_pix(&self, c: Coord) -> Vec2 { (self.to_pix(c.0), self.to_pix(c.1)) }
    fn pix_to_co(&self, p: Vec2) -> Coord<isize> { 
        Coord(self.from_pix(p.0), self.from_pix(p.1)) 
    }
    
    fn draw_tile(&self, pos: Vec2, cell: &Cell) {
        let cen = (pos.0 + self.grid / 2., pos.1 + self.grid / 2.);
        if cell.dug() {
            let n = cell.get_num();
            self.draw_ctext(&format!("{n}",), cen, 50, map_num(n));
            return;
        }
        if cell.is_mine() && self.state.gameover() && !cell.flagged() {
            self.draw_ctext("*", cen, 50, BLACK);
            return;
        }
        self.draw_raised_rect(pos, (self.grid, self.grid), 5., RAISED);
        if cell.flagged() {
            if self.state.gameover() && !cell.is_mine() { // if flag is wrong tint tile red
                self.draw_rect(pos, (self.grid, self.grid), RED.with_alpha(0.5)); 
            }
            self.draw_ctext("P", cen, 50, FLAG_COLOR);
        }
    }
}

impl WidgetImpl for InnerGame {
    fn draw(&mut self) {
        let screen = self.widget.screen;
        self.grid = screen.0 / self.game.get_size().w() as f32;

        self.draw_rect((0., 0.), screen, BASE);

        if let State::Dead(spot) = self.state {
            let pos = self.co_to_pix(spot);
            self.draw_rect(pos, (self.grid, self.grid), RED);
        }

        // draw horizontal lines
        for x in 1..self.game.get_size().x() {
            let x = self.to_pix(x);
            self.draw_line((x, 0.), (x, screen.1), 3., GRID);
        }

        // draw vertical lines
        for y in 1..self.game.get_size().y() {
            let y = self.to_pix(y);
            self.draw_line((0., y), (screen.0, y), 3., GRID);
        }
        
        // draw tiles
        for (co, cell) in self.game.grid.enumerate() {
            self.draw_tile(self.co_to_pix(co), &cell);
        }
    }

    fn update(&mut self, m: Option<(Vec2, bool)>) {
        if let Some((p, d)) = m {
            if self.state.gameover() {
                self.game.reset();
                self.first = true;
                self.state = State::Wait;
            } else {
                let spot = self.pix_to_co(self.local_mouse(p));

                //println!("{:?} - {:?} / {:?} = {:?}", p, self.offset, self.grid, spot);

                if self.first {
                    // if generate mines fails, returns false, still first, go again
                    self.first = !self.game.generate_mines(NUM_MINES, spot.into());
                }
                if self.game.action(spot, d) { 
                    self.state = State::Dead(spot.into()); 
                }
                self.game.print_game();
                if self.game.get_score() == 0 {
                    println!("gamewon!");
                    self.state = State::Won;
                }
            }
            self.widget.mouse = None;
        }
    }

    fn screen(&self) -> Vec2 { self.widget.screen }
    fn offset(&self) -> Vec2 { self.widget.offset }
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
