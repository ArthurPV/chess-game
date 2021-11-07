#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChessPieceColor {
    Black,
    White,
}

impl ChessPieceColor {
    pub fn inverse_color(&self) -> ChessPieceColor {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChessBishopKind {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn chess_piece_to_value(&self) -> usize {
        match self {
            Self::Pawn(_) => 1,
            Self::Bishop(_, _) => 3,
            Self::Knight(_) => 3,
            Self::Rook(_) => 5,
            Self::Queen(_) => 9,
            Self::King(_) => 0,
        }
    }

    pub fn is_edible(&self, color_piece: &ChessPieceColor) -> bool {
        match self {
            Self::Pawn(v) if !(v == color_piece) => true,
            Self::Bishop(v, _) if !(v == color_piece) => true,
            Self::Knight(v) if !(v == color_piece) => true,
            Self::Rook(v) if !(v == color_piece) => true,
            Self::Queen(v) if !(v == color_piece) => true,
            _ => false,
        }
    }
}
