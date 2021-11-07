#[derive(Debug, Clone, PartialEq)]
pub enum ChessPieceColor {
    Black,
    White,
}

#[derive(Debug, Clone)]
pub enum ChessBishopKind {
    Black,
    White,
}

#[derive(Debug, Clone)]
pub enum ChessPieceKind {
    Pawn(ChessPieceColor),
    Bishop(ChessPieceColor, ChessBishopKind),
    Knight(ChessPieceColor),
    Rook(ChessPieceColor),
    Queen(ChessPieceColor),
    King(ChessPieceColor),
}

impl ChessPieceKind {
    pub fn chess_piece_to_unicode(&self) -> &str {
        match self {
            Self::Pawn(ChessPieceColor::Black) => "♟",
            Self::Bishop(ChessPieceColor::Black, _) => "♝",
            Self::Knight(ChessPieceColor::Black) => "♞",
            Self::Rook(ChessPieceColor::Black) => "♜",
            Self::Queen(ChessPieceColor::Black) => "♛",
            Self::King(ChessPieceColor::Black) => "♚",
            Self::Pawn(ChessPieceColor::White) => "♙",
            Self::Bishop(ChessPieceColor::White, _) => "♗",
            Self::Knight(ChessPieceColor::White) => "♘",
            Self::Rook(ChessPieceColor::White) => "♖",
            Self::Queen(ChessPieceColor::White) => "♕",
            Self::King(ChessPieceColor::White) => "♔",
        }
    }
}
