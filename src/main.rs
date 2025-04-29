//use std::{process, env};
use macroquad::{prelude::*};
mod array2d;
mod game;
mod rsweeper;
mod widget;
mod window;

use rsweeper::Rsweeper;
use window::Window;
use widget::*;

const ASPRAT: f32 = 16. / 9.;

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
    let mut mouse = Mouse::None;
    let bg = Texture2D::from_file_with_format(include_bytes!("bliss.png"), None);

    let mut win = Window::<Rsweeper>::default();
    win.set_size((158., 239.).into());
    win.app.game.game.resize(8, 8);

    loop {
        let max = f32::max(screen_width(), screen_height() * ASPRAT);
        draw_texture_ex(&bg, 0., 0., WHITE, DrawTextureParams{
            dest_size: Some(vec2(max, max / ASPRAT)), ..Default::default()
        }); //bliss bg

        if is_mouse_button_down(MouseButton::Left) {
            mouse = Mouse::Drag(mouse_position().into(), false);
        } else if is_mouse_button_down(MouseButton::Right) {
            mouse = Mouse::Drag(mouse_position().into(), true);
        } else {
            match mouse {
                Mouse::Drag(p, b) => { mouse = Mouse::Released(p, b) },
                Mouse::Released(_, _) => { mouse = Mouse::None },
                Mouse::None => {},
            }
        }
        
        *win.mouse_mut() = mouse;
        win.update();
        win.draw();

        next_frame().await
    }
}
