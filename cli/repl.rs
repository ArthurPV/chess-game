use std::io::Write;

use crate::version::CHESS_GAME_VERSION;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use chess_game::ChessBox;
use chess_game::ChessPieceColor;
use chess_game::ChessPlayer;
use chess_game::Tray;

pub enum ReplCommand {
    Help,
    Start,
    Opening,
    Empty,
    Unknown,
}

impl ReplCommand {
    pub fn string_to_repl_command(s: &str) -> ReplCommand {
        match s {
            "help" => ReplCommand::Help,
            "start" => ReplCommand::Start,
            "opening" => ReplCommand::Opening,
            "" => ReplCommand::Empty,
            _ => ReplCommand::Unknown,
        }
    }
}

const HELP_STR: &str = "Welcome to help chess game

Command:

    help                    Print help
    start                   Start game
    opening                 List all openings";

pub fn run_help_repl_command() {
    println!("{}", HELP_STR);
}

pub fn run_start_repl_command() {
    let mut answer_random_choice = String::new();
    let mut answer_color_choice = String::new();

    loop {
        print!("Do you want to choose your color or make it random (y/n)? ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut answer_random_choice)
            .unwrap();

        match answer_random_choice.as_str() {
            "y\n" | "yes\n" | "\n" => {
                loop {
                    print!("What color do you want to choose (b/w)? ");
                    std::io::stdout().flush().unwrap();
                    std::io::stdin()
                        .read_line(&mut answer_color_choice)
                        .unwrap();

                    match answer_color_choice.as_str() {
                        "black\n" | "white\n" | "b\n" | "w\n" | "\n" => break,
                        _ => (),
                    }
                }
                break;
            }
            "n\n" | "no\n" => break,
            _ => (),
        }
    }

    if answer_random_choice != "n\n" || answer_random_choice != "no\n" {
        let mut tray = Tray::new();
        tray.build_new_tray();
        tray.build_default_game_tray();
        let chess_player1 = ChessPlayer::new_player_1(ChessPieceColor::str_to_piece_color(
            &answer_color_choice[..answer_color_choice.len() - 1],
        ));
        let chess_player2 = ChessPlayer::new_player_2(ChessPieceColor::inverse_color(
            &ChessPieceColor::str_to_piece_color(
                &answer_color_choice[..answer_color_choice.len() - 1],
            ),
        ));

        match chess_player1 {
            ChessPlayer::Player1(ChessPieceColor::White) => loop {
                let mut chess_move = String::new();

                tray.print_tray(ChessPieceColor::White);
                print!("::: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut chess_move).unwrap();

                match &chess_move[..chess_move.len() - 1] {
                    "quit" => break,
                    "q" => break,
                    _ => (),
                }

                println!("To computer to play");
            },
            _ => loop {
                let mut chess_move = String::new();
                tray.print_tray(ChessPieceColor::Black);
                print!("::: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut chess_move).unwrap();

                match &chess_move[..chess_move.len() - 1] {
                    "quit" => break,
                    "q" => break,
                    _ => (),
                }

                println!("To computer to play");
            },
        }
    }
}

pub fn run_opening_repl_command() {}

pub fn run_chess_repl() {
    let mut rl = Editor::<()>::new();
    println!(
        "Welcome to chess game \x1b[32mv{}\x1b[0m",
        CHESS_GAME_VERSION
    );

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => match ReplCommand::string_to_repl_command(line.as_str()) {
                ReplCommand::Help => run_help_repl_command(),
                ReplCommand::Start => run_start_repl_command(),
                ReplCommand::Opening => println!("opening"),
                ReplCommand::Empty => (),
                ReplCommand::Unknown => println!("error: unknown command: `{}`", &line),
            },
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => break,
        }
    }
}
