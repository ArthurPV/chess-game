mod argument;
mod repl;
mod version;

use crate::argument::Argument;
use crate::repl::run_chess_repl;

fn main() {
    let arg = Argument::new();

    match arg.argc {
        1 => println!("Welcome to chess world"),
        _ => match arg.argv[1].as_str() {
            "-h" => println!("help"),
            "opening" => println!("opening"),
            "repl" => run_chess_repl(),
            _ => println!("error: unknown argument: `{}`", arg.argv[1]),
        },
    }
}
