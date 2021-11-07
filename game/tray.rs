use crate::piece::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ChessBoxColor {
    Black,
    White,
}

#[derive(Debug, Clone)]
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
            Self::A1
            | Self::A2
            | Self::A3
            | Self::A4
            | Self::A5
            | Self::A6
            | Self::A7
            | Self::A8 => 0,
            Self::B1
            | Self::B2
            | Self::B3
            | Self::B4
            | Self::B5
            | Self::B6
            | Self::B7
            | Self::B8 => 1,
            Self::C1
            | Self::C2
            | Self::C3
            | Self::C4
            | Self::C5
            | Self::C6
            | Self::C7
            | Self::C8 => 2,
            Self::D1
            | Self::D2
            | Self::D3
            | Self::D4
            | Self::D5
            | Self::D6
            | Self::D7
            | Self::D8 => 3,
            Self::E1
            | Self::E2
            | Self::E3
            | Self::E4
            | Self::E5
            | Self::E6
            | Self::E7
            | Self::E8 => 4,
            Self::F1
            | Self::F2
            | Self::F3
            | Self::F4
            | Self::F5
            | Self::F6
            | Self::F7
            | Self::F8 => 5,
            Self::G1
            | Self::G2
            | Self::G3
            | Self::G4
            | Self::G5
            | Self::G6
            | Self::G7
            | Self::G8 => 6,
            Self::H1
            | Self::H2
            | Self::H3
            | Self::H4
            | Self::H5
            | Self::H6
            | Self::H7
            | Self::H8 => 7,
            Self::Unknown => 404,
        }
    }

    pub fn get_line_code(&self) -> usize {
        match &self {
            Self::A1 => 0,
            Self::B1 => 1,
            Self::C1 => 2,
            Self::D1 => 3,
            Self::E1 => 4,
            Self::F1 => 5,
            Self::G1 => 6,
            Self::H1 => 7,
            Self::A2 => 0,
            Self::B2 => 1,
            Self::C2 => 2,
            Self::D2 => 3,
            Self::E2 => 4,
            Self::F2 => 5,
            Self::G2 => 6,
            Self::H2 => 7,
            Self::A3 => 0,
            Self::B3 => 1,
            Self::C3 => 2,
            Self::D3 => 3,
            Self::E3 => 4,
            Self::F3 => 5,
            Self::G3 => 6,
            Self::H3 => 7,
            Self::A4 => 0,
            Self::B4 => 1,
            Self::C4 => 2,
            Self::D4 => 3,
            Self::E4 => 4,
            Self::F4 => 5,
            Self::G4 => 6,
            Self::H4 => 7,
            Self::A5 => 0,
            Self::B5 => 1,
            Self::C5 => 2,
            Self::D5 => 3,
            Self::E5 => 4,
            Self::F5 => 5,
            Self::G5 => 6,
            Self::H5 => 7,
            Self::A6 => 0,
            Self::B6 => 1,
            Self::C6 => 2,
            Self::D6 => 3,
            Self::E6 => 4,
            Self::F6 => 5,
            Self::G6 => 6,
            Self::H6 => 7,
            Self::A7 => 0,
            Self::B7 => 1,
            Self::C7 => 2,
            Self::D7 => 3,
            Self::E7 => 4,
            Self::F7 => 5,
            Self::G7 => 6,
            Self::H7 => 7,
            Self::A8 => 0,
            Self::B8 => 1,
            Self::C8 => 2,
            Self::D8 => 3,
            Self::E8 => 4,
            Self::F8 => 5,
            Self::G8 => 6,
            Self::H8 => 7,
            Self::Unknown => 404,
        }
    }

    // (line, column)
    pub fn location_to_box_kind(loc: (usize, usize)) -> ChessBoxKind {
        match loc {
            (0, 0) => ChessBoxKind::A1,
            (0, 1) => ChessBoxKind::B1,
            (0, 2) => ChessBoxKind::C1,
            (0, 3) => ChessBoxKind::D1,
            (0, 4) => ChessBoxKind::E1,
            (0, 5) => ChessBoxKind::F1,
            (0, 6) => ChessBoxKind::G1,
            (0, 7) => ChessBoxKind::H1,
            (1, 0) => ChessBoxKind::A2,
            (1, 1) => ChessBoxKind::B2,
            (1, 2) => ChessBoxKind::C2,
            (1, 3) => ChessBoxKind::D2,
            (1, 4) => ChessBoxKind::E2,
            (1, 5) => ChessBoxKind::F2,
            (1, 6) => ChessBoxKind::G2,
            (1, 7) => ChessBoxKind::H2,
            (2, 0) => ChessBoxKind::A3,
            (2, 1) => ChessBoxKind::B3,
            (2, 2) => ChessBoxKind::C3,
            (2, 3) => ChessBoxKind::D3,
            (2, 4) => ChessBoxKind::E3,
            (2, 5) => ChessBoxKind::F3,
            (2, 6) => ChessBoxKind::G3,
            (2, 7) => ChessBoxKind::H3,
            (3, 0) => ChessBoxKind::A4,
            (3, 1) => ChessBoxKind::B4,
            (3, 2) => ChessBoxKind::C4,
            (3, 3) => ChessBoxKind::D4,
            (3, 4) => ChessBoxKind::E4,
            (3, 5) => ChessBoxKind::F4,
            (3, 6) => ChessBoxKind::G4,
            (3, 7) => ChessBoxKind::H4,
            (4, 0) => ChessBoxKind::A5,
            (4, 1) => ChessBoxKind::B5,
            (4, 2) => ChessBoxKind::C5,
            (4, 3) => ChessBoxKind::D5,
            (4, 4) => ChessBoxKind::E5,
            (4, 5) => ChessBoxKind::F5,
            (4, 6) => ChessBoxKind::G5,
            (4, 7) => ChessBoxKind::H5,
            (5, 0) => ChessBoxKind::A6,
            (5, 1) => ChessBoxKind::B6,
            (5, 2) => ChessBoxKind::C6,
            (5, 3) => ChessBoxKind::D6,
            (5, 4) => ChessBoxKind::E6,
            (5, 5) => ChessBoxKind::F6,
            (5, 6) => ChessBoxKind::G6,
            (5, 7) => ChessBoxKind::H6,
            (6, 0) => ChessBoxKind::A7,
            (6, 1) => ChessBoxKind::B7,
            (6, 2) => ChessBoxKind::C7,
            (6, 3) => ChessBoxKind::D7,
            (6, 4) => ChessBoxKind::E7,
            (6, 5) => ChessBoxKind::F7,
            (6, 6) => ChessBoxKind::G7,
            (6, 7) => ChessBoxKind::H7,
            (7, 0) => ChessBoxKind::A8,
            (7, 1) => ChessBoxKind::B8,
            (7, 2) => ChessBoxKind::C8,
            (7, 3) => ChessBoxKind::D8,
            (7, 4) => ChessBoxKind::E8,
            (7, 5) => ChessBoxKind::F8,
            (7, 6) => ChessBoxKind::G8,
            (7, 7) => ChessBoxKind::H8,
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
            | &ChessBoxKind::H1 => self.tray.0[ChessBoxKind::get_line_code(&box_kind)].clone(),
            &ChessBoxKind::A2
            | &ChessBoxKind::B2
            | &ChessBoxKind::C2
            | &ChessBoxKind::D2
            | &ChessBoxKind::E2
            | &ChessBoxKind::F2
            | &ChessBoxKind::G2
            | &ChessBoxKind::H2 => self.tray.1[ChessBoxKind::get_line_code(&box_kind)].clone(),
            &ChessBoxKind::A3
            | &ChessBoxKind::B3
            | &ChessBoxKind::C3
            | &ChessBoxKind::D3
            | &ChessBoxKind::E3
            | &ChessBoxKind::F3
            | &ChessBoxKind::G3
            | &ChessBoxKind::H3 => self.tray.2[ChessBoxKind::get_line_code(&box_kind)].clone(),
            &ChessBoxKind::A4
            | &ChessBoxKind::B4
            | &ChessBoxKind::C4
            | &ChessBoxKind::D4
            | &ChessBoxKind::E4
            | &ChessBoxKind::F4
            | &ChessBoxKind::G4
            | &ChessBoxKind::H4 => self.tray.3[ChessBoxKind::get_line_code(&box_kind)].clone(),
            &ChessBoxKind::A5
            | &ChessBoxKind::B5
            | &ChessBoxKind::C5
            | &ChessBoxKind::D5
            | &ChessBoxKind::E5
            | &ChessBoxKind::F5
            | &ChessBoxKind::G5
            | &ChessBoxKind::H5 => self.tray.4[ChessBoxKind::get_line_code(&box_kind)].clone(),
            &ChessBoxKind::A6
            | &ChessBoxKind::B6
            | &ChessBoxKind::C6
            | &ChessBoxKind::D6
            | &ChessBoxKind::E6
            | &ChessBoxKind::F6
            | &ChessBoxKind::G6
            | &ChessBoxKind::H6 => self.tray.5[ChessBoxKind::get_line_code(&box_kind)].clone(),
            &ChessBoxKind::A7
            | &ChessBoxKind::B7
            | &ChessBoxKind::C7
            | &ChessBoxKind::D7
            | &ChessBoxKind::E7
            | &ChessBoxKind::F7
            | &ChessBoxKind::G7
            | &ChessBoxKind::H7 => self.tray.6[ChessBoxKind::get_line_code(&box_kind)].clone(),
            &ChessBoxKind::A8
            | &ChessBoxKind::B8
            | &ChessBoxKind::C8
            | &ChessBoxKind::D8
            | &ChessBoxKind::E8
            | &ChessBoxKind::F8
            | &ChessBoxKind::G8
            | &ChessBoxKind::H8 => self.tray.7[ChessBoxKind::get_line_code(&box_kind)].clone(),
            &ChessBoxKind::Unknown => {
                ChessBox::new(ChessBoxKind::Unknown, ChessBoxColor::White, None)
            }
        }
    }

    pub fn get_box_str(
        &self,
        chess_piece: &Option<ChessPieceKind>,
        color_box: &ChessBoxColor,
    ) -> String {
        match chess_piece {
            Some(p) => ChessPieceKind::chess_piece_to_unicode(&p).to_string(),
            None if color_box == &ChessBoxColor::Black => "\x1b[32m.\x1b[0m".to_string(),
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
}