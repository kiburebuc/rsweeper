//use std::{process, env};
use macroquad::{prelude::*};
mod array2d;
mod game;

type Vec2 = (f32, f32);


const GAME_SIZE: [usize; 2] = [10, 10];
const NUM_MINES: usize = 10;

const BASE: Color = Color::from_hex(0xc0c0c0);
const HIGHLIGHT: Color = WHITE;
const SHADOW: Color = Color::from_hex(0x808080);
const GRID: Color = Color::from_hex(0x808080);
const FLAG_COLOR: Color = Color::from_hex(0xfe0000);

const PADDING: f32 = 5.;


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
    let mut mouse = None;
    let mut key = None;
    let mut gameover = None;
    let mut grid;
    let mut debug = String::new();
    let mut first = true;
    let mut game = game::Game::default();
    game.resize(GAME_SIZE[0], GAME_SIZE[1]);

    loop {
        let screen_size = (screen_width(), screen_height());
        grid = (
            screen_size.0 / game.get_size().x() as f32,
            screen_size.1 / game.get_size().y() as f32);
        redraw(&mut game);
        if !debug.is_empty() {
            draw_text(&debug, 0., 100., 30., BLUE);
        }
        
        // handle gameover
        if let Some(state) = gameover {
            let msg = if state { "You Hit A Mine!" } else { "You Won! B]" };
            let msg1 = "Click to reset!";
            let center = get_text_center(&msg1, None, 50, 1., 0.);
            println!("{}", center);
            let dim = measure_text(msg, None, 50, 1.);
            draw_rectangle(0., 0., dim.width + PADDING * 2.,
                dim.height + PADDING * 2., BLACK);
            let dim = measure_text(msg1, None, 50, 1.);
            draw_rectangle(0., dim.height + PADDING * 2., dim.width + PADDING * 2.,
                dim.height + PADDING * 2., BLACK);
            draw_text(msg, PADDING, dim.height + PADDING, 50., FLAG_COLOR);
            draw_text(msg1, PADDING, dim.height * 2. + PADDING * 3., 50., FLAG_COLOR);
        }

        if is_mouse_button_released(MouseButton::Left) {
            mouse = Some(false);
        } else if is_mouse_button_released(MouseButton::Right) {
            mouse = Some(true);
        }

        if let Some(dig_or_flag) = mouse {
            if gameover.is_none() {
                let (my, mx) = mouse_position();
                let spot = game::Coord(
                    (mx / grid.1) as isize, (my / grid.0) as isize);


                //debug = format!("Clicked at {} {} = spot {} {}",
                    //mx, my, spot.x(), spot.y());

                if first {
                    game.generate_mines(NUM_MINES, game::Coord::from(spot));
                    first = false;
                }
                if game.action(spot, dig_or_flag) { gameover = Some(true); }
                game.print_game();
                if game.get_score() == 0 {
                    println!("gamewon!");
                    gameover = Some(false);
                }
            } else {
                game.reset();
                first = true;
                gameover = None;
            }
            mouse = None;
        }
        // DONE UPDATE
        //
        next_frame().await
    }
}


fn redraw(game: & mut game::Game) {
        println!("left");
        clear_background(BASE);

        let screen = (screen_width(), screen_height());
        let grid = (screen.0 / game.get_size().x() as f32,
            screen.1 / game.get_size().y() as f32);

        // draw horizontal lines
        for x in 1..game.get_size().x() {
            let x = x as f32 * grid.0;
            draw_line(x, 0., x, screen.1, 3., GRID);
        }

        // draw vertical lines
        for y in 1..game.get_size().y() {
            let y = y as f32 * grid.1;
            draw_line(0., y, screen.0, y, 3., GRID);
        }

        // draw tiles
        for (co, cell) in game.grid.enumerate() {
            let co = (co.y() as f32 * grid.0, co.x() as f32 * grid.1);
            let dug = if cell.dug() { Some(cell.get_num()) } else { None };
            draw_tile(co, grid, dug, cell.flagged());
        }

        // DONE DRAWING
}

fn map_num(n: u8) -> Color {
    match n {
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
fn draw_tile(pos: (f32, f32), grid: (f32, f32), dug: Option<u8>, flag: bool) {
    if let Some(num) = dug {
        if num != 0 {
            let text = format!("{}", num);
            let center = get_text_center(&text, None, 50, 1., 0.);
            let tx = pos.0 - center.x + grid.0 / 2.;
            let ty = pos.1 - center.y + grid.1 / 2.;
            draw_text(&text, tx, ty, 50., map_num(num)); 
        }
    } else {
        draw_raised_rect(pos, grid, true, 5.);

        if flag {
            let center = get_text_center("P", None, 50, 1., 0.);
            let tx = pos.0 - center.x + grid.0 / 2.;
            let ty = pos.1 - center.y + grid.1 / 2.;
            draw_text("P", tx, ty, 50., map_num(3)); 
        }
    }
}

//    _______         _______ 
//   |      /|       |  ___ /|
//   |    /  |  -->  | |   | |
//   |  /    |       | |___| |
//   |/______|       |/______|
//
//                       x y         w h
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
