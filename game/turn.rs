use crate::piece::ChessPieceColor;
use crate::piece::ChessPieceKind;
use crate::tray::ChessBox;

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
    Move(ChessPieceKind, ChessBox), // piece move and where he move
    Eat(ChessPieceKind, ChessBox),
    Rook(ChessPieceKind, ChessBox),
    Check(ChessPieceKind, ChessBox),
    Checkmate(ChessPieceKind, ChessBox),
}

impl ChessTurnAction {
    pub fn new_move(piece_kind: ChessPieceKind, box_kind: ChessBox) -> ChessTurnAction {
        ChessTurnAction::Move(piece_kind, box_kind)
    }

    pub fn new_eat(piece_kind: ChessPieceKind, box_kind: ChessBox) -> ChessTurnAction {
        ChessTurnAction::Eat(piece_kind, box_kind)
    }

    pub fn new_rook(piece_kind: ChessPieceKind, box_kind: ChessBox) -> ChessTurnAction {
        ChessTurnAction::Rook(piece_kind, box_kind)
    }

    pub fn new_check(piece_kind: ChessPieceKind, box_kind: ChessBox) -> ChessTurnAction {
        ChessTurnAction::Check(piece_kind, box_kind)
    }

    pub fn new_checkmate(piece_kind: ChessPieceKind, box_kind: ChessBox) -> ChessTurnAction {
        ChessTurnAction::Checkmate(piece_kind, box_kind)
    }
}