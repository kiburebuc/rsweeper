//use std::{process, env};
use macroquad::prelude::*;

mod array2d;
mod game;

const GAME_SIZE: [usize; 2] = [10, 10];
const NUM_MINES: usize = 10;

const GRID_COLOR: Color = GOLD;
const CELL_COLOR: Color = GREEN;
const FLAG_COLOR: Color = RED;
const NUM_COLOR: Color = RED;

fn conf() -> Conf {
    Conf {
        window_title: String::from("rsweeper"),
        window_width: 500,
        window_height: 500,
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
    let mut first = true;
    let mut gameover;
    let mut click = None;

    let mut game = game::Game::default();
    game.resize(GAME_SIZE[0], GAME_SIZE[1]);

    loop { 
        //draw
        let screen_size = (screen_width(), screen_height());
        let grid_size = (
            screen_size.0 / game.get_size().x() as f32,
            screen_size.1 / game.get_size().y() as f32);
    
        clear_background(WHITE);

        // draw horizontal lines
        for x in 1..game.get_size().x() {
            let x = x as f32 * grid_size.0;
            draw_line(x, 0., x, screen_size.1, 5., GRID_COLOR);
        }
        
        // draw vertical lines
        for y in 1..game.get_size().y() {
            let y = y as f32 * grid_size.1;
            draw_line(0., y, screen_size.0, y, 5., GRID_COLOR);
        }

        // draw tiles
        for (co, cell) in game.grid.enumerate() {
            let co = (co.y() as f32 * grid_size.0, co.x() as f32 * grid_size.1);
            if !cell.dug() {
                draw_rectangle(co.0, co.1, grid_size.0, grid_size.1, CELL_COLOR); 
                if cell.flagged() {
                    draw_triangle(co.into(), (co.0 + grid_size.0, co.1).into(),
                    (co.0, co.1 + grid_size.1).into(), FLAG_COLOR);
                }
            } else {
                if !cell.is_zero() {
                    let num = cell.get_num();
                    draw_text(&format!("{}", num), 
                        co.0, co.1 + grid_size.1 - 10., 50., NUM_COLOR); 
                }
            }
        } 
        
        //update
        if is_mouse_button_released(MouseButton::Left) {
            println!("left");
            click = Some(false);
        } else if is_mouse_button_released(MouseButton::Right) {
            click = Some(true);
        }

        if let Some(dig_or_flag) = click {
            let (mx, my) = mouse_position();
            let spot = game::Coord(
                (my / grid_size.0) as isize, (mx / grid_size.1) as isize); 

            if first {
                game.generate_mines(NUM_MINES, game::Coord::from(spot));
                first = false;
            }
            gameover = game.action(spot, dig_or_flag);
            game.print_game();
            if gameover { 
                println!("gameover");
                return; }
            click = None;
        }


        next_frame().await
    }
}
