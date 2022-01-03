use crate::{piece::*, ChessPlayer};
use colored::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ChessBoxColor {
    Black,
    White,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChessBoxKind {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    Unknown,
}

impl ChessBoxKind {
    pub fn get_column_code(&self) -> usize {
        match &self {
            Self::A1 => 1,
            Self::B1 => 2,
            Self::C1 => 3,
            Self::D1 => 4,
            Self::E1 => 5,
            Self::F1 => 6,
            Self::G1 => 7,
            Self::H1 => 8,
            Self::A2 => 1,
            Self::B2 => 2,
            Self::C2 => 3,
            Self::D2 => 4,
            Self::E2 => 5,
            Self::F2 => 6,
            Self::G2 => 7,
            Self::H2 => 8,
            Self::A3 => 1,
            Self::B3 => 2,
            Self::C3 => 3,
            Self::D3 => 4,
            Self::E3 => 5,
            Self::F3 => 6,
            Self::G3 => 7,
            Self::H3 => 8,
            Self::A4 => 1,
            Self::B4 => 2,
            Self::C4 => 3,
            Self::D4 => 4,
            Self::E4 => 5,
            Self::F4 => 6,
            Self::G4 => 7,
            Self::H4 => 8,
            Self::A5 => 1,
            Self::B5 => 2,
            Self::C5 => 3,
            Self::D5 => 4,
            Self::E5 => 5,
            Self::F5 => 6,
            Self::G5 => 7,
            Self::H5 => 8,
            Self::A6 => 1,
            Self::B6 => 2,
            Self::C6 => 3,
            Self::D6 => 4,
            Self::E6 => 5,
            Self::F6 => 6,
            Self::G6 => 7,
            Self::H6 => 8,
            Self::A7 => 1,
            Self::B7 => 2,
            Self::C7 => 3,
            Self::D7 => 4,
            Self::E7 => 5,
            Self::F7 => 6,
            Self::G7 => 7,
            Self::H7 => 8,
            Self::A8 => 1,
            Self::B8 => 2,
            Self::C8 => 3,
            Self::D8 => 4,
            Self::E8 => 5,
            Self::F8 => 6,
            Self::G8 => 7,
            Self::H8 => 8,
            Self::Unknown => 404,
        }
    }

    pub fn get_line_code(&self) -> usize {
        match &self {
            Self::A1
            | Self::B1
            | Self::C1
            | Self::D1
            | Self::E1
            | Self::F1
            | Self::G1
            | Self::H1 => 1,
            Self::A2
            | Self::B2
            | Self::C2
            | Self::D2
            | Self::E2
            | Self::F2
            | Self::G2
            | Self::H2 => 2,
            Self::A3
            | Self::B3
            | Self::C3
            | Self::D3
            | Self::E3
            | Self::F3
            | Self::G3
            | Self::H3 => 3,
            Self::A4
            | Self::B4
            | Self::C4
            | Self::D4
            | Self::E4
            | Self::F4
            | Self::G4
            | Self::H4 => 4,
            Self::A5
            | Self::B5
            | Self::C5
            | Self::D5
            | Self::E5
            | Self::F5
            | Self::G5
            | Self::H5 => 5,
            Self::A6
            | Self::B6
            | Self::C6
            | Self::D6
            | Self::E6
            | Self::F6
            | Self::G6
            | Self::H6 => 6,
            Self::A7
            | Self::B7
            | Self::C7
            | Self::D7
            | Self::E7
            | Self::F7
            | Self::G7
            | Self::H7 => 7,
            Self::A8
            | Self::B8
            | Self::C8
            | Self::D8
            | Self::E8
            | Self::F8
            | Self::G8
            | Self::H8 => 8,
            Self::Unknown => 404,
        }
    }

    // (line, column)
    pub fn location_to_box_kind(loc: (usize, usize)) -> ChessBoxKind {
        match loc {
            (1, 1) => ChessBoxKind::A1,
            (1, 2) => ChessBoxKind::B1,
            (1, 3) => ChessBoxKind::C1,
            (1, 4) => ChessBoxKind::D1,
            (1, 5) => ChessBoxKind::E1,
            (1, 6) => ChessBoxKind::F1,
            (1, 7) => ChessBoxKind::G1,
            (1, 8) => ChessBoxKind::H1,
            (2, 1) => ChessBoxKind::A2,
            (2, 2) => ChessBoxKind::B2,
            (2, 3) => ChessBoxKind::C2,
            (2, 4) => ChessBoxKind::D2,
            (2, 5) => ChessBoxKind::E2,
            (2, 6) => ChessBoxKind::F2,
            (2, 7) => ChessBoxKind::G2,
            (2, 8) => ChessBoxKind::H2,
            (3, 1) => ChessBoxKind::A3,
            (3, 2) => ChessBoxKind::B3,
            (3, 3) => ChessBoxKind::C3,
            (3, 4) => ChessBoxKind::D3,
            (3, 5) => ChessBoxKind::E3,
            (3, 6) => ChessBoxKind::F3,
            (3, 7) => ChessBoxKind::G3,
            (3, 8) => ChessBoxKind::H3,
            (4, 1) => ChessBoxKind::A4,
            (4, 2) => ChessBoxKind::B4,
            (4, 3) => ChessBoxKind::C4,
            (4, 4) => ChessBoxKind::D4,
            (4, 5) => ChessBoxKind::E4,
            (4, 6) => ChessBoxKind::F4,
            (4, 7) => ChessBoxKind::G4,
            (4, 8) => ChessBoxKind::H4,
            (5, 1) => ChessBoxKind::A5,
            (5, 2) => ChessBoxKind::B5,
            (5, 3) => ChessBoxKind::C5,
            (5, 4) => ChessBoxKind::D5,
            (5, 5) => ChessBoxKind::E5,
            (5, 6) => ChessBoxKind::F5,
            (5, 7) => ChessBoxKind::G5,
            (5, 8) => ChessBoxKind::H5,
            (6, 1) => ChessBoxKind::A6,
            (6, 2) => ChessBoxKind::B6,
            (6, 3) => ChessBoxKind::C6,
            (6, 4) => ChessBoxKind::D6,
            (6, 5) => ChessBoxKind::E6,
            (6, 6) => ChessBoxKind::F6,
            (6, 7) => ChessBoxKind::G6,
            (6, 8) => ChessBoxKind::H6,
            (7, 1) => ChessBoxKind::A7,
            (7, 2) => ChessBoxKind::B7,
            (7, 3) => ChessBoxKind::C7,
            (7, 4) => ChessBoxKind::D7,
            (7, 5) => ChessBoxKind::E7,
            (7, 6) => ChessBoxKind::F7,
            (7, 7) => ChessBoxKind::G7,
            (7, 8) => ChessBoxKind::H7,
            (8, 1) => ChessBoxKind::A8,
            (8, 2) => ChessBoxKind::B8,
            (8, 3) => ChessBoxKind::C8,
            (8, 4) => ChessBoxKind::D8,
            (8, 5) => ChessBoxKind::E8,
            (8, 6) => ChessBoxKind::F8,
            (8, 7) => ChessBoxKind::G8,
            (8, 8) => ChessBoxKind::H8,
            _ => ChessBoxKind::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChessBox {
    pub kind: ChessBoxKind,
    pub color: ChessBoxColor,
    pub piece: Option<ChessPieceKind>,
}

impl ChessBox {
    pub fn new(
        kind: ChessBoxKind,
        color: ChessBoxColor,
        piece: Option<ChessPieceKind>,
    ) -> ChessBox {
        ChessBox { kind, color, piece }
    }
}

#[derive(Debug, Clone)]
pub struct Tray {
    pub tray: (
        Vec<ChessBox>,
        Vec<ChessBox>,
        Vec<ChessBox>,
        Vec<ChessBox>,
        Vec<ChessBox>,
        Vec<ChessBox>,
        Vec<ChessBox>,
        Vec<ChessBox>,
    ),
    // tray.0 = a1..h1
    // tray.1 = a2..h2
    // tray.2 = a3..h3
    // tray.3 = a4..h4
    // tray.4 = a5..h5
    // tray.5 = a6..h6
    // tray.6 = a7..h7
    // tray.7 = a8..h8
    pub active_black_piece: Vec<ChessBox>,
    pub active_white_piece: Vec<ChessBox>,
    pub eat_black_piece: Vec<ChessPieceKind>,
    pub eat_white_piece: Vec<ChessPieceKind>,
}

impl Tray {
    pub fn new() -> Tray {
        Tray {
            tray: (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            active_black_piece: Vec::new(),
            active_white_piece: Vec::new(),
            eat_black_piece: Vec::new(),
            eat_white_piece: Vec::new(),
        }
    }

    pub fn build_new_tray(&mut self) {
        // build a1..h1 line
        self.tray
            .0
            .push(ChessBox::new(ChessBoxKind::A1, ChessBoxColor::Black, None));
        self.tray
            .0
            .push(ChessBox::new(ChessBoxKind::B1, ChessBoxColor::White, None));
        self.tray
            .0
            .push(ChessBox::new(ChessBoxKind::C1, ChessBoxColor::Black, None));
        self.tray
            .0
            .push(ChessBox::new(ChessBoxKind::D1, ChessBoxColor::White, None));
        self.tray
            .0
            .push(ChessBox::new(ChessBoxKind::E1, ChessBoxColor::Black, None));
        self.tray
            .0
            .push(ChessBox::new(ChessBoxKind::F1, ChessBoxColor::White, None));
        self.tray
            .0
            .push(ChessBox::new(ChessBoxKind::G1, ChessBoxColor::Black, None));
        self.tray
            .0
            .push(ChessBox::new(ChessBoxKind::H1, ChessBoxColor::White, None));

        // build a2..h2 line
        self.tray
            .1
            .push(ChessBox::new(ChessBoxKind::A2, ChessBoxColor::White, None));
        self.tray
            .1
            .push(ChessBox::new(ChessBoxKind::B2, ChessBoxColor::Black, None));
        self.tray
            .1
            .push(ChessBox::new(ChessBoxKind::C2, ChessBoxColor::White, None));
        self.tray
            .1
            .push(ChessBox::new(ChessBoxKind::D2, ChessBoxColor::Black, None));
        self.tray
            .1
            .push(ChessBox::new(ChessBoxKind::E2, ChessBoxColor::White, None));
        self.tray
            .1
            .push(ChessBox::new(ChessBoxKind::F2, ChessBoxColor::Black, None));
        self.tray
            .1
            .push(ChessBox::new(ChessBoxKind::G2, ChessBoxColor::White, None));
        self.tray
            .1
            .push(ChessBox::new(ChessBoxKind::H2, ChessBoxColor::Black, None));

        // build a3..h3 line
        self.tray
            .2
            .push(ChessBox::new(ChessBoxKind::A3, ChessBoxColor::Black, None));
        self.tray
            .2
            .push(ChessBox::new(ChessBoxKind::B3, ChessBoxColor::White, None));
        self.tray
            .2
            .push(ChessBox::new(ChessBoxKind::C3, ChessBoxColor::Black, None));
        self.tray
            .2
            .push(ChessBox::new(ChessBoxKind::D3, ChessBoxColor::White, None));
        self.tray
            .2
            .push(ChessBox::new(ChessBoxKind::E3, ChessBoxColor::Black, None));
        self.tray
            .2
            .push(ChessBox::new(ChessBoxKind::F3, ChessBoxColor::White, None));
        self.tray
            .2
            .push(ChessBox::new(ChessBoxKind::G3, ChessBoxColor::Black, None));
        self.tray
            .2
            .push(ChessBox::new(ChessBoxKind::H3, ChessBoxColor::White, None));

        // build a4..h4 line
        self.tray
            .3
            .push(ChessBox::new(ChessBoxKind::A4, ChessBoxColor::White, None));
        self.tray
            .3
            .push(ChessBox::new(ChessBoxKind::B4, ChessBoxColor::Black, None));
        self.tray
            .3
            .push(ChessBox::new(ChessBoxKind::C4, ChessBoxColor::White, None));
        self.tray
            .3
            .push(ChessBox::new(ChessBoxKind::D4, ChessBoxColor::Black, None));
        self.tray
            .3
            .push(ChessBox::new(ChessBoxKind::E4, ChessBoxColor::White, None));
        self.tray
            .3
            .push(ChessBox::new(ChessBoxKind::F4, ChessBoxColor::Black, None));
        self.tray
            .3
            .push(ChessBox::new(ChessBoxKind::G4, ChessBoxColor::White, None));
        self.tray
            .3
            .push(ChessBox::new(ChessBoxKind::H4, ChessBoxColor::Black, None));

        // build a5..h5 line
        self.tray
            .4
            .push(ChessBox::new(ChessBoxKind::A5, ChessBoxColor::Black, None));
        self.tray
            .4
            .push(ChessBox::new(ChessBoxKind::B5, ChessBoxColor::White, None));
        self.tray
            .4
            .push(ChessBox::new(ChessBoxKind::C5, ChessBoxColor::Black, None));
        self.tray
            .4
            .push(ChessBox::new(ChessBoxKind::D5, ChessBoxColor::White, None));
        self.tray
            .4
            .push(ChessBox::new(ChessBoxKind::E5, ChessBoxColor::Black, None));
        self.tray
            .4
            .push(ChessBox::new(ChessBoxKind::F5, ChessBoxColor::White, None));
        self.tray
            .4
            .push(ChessBox::new(ChessBoxKind::G5, ChessBoxColor::Black, None));
        self.tray
            .4
            .push(ChessBox::new(ChessBoxKind::H5, ChessBoxColor::White, None));

        // build a6..h6 line
        self.tray
            .5
            .push(ChessBox::new(ChessBoxKind::A6, ChessBoxColor::White, None));
        self.tray
            .5
            .push(ChessBox::new(ChessBoxKind::B6, ChessBoxColor::Black, None));
        self.tray
            .5
            .push(ChessBox::new(ChessBoxKind::C6, ChessBoxColor::White, None));
        self.tray
            .5
            .push(ChessBox::new(ChessBoxKind::D6, ChessBoxColor::Black, None));
        self.tray
            .5
            .push(ChessBox::new(ChessBoxKind::E6, ChessBoxColor::White, None));
        self.tray
            .5
            .push(ChessBox::new(ChessBoxKind::F6, ChessBoxColor::Black, None));
        self.tray
            .5
            .push(ChessBox::new(ChessBoxKind::G6, ChessBoxColor::White, None));
        self.tray
            .5
            .push(ChessBox::new(ChessBoxKind::H6, ChessBoxColor::Black, None));

        // build a7..h7 line
        self.tray
            .6
            .push(ChessBox::new(ChessBoxKind::A7, ChessBoxColor::Black, None));
        self.tray
            .6
            .push(ChessBox::new(ChessBoxKind::B7, ChessBoxColor::White, None));
        self.tray
            .6
            .push(ChessBox::new(ChessBoxKind::C7, ChessBoxColor::Black, None));
        self.tray
            .6
            .push(ChessBox::new(ChessBoxKind::D7, ChessBoxColor::White, None));
        self.tray
            .6
            .push(ChessBox::new(ChessBoxKind::E7, ChessBoxColor::Black, None));
        self.tray
            .6
            .push(ChessBox::new(ChessBoxKind::F7, ChessBoxColor::White, None));
        self.tray
            .6
            .push(ChessBox::new(ChessBoxKind::G7, ChessBoxColor::Black, None));
        self.tray
            .6
            .push(ChessBox::new(ChessBoxKind::H7, ChessBoxColor::White, None));

        // build a8..h8 line
        self.tray
            .7
            .push(ChessBox::new(ChessBoxKind::A8, ChessBoxColor::White, None));
        self.tray
            .7
            .push(ChessBox::new(ChessBoxKind::B8, ChessBoxColor::Black, None));
        self.tray
            .7
            .push(ChessBox::new(ChessBoxKind::C8, ChessBoxColor::White, None));
        self.tray
            .7
            .push(ChessBox::new(ChessBoxKind::D8, ChessBoxColor::Black, None));
        self.tray
            .7
            .push(ChessBox::new(ChessBoxKind::E8, ChessBoxColor::White, None));
        self.tray
            .7
            .push(ChessBox::new(ChessBoxKind::F8, ChessBoxColor::Black, None));
        self.tray
            .7
            .push(ChessBox::new(ChessBoxKind::G8, ChessBoxColor::White, None));
        self.tray
            .7
            .push(ChessBox::new(ChessBoxKind::H8, ChessBoxColor::Black, None));
    }

    pub fn build_default_game_tray(&mut self) {
        // a1 = rook white
        self.tray.0[0].piece = Some(ChessPieceKind::Rook(ChessPieceColor::White));

        // b1 = knight white
        self.tray.0[1].piece = Some(ChessPieceKind::Knight(ChessPieceColor::White));

        // c1 = bishop white
        self.tray.0[2].piece = Some(ChessPieceKind::Bishop(
            ChessPieceColor::White,
            ChessBishopKind::Black,
        ));

        // d1 = queen white
        self.tray.0[3].piece = Some(ChessPieceKind::Queen(ChessPieceColor::White));

        // e1 = king white
        self.tray.0[4].piece = Some(ChessPieceKind::King(ChessPieceColor::White));

        // f1 = bishop white
        self.tray.0[5].piece = Some(ChessPieceKind::Bishop(
            ChessPieceColor::White,
            ChessBishopKind::White,
        ));

        // g1 = knight white
        self.tray.0[6].piece = Some(ChessPieceKind::Knight(ChessPieceColor::White));

        // h1 = rook white
        self.tray.0[7].piece = Some(ChessPieceKind::Rook(ChessPieceColor::White));

        // a2 = pawn white
        self.tray.1[0].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::White));

        // b2 = pawn white
        self.tray.1[1].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::White));

        // c2 = pawn white
        self.tray.1[2].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::White));

        // d2 = pawn white
        self.tray.1[3].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::White));

        // e2 = pawn white
        self.tray.1[4].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::White));

        // f2 = pawn white
        self.tray.1[5].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::White));

        // g2 = pawn white
        self.tray.1[6].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::White));

        // h2 = pawn white
        self.tray.1[7].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::White));

        // a8 = rook black
        self.tray.7[0].piece = Some(ChessPieceKind::Rook(ChessPieceColor::Black));

        // b8 = knight black
        self.tray.7[1].piece = Some(ChessPieceKind::Knight(ChessPieceColor::Black));

        // c8 = bishop black
        self.tray.7[2].piece = Some(ChessPieceKind::Bishop(
            ChessPieceColor::Black,
            ChessBishopKind::White,
        ));

        // d8 = queen black
        self.tray.7[3].piece = Some(ChessPieceKind::Queen(ChessPieceColor::Black));

        // e8 = king black
        self.tray.7[4].piece = Some(ChessPieceKind::King(ChessPieceColor::Black));

        // f8 = bishop black
        self.tray.7[5].piece = Some(ChessPieceKind::Bishop(
            ChessPieceColor::Black,
            ChessBishopKind::Black,
        ));

        // g8 = knight black
        self.tray.7[6].piece = Some(ChessPieceKind::Knight(ChessPieceColor::Black));

        // h8 = rook black
        self.tray.7[7].piece = Some(ChessPieceKind::Rook(ChessPieceColor::Black));

        // a7 = pawn black
        self.tray.6[0].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::Black));

        // b7 = pawn black
        self.tray.6[1].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::Black));

        // c7 = pawn black
        self.tray.6[2].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::Black));

        // d7 = pawn black
        self.tray.6[3].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::Black));

        // e7 = pawn black
        self.tray.6[4].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::Black));

        // f7 = pawn black
        self.tray.6[5].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::Black));

        // g7 = pawn black
        self.tray.6[6].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::Black));

        // h7 = pawn black
        self.tray.6[7].piece = Some(ChessPieceKind::Pawn(ChessPieceColor::Black));
    }

    pub fn get_box(&self, box_kind: &ChessBoxKind) -> ChessBox {
        match box_kind {
            &ChessBoxKind::A1
            | &ChessBoxKind::B1
            | &ChessBoxKind::C1
            | &ChessBoxKind::D1
            | &ChessBoxKind::E1
            | &ChessBoxKind::F1
            | &ChessBoxKind::G1
            | &ChessBoxKind::H1 => {
                self.tray.0[ChessBoxKind::get_column_code(&box_kind) - 1].clone()
            }
            &ChessBoxKind::A2
            | &ChessBoxKind::B2
            | &ChessBoxKind::C2
            | &ChessBoxKind::D2
            | &ChessBoxKind::E2
            | &ChessBoxKind::F2
            | &ChessBoxKind::G2
            | &ChessBoxKind::H2 => {
                self.tray.1[ChessBoxKind::get_column_code(&box_kind) - 1].clone()
            }
            &ChessBoxKind::A3
            | &ChessBoxKind::B3
            | &ChessBoxKind::C3
            | &ChessBoxKind::D3
            | &ChessBoxKind::E3
            | &ChessBoxKind::F3
            | &ChessBoxKind::G3
            | &ChessBoxKind::H3 => {
                self.tray.2[ChessBoxKind::get_column_code(&box_kind) - 1].clone()
            }
            &ChessBoxKind::A4
            | &ChessBoxKind::B4
            | &ChessBoxKind::C4
            | &ChessBoxKind::D4
            | &ChessBoxKind::E4
            | &ChessBoxKind::F4
            | &ChessBoxKind::G4
            | &ChessBoxKind::H4 => {
                self.tray.3[ChessBoxKind::get_column_code(&box_kind) - 1].clone()
            }
            &ChessBoxKind::A5
            | &ChessBoxKind::B5
            | &ChessBoxKind::C5
            | &ChessBoxKind::D5
            | &ChessBoxKind::E5
            | &ChessBoxKind::F5
            | &ChessBoxKind::G5
            | &ChessBoxKind::H5 => {
                self.tray.4[ChessBoxKind::get_column_code(&box_kind) - 1].clone()
            }
            &ChessBoxKind::A6
            | &ChessBoxKind::B6
            | &ChessBoxKind::C6
            | &ChessBoxKind::D6
            | &ChessBoxKind::E6
            | &ChessBoxKind::F6
            | &ChessBoxKind::G6
            | &ChessBoxKind::H6 => {
                self.tray.5[ChessBoxKind::get_column_code(&box_kind) - 1].clone()
            }
            &ChessBoxKind::A7
            | &ChessBoxKind::B7
            | &ChessBoxKind::C7
            | &ChessBoxKind::D7
            | &ChessBoxKind::E7
            | &ChessBoxKind::F7
            | &ChessBoxKind::G7
            | &ChessBoxKind::H7 => {
                self.tray.6[ChessBoxKind::get_column_code(&box_kind) - 1].clone()
            }
            &ChessBoxKind::A8
            | &ChessBoxKind::B8
            | &ChessBoxKind::C8
            | &ChessBoxKind::D8
            | &ChessBoxKind::E8
            | &ChessBoxKind::F8
            | &ChessBoxKind::G8
            | &ChessBoxKind::H8 => {
                self.tray.7[ChessBoxKind::get_column_code(&box_kind) - 1].clone()
            }
            &ChessBoxKind::Unknown => {
                ChessBox::new(ChessBoxKind::Unknown, ChessBoxColor::White, None)
            }
        }
    }

    pub fn box_str_to_box(&self, box_str: &str) -> ChessBox {
        match box_str {
            "a1" => self.get_box(&ChessBoxKind::A1),
            "b1" => self.get_box(&ChessBoxKind::B1),
            "c1" => self.get_box(&ChessBoxKind::C1),
            "d1" => self.get_box(&ChessBoxKind::D1),
            "e1" => self.get_box(&ChessBoxKind::E1),
            "f1" => self.get_box(&ChessBoxKind::F1),
            "g1" => self.get_box(&ChessBoxKind::G1),
            "h1" => self.get_box(&ChessBoxKind::H1),
            "a2" => self.get_box(&ChessBoxKind::A2),
            "b2" => self.get_box(&ChessBoxKind::B2),
            "c2" => self.get_box(&ChessBoxKind::C2),
            "d2" => self.get_box(&ChessBoxKind::D2),
            "e2" => self.get_box(&ChessBoxKind::E2),
            "f2" => self.get_box(&ChessBoxKind::F2),
            "g2" => self.get_box(&ChessBoxKind::G2),
            "h2" => self.get_box(&ChessBoxKind::H2),
            "a3" => self.get_box(&ChessBoxKind::A3),
            "b3" => self.get_box(&ChessBoxKind::B3),
            "c3" => self.get_box(&ChessBoxKind::C3),
            "d3" => self.get_box(&ChessBoxKind::D3),
            "e3" => self.get_box(&ChessBoxKind::E3),
            "f3" => self.get_box(&ChessBoxKind::F3),
            "g3" => self.get_box(&ChessBoxKind::G3),
            "h3" => self.get_box(&ChessBoxKind::H3),
            "a4" => self.get_box(&ChessBoxKind::A4),
            "b4" => self.get_box(&ChessBoxKind::B4),
            "c4" => self.get_box(&ChessBoxKind::C4),
            "d4" => self.get_box(&ChessBoxKind::D4),
            "e4" => self.get_box(&ChessBoxKind::E4),
            "f4" => self.get_box(&ChessBoxKind::F4),
            "g4" => self.get_box(&ChessBoxKind::G4),
            "h4" => self.get_box(&ChessBoxKind::H4),
            "a5" => self.get_box(&ChessBoxKind::A5),
            "b5" => self.get_box(&ChessBoxKind::B5),
            "c5" => self.get_box(&ChessBoxKind::C5),
            "d5" => self.get_box(&ChessBoxKind::D5),
            "e5" => self.get_box(&ChessBoxKind::E5),
            "f5" => self.get_box(&ChessBoxKind::F5),
            "g5" => self.get_box(&ChessBoxKind::G5),
            "h5" => self.get_box(&ChessBoxKind::H5),
            "a6" => self.get_box(&ChessBoxKind::A6),
            "b6" => self.get_box(&ChessBoxKind::B6),
            "c6" => self.get_box(&ChessBoxKind::C6),
            "d6" => self.get_box(&ChessBoxKind::D6),
            "e6" => self.get_box(&ChessBoxKind::E6),
            "f6" => self.get_box(&ChessBoxKind::F6),
            "g6" => self.get_box(&ChessBoxKind::G6),
            "h6" => self.get_box(&ChessBoxKind::H6),
            "a7" => self.get_box(&ChessBoxKind::A7),
            "b7" => self.get_box(&ChessBoxKind::B7),
            "c7" => self.get_box(&ChessBoxKind::C7),
            "d7" => self.get_box(&ChessBoxKind::D7),
            "e7" => self.get_box(&ChessBoxKind::E7),
            "f7" => self.get_box(&ChessBoxKind::F7),
            "g7" => self.get_box(&ChessBoxKind::G7),
            "h7" => self.get_box(&ChessBoxKind::H7),
            "a8" => self.get_box(&ChessBoxKind::A8),
            "b8" => self.get_box(&ChessBoxKind::B8),
            "c8" => self.get_box(&ChessBoxKind::C8),
            "d8" => self.get_box(&ChessBoxKind::D8),
            "e8" => self.get_box(&ChessBoxKind::E8),
            "f8" => self.get_box(&ChessBoxKind::F8),
            "g8" => self.get_box(&ChessBoxKind::G8),
            "h8" => self.get_box(&ChessBoxKind::H8),
            _ => unreachable!(),
        }
    }

    pub fn get_box_str(
        &self,
        chess_piece: &Option<ChessPieceKind>,
        color_box: &ChessBoxColor,
    ) -> String {
        match chess_piece {
            Some(p) => ChessPieceKind::chess_piece_to_unicode(&p).to_string(),
            None if color_box == &ChessBoxColor::Black => format!("{}", ".".green()),
            None => ".".to_string(),
        }
    }

    pub fn render_tray_to_string(&self) -> String {
        let mut tray_str = String::new();

        tray_str.push_str("1 ");
        for (_, v) in self.tray.0.iter().enumerate() {
            tray_str.push_str(self.get_box_str(&v.piece, &v.color).as_str());
            tray_str.push_str(" ");
        }
        tray_str.push_str("\n");

        tray_str.push_str("2 ");
        for (_, v) in self.tray.1.iter().enumerate() {
            tray_str.push_str(self.get_box_str(&v.piece, &v.color).as_str());
            tray_str.push_str(" ");
        }
        tray_str.push_str("\n");

        tray_str.push_str("3 ");
        for (_, v) in self.tray.2.iter().enumerate() {
            tray_str.push_str(self.get_box_str(&v.piece, &v.color).as_str());
            tray_str.push_str(" ");
        }
        tray_str.push_str("\n");

        tray_str.push_str("4 ");
        for (_, v) in self.tray.3.iter().enumerate() {
            tray_str.push_str(self.get_box_str(&v.piece, &v.color).as_str());
            tray_str.push_str(" ");
        }
        tray_str.push_str("\n");

        tray_str.push_str("5 ");
        for (_, v) in self.tray.4.iter().enumerate() {
            tray_str.push_str(self.get_box_str(&v.piece, &v.color).as_str());
            tray_str.push_str(" ");
        }
        tray_str.push_str("\n");

        tray_str.push_str("6 ");
        for (_, v) in self.tray.5.iter().enumerate() {
            tray_str.push_str(self.get_box_str(&v.piece, &v.color).as_str());
            tray_str.push_str(" ");
        }
        tray_str.push_str("\n");

        tray_str.push_str("7 ");
        for (_, v) in self.tray.6.iter().enumerate() {
            tray_str.push_str(self.get_box_str(&v.piece, &v.color).as_str());
            tray_str.push_str(" ");
        }
        tray_str.push_str("\n");

        tray_str.push_str("8 ");
        for (_, v) in self.tray.7.iter().enumerate() {
            tray_str.push_str(self.get_box_str(&v.piece, &v.color).as_str());
            tray_str.push_str(" ");
        }
        tray_str.push_str("\n");

        tray_str.push_str("  a");
        tray_str.push_str(" b");
        tray_str.push_str(" c");
        tray_str.push_str(" d");
        tray_str.push_str(" e");
        tray_str.push_str(" f");
        tray_str.push_str(" g");
        tray_str.push_str(" h");

        tray_str
    }

    pub fn print_tray(&self, color: ChessPieceColor) {
        if ChessPieceColor::Black == color {
            println!("{}", self.render_tray_to_string()); // TODO: reverse the tray when player play with black piece
        } else {
            println!("{}", self.render_tray_to_string());
        }
    }

    pub fn get_active_piece(&mut self) {
        for (_, v) in self.tray.0.iter().enumerate() {
            match v.piece {
                Some(p) => {
                    if ChessPieceKind::is_black_piece(&p) {
                        self.active_black_piece.push(v.clone())
                    } else {
                        self.active_white_piece.push(v.clone())
                    }
                }
                None => (),
            }
        }

        for (_, v) in self.tray.1.iter().enumerate() {
            match v.piece {
                Some(p) => {
                    if ChessPieceKind::is_black_piece(&p) {
                        self.active_black_piece.push(v.clone())
                    } else {
                        self.active_white_piece.push(v.clone())
                    }
                }
                None => (),
            }
        }

        for (_, v) in self.tray.2.iter().enumerate() {
            match v.piece {
                Some(p) => {
                    if ChessPieceKind::is_black_piece(&p) {
                        self.active_black_piece.push(v.clone())
                    } else {
                        self.active_white_piece.push(v.clone())
                    }
                }
                None => (),
            }
        }

        for (_, v) in self.tray.3.iter().enumerate() {
            match v.piece {
                Some(p) => {
                    if ChessPieceKind::is_black_piece(&p) {
                        self.active_black_piece.push(v.clone())
                    } else {
                        self.active_white_piece.push(v.clone())
                    }
                }
                None => (),
            }
        }

        for (_, v) in self.tray.4.iter().enumerate() {
            match v.piece {
                Some(p) => {
                    if ChessPieceKind::is_black_piece(&p) {
                        self.active_black_piece.push(v.clone())
                    } else {
                        self.active_white_piece.push(v.clone())
                    }
                }
                None => (),
            }
        }

        for (_, v) in self.tray.5.iter().enumerate() {
            match v.piece {
                Some(p) => {
                    if ChessPieceKind::is_black_piece(&p) {
                        self.active_black_piece.push(v.clone())
                    } else {
                        self.active_white_piece.push(v.clone())
                    }
                }
                None => (),
            }
        }

        for (_, v) in self.tray.6.iter().enumerate() {
            match v.piece {
                Some(p) => {
                    if ChessPieceKind::is_black_piece(&p) {
                        self.active_black_piece.push(v.clone())
                    } else {
                        self.active_white_piece.push(v.clone())
                    }
                }
                None => (),
            }
        }

        for (_, v) in self.tray.7.iter().enumerate() {
            match v.piece {
                Some(p) => {
                    if ChessPieceKind::is_black_piece(&p) {
                        self.active_black_piece.push(v.clone())
                    } else {
                        self.active_white_piece.push(v.clone())
                    }
                }
                None => (),
            }
        }
    }

    pub fn add_eat_black_piece(&mut self, piece: ChessPieceKind) {
        self.eat_black_piece.push(piece);
    }

    pub fn add_eat_white_piece(&mut self, piece: ChessPieceKind) {
        self.eat_white_piece.push(piece);
    }
}

