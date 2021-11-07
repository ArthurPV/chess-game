use crate::piece::ChessPieceColor;
use crate::tray::ChessBoxKind;

pub enum ChessPlayer {
    Player1(ChessPieceColor),
    Player2(ChessPieceColor),
}

impl ChessPlayer {
    pub fn new_player_1(color: ChessPieceColor) -> ChessPlayer {
        ChessPlayer::Player1(color)
    }

    pub fn new_player_2(color: ChessPieceColor) -> ChessPlayer {
        ChessPlayer::Player2(color)
    }
}

pub enum ChessTurnAction {
    Move(ChessBoxKind),
    Eat(ChessBoxKind),
    Rook(ChessBoxKind),
    Check(ChessBoxKind),
    Checkmate(ChessBoxKind),
}

impl ChessTurnAction {
    pub fn new_move(box_kind: ChessBoxKind) -> ChessTurnAction {
        ChessTurnAction::Move(box_kind)
    }

    pub fn new_eat(box_kind: ChessBoxKind) -> ChessTurnAction {
        ChessTurnAction::Eat(box_kind)
    }

    pub fn new_rook(box_kind: ChessBoxKind) -> ChessTurnAction {
        ChessTurnAction::Rook(box_kind)
    }

    pub fn new_check(box_kind: ChessBoxKind) -> ChessTurnAction {
        ChessTurnAction::Check(box_kind)
    }

    pub fn new_checkmate(box_kind: ChessBoxKind) -> ChessTurnAction {
        ChessTurnAction::Checkmate(box_kind)
    }
}
