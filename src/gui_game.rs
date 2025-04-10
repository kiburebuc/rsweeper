use piston_window::*;
use piston_window::types::Color;

use crate::game;

const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
const FONT: &str = "/home/kibur/Downloads/com.ttf";

#[derive(Default)]
pub struct GuiGame {
    last_pos: [f64; 2],
    mouse_input: Option<MouseButton>,
    bg_color: Color,
    grid_size: [f64; 2],
    draw: bool,
    first: bool,
    game: game::Game,
}

impl GuiGame {
    pub fn execute(&mut self) {
        let mut window = WindowSettings::new("Rsweeper", [320, 320])
            .exit_on_esc(true);
        window.set_vsync(true);
        let mut window: PistonWindow = window.build().unwrap();

        let mut font = window.load_font(FONT).unwrap();

        self.game = game::Game::default();
        self.game.resize(16, 16);
        self.bg_color = WHITE;
        self.draw = true;
        self.first = true;
        
        while let Some(event) = window.next() {
            window.draw_2d(&event, |c, g, d| {
                self.draw(&c, g, &mut font);
                font.factory.encoder.flush(d);
            });
            
            self.handle_input(&event);

            event.update(|arg| {
                self.update(arg);
            });
        }
    }


    fn handle_input(&mut self, e: &Event) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.last_pos = pos;
        }

        if let Some(siz) = e.resize_args() {
            let s = self.game.get_size();
            self.grid_size[0] = siz.window_size[0] / s.x() as f64;
            self.grid_size[1] = siz.window_size[1] / s.y() as f64;
            self.draw = true;
        }

        if let Some(Button::Mouse(key)) = e.release_args() {
            
            self.mouse_input = Some(key);
        }
    }

    fn draw(&mut self, c: &Context, g: &mut G2d, f: &mut Glyphs) {
        // skip draw call if no update
        if  !self.draw { return; }

        let game_size = self.game.get_size();
        let win_size = c.get_view_size();

        //draw horizantal lines
        clear(self.bg_color, g);
        for i in 1..game_size.x() {
            let xcoord = i as f64 * self.grid_size[0];
            graphics::line(RED, 0.5,
                [xcoord, 0.0, xcoord, win_size[1]], c.transform, g);
        }

        //draw vertical lines
        for i in 1..game_size.y() {
            let ycoord = i as f64 * self.grid_size[1];
            graphics::line(RED, 0.5,
                [0.0, ycoord, win_size[0], ycoord], c.transform, g);
        }

        for (co, cell) in self.game.grid.enumerate() {
            if !cell.dug() {
                graphics::rectangle(GREEN, 
                    [co.y() as f64 * self.grid_size[0],
                    co.x() as f64 * self.grid_size[1], self.grid_size[0], self.grid_size[1]]
                    , c.transform, g);
                if cell.flagged() {
                    let text_trans = c.transform.trans(
                        co.y() as f64 * self.grid_size[0]+ 5.0,
                        co.x() as f64 * self.grid_size[1]+ 16.0);
                    let _ = graphics::text(RED, 15, "P", 
                        f, text_trans, g);
                }
            } else {
                if !cell.is_zero() {
                    let num = cell.get_num();
                    let text_trans = c.transform.trans(
                        co.y() as f64 * self.grid_size[0]+ 5.0,
                        co.x() as f64 * self.grid_size[1]+ 16.0);
                    let _ = graphics::text(BLUE, 15, &num.to_string(), 
                        f, text_trans, g);
                }
            }
        } 
        
        self.draw = false;
    }

    fn update(&mut self, _arg: &UpdateArgs) {
        if let Some(key) = self.mouse_input {
            let ygrid = (self.last_pos[0] / self.grid_size[0]) as isize;
            let xgrid = (self.last_pos[1] / self.grid_size[1]) as isize;

            let spot = game::Coord(xgrid, ygrid);

            if self.first {
                self.game.generate_mines(40, game::Coord::from(spot));
                self.first = false;
            }
            if self.game.action(spot, key == MouseButton::Right) {
                println!("you lose!");
                self.game.reset();
                self.first = true;
            }
            self.game.print_game();

            self.mouse_input = None;
            self.draw = true;
        }
    }
}
