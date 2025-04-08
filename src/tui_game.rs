use crate::game;
use text_io::read;

pub fn execute() {
    let mut this_game = game::Game::default();
    this_game.resize(10, 10);

    // initiate game
    this_game.print_game();
    let first = get_xy();
    this_game.generate_mines(1, first.0);
    this_game.action(game::Coord::from(first.0), first.1);

    // main loop
    loop {
        this_game.print_game();
        println!("Score: {}", this_game.get_score());
        if this_game.get_score() == 0 {
            println!("You Dug Everything!  You Win!");
            return;
        }
        let (xy, flag) = get_xy();
        if this_game.action(game::Coord::from(xy), flag) {
            println!("You Hit A Mine :[[ Game Over!");
            return;
        }
    }
}

fn get_xy() -> (game::Coord, bool) {
    loop {
        print!("input(Ex: '68' or '12p'): ");
        let input: String = read!();
        let mut it = input.chars();
        let (Some(y), Some(x)) = (it.next(), it.next()) else {
            println!("Invalid input, try again!");
            continue;
        };
        if it.next() == None {
            return (game::Coord(game::from_ascii(x), game::from_ascii(y)), false);
        } else {
            (game::Coord(game::from_ascii(x), game::from_ascii(y)), true);
        }
    }
}
