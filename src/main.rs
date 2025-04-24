//use std::{process, env};
use macroquad::{prelude::*};
mod array2d;
mod game;

type Vec2 = (f32, f32);


const GAME_SIZE: [usize; 2] = [15, 10];
const NUM_MINES: usize = 10;

const BASE: Color = Color::from_hex(0xc0c0c0);
const HIGHLIGHT: Color = WHITE;
const SHADOW: Color = Color::from_hex(0x808080);
const GRID: Color = Color::from_hex(0x808080);
const FLAG_COLOR: Color = Color::from_hex(0xfe0000);

const MARGIN: f32 = 5.;

const WINSIZE: f32 = 500.;

#[derive(PartialEq, Default)]
enum State { 
    #[default]
    Wait,
    Dead(game::Coord),
    Won,
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

#[derive(Default)]
struct InnerGame {
    screen: Vec2, // size of virtual screen
    offset: Vec2, // offset to draw from origin
    grid: f32,
    state: State,
    mouse: Option<(Vec2, bool)>,
    first: bool,
    game: game::Game,
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("rsweeper"),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    /*let args: Vec<String> = env::args().collect();
    if args.len() <= 2 { println!("Provide args!"); process::exit(1); }
    
    let mut dim: Vec<usize> = Vec::new();
    for i in 1..=2 {
        dim.push(args[i].parse().expect("Arg {i} is not integer"));
    }
    
    println!("{} {}", dim[0], dim[1]);
    */
   
    //initalize
    let mut screen;
    
    let bg = Texture2D::from_file_with_format(include_bytes!("bliss.png"), None);
    
    let mut game = InnerGame::default();
    game.game.resize(GAME_SIZE[0], GAME_SIZE[1]);
    game.first = true;

    loop {
        draw_texture_ex(&bg, 0., 0., WHITE, DrawTextureParams {
            dest_size: Some(vec2(1280., 720.)), ..Default::default()
        }); //bliss bg

        let height = (WINSIZE - MARGIN * 6.) / game.game.get_size().x() as f32 
            * game.game.get_size().y() as f32;

        // calc sweeper window height
        screen = (WINSIZE, height + 105.); 


        // window
        draw_raised_rect((0., 0.), screen, true, MARGIN);
        draw_raised_rect((MARGIN * 2., MARGIN * 2.), 
            (screen.0 - MARGIN * 4., 70.), false, MARGIN);
        draw_raised_rect((MARGIN * 4., MARGIN * 4.), (70., 50.), false, MARGIN);
        draw_raised_rect((WINSIZE - 70. - MARGIN * 4., MARGIN * 4.), 
            (70., 50.), false, MARGIN);
        draw_raised_rect(((WINSIZE - 50.) / 2., MARGIN * 4.), 
            (50., 50.), true, MARGIN);
        
        draw_raised_rect(((MARGIN * 2.), 70. + MARGIN * 3.), 
            (WINSIZE - MARGIN * 4., height + MARGIN * 2.), false, MARGIN);
        game.screen = (WINSIZE - MARGIN * 6., height);
        game.offset = (MARGIN * 3., 90.);

        game.redraw();
        
        if is_mouse_button_released(MouseButton::Left) {
            game.mouse = Some((mouse_position(), false));
        } else if is_mouse_button_released(MouseButton::Right) {
            game.mouse = Some((mouse_position(), true));
        }

        game.update();

        next_frame().await
    }
}

impl InnerGame {
    fn redraw(&mut self) {
        self.grid = self.screen.0 / self.game.get_size().x() as f32;

        draw_rectangle(self.offset.0, self.offset.1, 
            self.screen.0, self.screen.1, BASE);

        if let State::Dead(spot) = self.state {
            let (x, y) = (spot.x() as f32 * self.grid + self.offset.0, 
                spot.y() as f32 * self.grid + self.offset.1);
            draw_rectangle(x, y, self.grid, self.grid, RED);
        }

        // draw horizontal lines
        for x in 1..self.game.get_size().x() {
            let x = x as f32 * self.grid + self.offset.0;
            draw_line(x, self.offset.1, x, self.screen.1 + self.offset.1, 3., GRID);
        }

        // draw vertical lines
        for y in 1..self.game.get_size().y() {
            let y = y as f32 * self.grid + self.offset.1;
            draw_line(self.offset.0, y, self.screen.0 + self.offset.0, y, 3., GRID);
        }

        // draw tiles
        for (co, cell) in self.game.grid.enumerate() {
            let co = (co.x() as f32 * self.grid + self.offset.0, 
                co.y() as f32 * self.grid + self.offset.1);
            draw_tile(co, self.grid, &cell, self.state.gameover());
        }
    }

    fn update(&mut self) {
        if let Some((p, d)) = self.mouse {
            if self.state.gameover() {
                self.game.reset();
                self.first = true;
                self.state = State::Wait;
            } else {
                let spot = game::Coord(
                    ((p.0 - self.offset.0) / self.grid) as isize, 
                    ((p.1 - self.offset.1) / self.grid) as isize);

                println!("{:?} - {:?} / {:?} = {:?}", p, self.offset, self.grid, spot);

                if self.first {
                    // if generate mines fails, returns false, still first, go again
                    self.first = 
                        !self.game.generate_mines(NUM_MINES, game::Coord::from(spot));
                }
                if self.game.action(spot, d) { 
                    self.state = State::Dead(game::Coord::from(spot)); 
                }
                self.game.print_game();
                if self.game.get_score() == 0 {
                    println!("gamewon!");
                    self.state = State::Won;
                }
            }
            self.mouse = None;
        }
    }
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

//                 X    Y            W    H
fn draw_tile(pos: Vec2, size: f32, cell: &game::Cell, over: bool) {
    let cen = (pos.0 + size / 2., pos.1 + size / 2.);
    if cell.dug() {
        let n = cell.get_num();
        draw_ctext(&format!("{n}",), cen, 50, map_num(n));
        return;
    }
    if cell.is_mine() && over && !cell.flagged() {
        draw_ctext("*", cen, 50, BLACK);
        return;
    }
    draw_raised_rect(pos, (size, size), true, 5.);
    if cell.flagged() {
        if over && !cell.is_mine() { // if flag is wrong tint tile red
            draw_rectangle(pos.0, pos.1, size, size, RED.with_alpha(0.5)); 
        } 
        draw_ctext("P", cen, 50, FLAG_COLOR);
    }
}

fn draw_ctext(t: &str, pos: Vec2, s: u16, c: Color) {
    let center = get_text_center(&t, None, s, 1., 0.);
    draw_text(&t, pos.0 - center.x, pos.1 - center.y, s as f32, c); 
}


//    _______         _______ 
//   |      /|       |  ___ /|
//   |    /  |  -->  | |   | |
//   |  /    |       | |___| |
//   |/______|       |/______|
//
//                       x y
fn draw_raised_rect(pos: Vec2, size: Vec2, raised: bool, margin: f32) {
    let mut left = HIGHLIGHT;
    let mut right = SHADOW;
    if !raised {
        left = SHADOW;
        right = HIGHLIGHT;
    }

    // shadow
    draw_rectangle(pos.0, pos.1, size.0, size.1, right);
    // highlight
    draw_triangle(pos.into(), (pos.0 + size.0, pos.1).into(), 
        (pos.0, pos.1 + size.1).into(), left);
    // base
    draw_rectangle(pos.0 + margin, pos.1 + margin, 
        size.0 - margin * 2., size.1 - margin * 2., BASE);
}
