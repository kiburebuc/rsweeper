//use std::{process, env};
use macroquad::{prelude::*};
mod array2d;
mod game;
mod rsweeper;
mod widget;

use rsweeper::{Rsweeper};
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
    let mut mouse = None;
    let bg = Texture2D::from_file_with_format(include_bytes!("bliss.png"), None);

    let mut win = Rsweeper::default();

    loop {
        let max = f32::max(screen_width(), screen_height() * ASPRAT);
        draw_texture_ex(&bg, 0., 0., WHITE, DrawTextureParams{
            dest_size: Some(vec2(max, max / ASPRAT)), ..Default::default()
        }); //bliss bg


        win.draw();
        
        if is_mouse_button_released(MouseButton::Left) {
            mouse = Some((mouse_position(), false));
        } else if is_mouse_button_released(MouseButton::Right) {
            mouse = Some((mouse_position(), true));
        }

        win.update(mouse);
        mouse = None;

        next_frame().await
    }
}
