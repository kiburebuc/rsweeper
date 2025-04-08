//use std::{process, env};
mod array2d;
mod gui_game;
mod game;


//handle args, spawn app
fn main() {
    /*let args: Vec<String> = env::args().collect();
    if args.len() <= 2 { println!("Provide args!"); process::exit(1); }
    
    let mut dim: Vec<usize> = Vec::new();
    for i in 1..=2 {
        dim.push(args[i].parse().expect("Arg {i} is not integer"));
    }
    
    println!("{} {}", dim[0], dim[1]);
    */
    let mut newgame = gui_game::GuiGame::default();
    newgame.execute();
}
