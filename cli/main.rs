mod argument;

use crate::argument::Argument;

use chess_game::ChessBox;
use chess_game::Tray;

fn main() {
    let arg = Argument::new();

    match arg.argc {
        1 => println!("Welcome to chess world"),
        _ => match arg.argv[1].as_str() {
            "-h" => println!("help"),
            "opening" => println!("opening"),
            _ => println!("error: unknown argument: `{}`", arg.argv[1]),
        },
    }

    let mut tray = Tray::new();
    tray.build_new_tray();
    tray.build_default_game_tray();
    println!("{}", tray.render_tray_to_string());
}