#[cfg(test)]
mod test {
    use crate::tray::ChessBoxColor;
    use crate::tray::ChessBoxKind;
    use crate::tray::Tray;

    #[test]
    fn test_box_kind() {
        let mut tray = Tray::new();
        tray.build_new_tray();

        assert_eq!(tray.tray.0[0].kind, ChessBoxKind::A1);
        assert_eq!(tray.tray.0[1].kind, ChessBoxKind::B1);
        assert_eq!(tray.tray.0[2].kind, ChessBoxKind::C1);
        assert_eq!(tray.tray.0[3].kind, ChessBoxKind::D1);
        assert_eq!(tray.tray.0[4].kind, ChessBoxKind::E1);
        assert_eq!(tray.tray.0[5].kind, ChessBoxKind::F1);
        assert_eq!(tray.tray.0[6].kind, ChessBoxKind::G1);
        assert_eq!(tray.tray.0[7].kind, ChessBoxKind::H1);
        assert_eq!(tray.tray.1[0].kind, ChessBoxKind::A2);
        assert_eq!(tray.tray.1[1].kind, ChessBoxKind::B2);
        assert_eq!(tray.tray.1[2].kind, ChessBoxKind::C2);
        assert_eq!(tray.tray.1[3].kind, ChessBoxKind::D2);
        assert_eq!(tray.tray.1[4].kind, ChessBoxKind::E2);
        assert_eq!(tray.tray.1[5].kind, ChessBoxKind::F2);
        assert_eq!(tray.tray.1[6].kind, ChessBoxKind::G2);
        assert_eq!(tray.tray.1[7].kind, ChessBoxKind::H2);
        assert_eq!(tray.tray.2[0].kind, ChessBoxKind::A3);
        assert_eq!(tray.tray.2[1].kind, ChessBoxKind::B3);
        assert_eq!(tray.tray.2[2].kind, ChessBoxKind::C3);
        assert_eq!(tray.tray.2[3].kind, ChessBoxKind::D3);
        assert_eq!(tray.tray.2[4].kind, ChessBoxKind::E3);
        assert_eq!(tray.tray.2[5].kind, ChessBoxKind::F3);
        assert_eq!(tray.tray.2[6].kind, ChessBoxKind::G3);
        assert_eq!(tray.tray.2[7].kind, ChessBoxKind::H3);
        assert_eq!(tray.tray.3[0].kind, ChessBoxKind::A4);
        assert_eq!(tray.tray.3[1].kind, ChessBoxKind::B4);
        assert_eq!(tray.tray.3[2].kind, ChessBoxKind::C4);
        assert_eq!(tray.tray.3[3].kind, ChessBoxKind::D4);
        assert_eq!(tray.tray.3[4].kind, ChessBoxKind::E4);
        assert_eq!(tray.tray.3[5].kind, ChessBoxKind::F4);
        assert_eq!(tray.tray.3[6].kind, ChessBoxKind::G4);
        assert_eq!(tray.tray.3[7].kind, ChessBoxKind::H4);
        assert_eq!(tray.tray.4[0].kind, ChessBoxKind::A5);
        assert_eq!(tray.tray.4[1].kind, ChessBoxKind::B5);
        assert_eq!(tray.tray.4[2].kind, ChessBoxKind::C5);
        assert_eq!(tray.tray.4[3].kind, ChessBoxKind::D5);
        assert_eq!(tray.tray.4[4].kind, ChessBoxKind::E5);
        assert_eq!(tray.tray.4[5].kind, ChessBoxKind::F5);
        assert_eq!(tray.tray.4[6].kind, ChessBoxKind::G5);
        assert_eq!(tray.tray.4[7].kind, ChessBoxKind::H5);
        assert_eq!(tray.tray.5[0].kind, ChessBoxKind::A6);
        assert_eq!(tray.tray.5[1].kind, ChessBoxKind::B6);
        assert_eq!(tray.tray.5[2].kind, ChessBoxKind::C6);
        assert_eq!(tray.tray.5[3].kind, ChessBoxKind::D6);
        assert_eq!(tray.tray.5[4].kind, ChessBoxKind::E6);
        assert_eq!(tray.tray.5[5].kind, ChessBoxKind::F6);
        assert_eq!(tray.tray.5[6].kind, ChessBoxKind::G6);
        assert_eq!(tray.tray.5[7].kind, ChessBoxKind::H6);
        assert_eq!(tray.tray.6[0].kind, ChessBoxKind::A7);
        assert_eq!(tray.tray.6[1].kind, ChessBoxKind::B7);
        assert_eq!(tray.tray.6[2].kind, ChessBoxKind::C7);
        assert_eq!(tray.tray.6[3].kind, ChessBoxKind::D7);
        assert_eq!(tray.tray.6[4].kind, ChessBoxKind::E7);
        assert_eq!(tray.tray.6[5].kind, ChessBoxKind::F7);
        assert_eq!(tray.tray.6[6].kind, ChessBoxKind::G7);
        assert_eq!(tray.tray.6[7].kind, ChessBoxKind::H7);
        assert_eq!(tray.tray.7[0].kind, ChessBoxKind::A8);
        assert_eq!(tray.tray.7[1].kind, ChessBoxKind::B8);
        assert_eq!(tray.tray.7[2].kind, ChessBoxKind::C8);
        assert_eq!(tray.tray.7[3].kind, ChessBoxKind::D8);
        assert_eq!(tray.tray.7[4].kind, ChessBoxKind::E8);
        assert_eq!(tray.tray.7[5].kind, ChessBoxKind::F8);
        assert_eq!(tray.tray.7[6].kind, ChessBoxKind::G8);
        assert_eq!(tray.tray.7[7].kind, ChessBoxKind::H8);
    }

