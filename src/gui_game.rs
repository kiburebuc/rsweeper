use piston_window::*;
use piston_window::types::Color;

use crate::game;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const QUES: Color = [1.0, 0.0, 1.0, 1.0];

#[derive(Default)]
pub struct GuiGame {
    last_pos: [f64; 2],
    rect_pos: [f64; 2],
    bg_color: Color,
    click: bool,
    game: game::Game,
}

impl GuiGame {
    pub fn execute(&mut self) {
        let mut window = WindowSettings::new("Rsweeper", [200, 200])
            .exit_on_esc(true);

        window.set_vsync(true);

        let mut window: PistonWindow = window.build().unwrap();

        self.game = game::Game::default();
        self.game.resize(10, 10);
        self.bg_color = GREEN;
        
        while let Some(event) = window.next() {
            window.draw_2d(&event, |c, g, _| {
                clear(self.bg_color, g);
                self.draw(&c, g);
            });
            
            self.handle_mouse(&event);

            event.update(|arg| {
                self.update(arg);
            });
        }
    }

    fn handle_mouse(&mut self, e: &Event) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.last_pos = pos;
        }

        if let Some(Button::Mouse(_key)) = e.release_args() {
            println!("click!");
            self.bg_color = QUES;
            self.click = true;
        }
    }

    fn draw(&self, con: &Context, g: &mut G2d) {
                println!("draw");
        //graphics::rectangle(GREEN, 
            //[self.rect_pos[0], 0.0, 50.0, 50.0], con.transform, g);
    }

    fn update(&mut self, _arg: &UpdateArgs) {
        if self.click {
            self.rect_pos = self.last_pos;
            self.click = false;
        }
    }
}
