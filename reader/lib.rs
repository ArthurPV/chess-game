use chess_game::CastleKind;
use chess_game::ChessBox;
use chess_game::ChessBoxKind;
use chess_game::ChessPieceColor;
use chess_game::ChessPieceKind;
use chess_game::ChessTurnAction;
use chess_game::Tray;

fn is_valid_box_str(box_str: &str) -> bool {
    match box_str {
        "a1" | "b1" | "c1" | "d1" | "e1" | "f1" | "g1" | "h1" | "a2" | "b2" | "c2" | "d2"
        | "e2" | "f2" | "g2" | "h2" | "a3" | "b3" | "c3" | "f3" | "g3" | "h3" | "a4" | "b4"
        | "c4" | "d4" | "e4" | "f4" | "g4" | "h4" | "a5" | "b5" | "c5" | "d5" | "e5" | "f5"
        | "g5" | "h5" | "a6" | "b6" | "c6" | "d6" | "e6" | "f6" | "g6" | "h6" | "a7" | "b7"
        | "c7" | "d7" | "e7" | "f7" | "g7" | "h7" | "a8" | "b8" | "c8" | "d8" | "e8" | "f8"
        | "g8" | "h8" => true,
        _ => false,
    }
}

fn color_to_kingside(color_kind: ChessPieceColor) -> ChessBoxKind {
    match color_kind {
        ChessPieceColor::Black => ChessBoxKind::G8,
        ChessPieceColor::White => ChessBoxKind::G1,
    }
}

fn color_to_queenside(color_kind: ChessPieceColor) -> ChessBoxKind {
    match color_kind {
        ChessPieceColor::Black => ChessBoxKind::C8,
        ChessPieceColor::White => ChessBoxKind::C1,
    }
}

pub fn str_move_to_action<'a>(
    tray: &'a mut Tray,
    chess_move: &'a str,
    color_kind: ChessPieceColor,
) -> Result<ChessTurnAction, &'a str> {
    match chess_move.chars().nth(0).unwrap() {
        'O' => {
            if chess_move == "O-O" {
                return Ok(ChessTurnAction::Castle(
                    CastleKind::Kingside,
                    tray.get_box(&color_to_kingside(color_kind)),
                ));
            } else if chess_move == "O-O-O" {
                return Ok(ChessTurnAction::Castle(
                    CastleKind::Queenside,
                    tray.get_box(&color_to_queenside(color_kind)),
                ));
            } else {
                return Err("invalid move");
            }
        }
        'B' => {
            if chess_move.len() == 3 {
                match is_valid_box_str(&chess_move[1..]) {
                    true => {
                        todo!()
                        // return Ok(ChessTurnAction::Move(ChessPieceKind::Bishop(color_kind)));
                    }
                    false => return Err("invalid move"),
                }
                // move
            } else if chess_move.chars().nth(3).unwrap() == '+' {
                // check
                todo!()
            } else if chess_move.chars().nth(3).unwrap() == '#' {
                // checkmate
                todo!()
            } else {
                todo!()
            }
        }
        'N' => todo!(),
        'R' => todo!(),
        'Q' => todo!(),
        'K' => todo!(),
        _ => Err("invalid move"),
    }
}