    #[test]
    fn test_box_color() {
        let mut tray = Tray::new();
        tray.build_new_tray();

        assert_eq!(tray.tray.0[0].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.0[1].color, ChessBoxColor::White);
        assert_eq!(tray.tray.0[2].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.0[3].color, ChessBoxColor::White);
        assert_eq!(tray.tray.0[4].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.0[5].color, ChessBoxColor::White);
        assert_eq!(tray.tray.0[6].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.0[7].color, ChessBoxColor::White);
        assert_eq!(tray.tray.1[0].color, ChessBoxColor::White);
        assert_eq!(tray.tray.1[1].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.1[2].color, ChessBoxColor::White);
        assert_eq!(tray.tray.1[3].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.1[4].color, ChessBoxColor::White);
        assert_eq!(tray.tray.1[5].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.1[6].color, ChessBoxColor::White);
        assert_eq!(tray.tray.1[7].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.2[0].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.2[1].color, ChessBoxColor::White);
        assert_eq!(tray.tray.2[2].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.2[3].color, ChessBoxColor::White);
        assert_eq!(tray.tray.2[4].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.2[5].color, ChessBoxColor::White);
        assert_eq!(tray.tray.2[6].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.2[7].color, ChessBoxColor::White);
        assert_eq!(tray.tray.3[0].color, ChessBoxColor::White);
        assert_eq!(tray.tray.3[1].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.3[2].color, ChessBoxColor::White);
        assert_eq!(tray.tray.3[3].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.3[4].color, ChessBoxColor::White);
        assert_eq!(tray.tray.3[5].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.3[6].color, ChessBoxColor::White);
        assert_eq!(tray.tray.3[7].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.4[0].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.4[1].color, ChessBoxColor::White);
        assert_eq!(tray.tray.4[2].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.4[3].color, ChessBoxColor::White);
        assert_eq!(tray.tray.4[4].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.4[5].color, ChessBoxColor::White);
        assert_eq!(tray.tray.4[6].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.4[7].color, ChessBoxColor::White);
        assert_eq!(tray.tray.5[0].color, ChessBoxColor::White);
        assert_eq!(tray.tray.5[1].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.5[2].color, ChessBoxColor::White);
        assert_eq!(tray.tray.5[3].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.5[4].color, ChessBoxColor::White);
        assert_eq!(tray.tray.5[5].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.5[6].color, ChessBoxColor::White);
        assert_eq!(tray.tray.5[7].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.6[0].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.6[1].color, ChessBoxColor::White);
        assert_eq!(tray.tray.6[2].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.6[3].color, ChessBoxColor::White);
        assert_eq!(tray.tray.6[4].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.6[5].color, ChessBoxColor::White);
        assert_eq!(tray.tray.6[6].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.6[7].color, ChessBoxColor::White);
        assert_eq!(tray.tray.7[0].color, ChessBoxColor::White);
        assert_eq!(tray.tray.7[1].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.7[2].color, ChessBoxColor::White);
        assert_eq!(tray.tray.7[3].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.7[4].color, ChessBoxColor::White);
        assert_eq!(tray.tray.7[5].color, ChessBoxColor::Black);
        assert_eq!(tray.tray.7[6].color, ChessBoxColor::White);
        assert_eq!(tray.tray.7[7].color, ChessBoxColor::Black);
    }
}
