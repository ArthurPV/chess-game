use crate::piece::ChessPieceColor;
use crate::piece::ChessPieceKind;
use crate::tray::ChessBox;
use crate::tray::ChessBoxKind;
use crate::tray::Tray;
use crate::turn::ChessTurnAction;
use crate::ChessBishopKind;

pub trait ChessMove {
    fn chess_possible_pawn_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction>;
    fn chess_possible_bishop_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
        bishop_kind: &ChessBishopKind,
    ) -> Vec<ChessTurnAction>;
    fn chess_possible_knight_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction>;
    fn chess_possible_rook_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction>;
    fn chess_possible_queen_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction>;
    fn chess_possible_king_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction>;
    fn chess_modify_box(&mut self, box_kind: &ChessBoxKind, piece: Option<ChessPieceKind>);
    fn chess_possible_move(
        &self,
        piece_kind: ChessPieceKind,
        box_kind: ChessBoxKind,
    ) -> Vec<ChessTurnAction>;
    fn get_all_possible_move(&self, color_kind: &ChessPieceColor) -> Vec<ChessTurnAction>;
}

impl ChessMove for Tray {
    fn chess_possible_pawn_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction> {
        let mut p_move = vec![];

        let line = ChessBoxKind::get_line_code(box_kind);
        let column = ChessBoxKind::get_column_code(box_kind);

        // advance one case
        if color_kind == &ChessPieceColor::White {
            let box_above = self.get_box(&ChessBoxKind::location_to_box_kind((line + 1, column)));

            match box_above.piece {
                Some(_) => (),
                None => p_move.push(ChessTurnAction::new_move(
                    ChessPieceKind::Pawn(*color_kind),
                    *box_kind,
                    box_above,
                )),
            }
        } else {
            let box_below = self.get_box(&ChessBoxKind::location_to_box_kind((line - 1, column)));

            match box_below.piece {
                Some(_) => (),
                None => p_move.push(ChessTurnAction::new_move(
                    ChessPieceKind::Pawn(*color_kind),
                    *box_kind,
                    box_below,
                )),
            }
        }

        // advance two cases
        if color_kind == &ChessPieceColor::White {
            let box_above = self.get_box(&ChessBoxKind::location_to_box_kind((line + 2, column)));

            match box_above.piece {
                Some(_) => (),
                None => match ChessBoxKind::location_to_box_kind((line, column)) {
                    ChessBoxKind::A2
                    | ChessBoxKind::B2
                    | ChessBoxKind::C2
                    | ChessBoxKind::D2
                    | ChessBoxKind::E2
                    | ChessBoxKind::F2
                    | ChessBoxKind::G2
                    | ChessBoxKind::H2 => p_move.push(ChessTurnAction::new_move(
                        ChessPieceKind::Pawn(*color_kind),
                        *box_kind,
                        box_above,
                    )),
                    _ => (),
                },
            }
        } else {
            let box_below = self.get_box(&ChessBoxKind::location_to_box_kind((line - 2, column)));

            match box_below.piece {
                Some(_) => (),
                None => match ChessBoxKind::location_to_box_kind((line, column)) {
                    ChessBoxKind::A7
                    | ChessBoxKind::B7
                    | ChessBoxKind::C7
                    | ChessBoxKind::D7
                    | ChessBoxKind::E7
                    | ChessBoxKind::F7
                    | ChessBoxKind::G7
                    | ChessBoxKind::H7 => p_move.push(ChessTurnAction::new_move(
                        ChessPieceKind::Pawn(*color_kind),
                        *box_kind,
                        box_below,
                    )),
                    _ => (),
                },
            }
        }

        // eat piece
        {
            // TODO: review this feature later
            if column == 0 {
                let eat = self.get_box(&ChessBoxKind::location_to_box_kind((line, column + 1)));

                match eat.piece {
                    Some(v) if ChessPieceKind::is_edible(&v, color_kind) => match v {
                        ChessPieceKind::King(_) => p_move.push(ChessTurnAction::new_check(
                            ChessPieceKind::Pawn(*color_kind),
                            *box_kind,
                            eat,
                        )),
                        _ => p_move.push(ChessTurnAction::new_eat(
                            ChessPieceKind::Pawn(*color_kind),
                            *box_kind,
                            eat,
                        )),
                    },
                    Some(_) => (),
                    None => (),
                }
            } else if column == 7 {
                let eat = self.get_box(&ChessBoxKind::location_to_box_kind((line, column - 1)));

                match eat.piece {
                    Some(v) if ChessPieceKind::is_edible(&v, color_kind) => match v {
                        ChessPieceKind::King(_) => p_move.push(ChessTurnAction::new_check(
                            ChessPieceKind::Pawn(*color_kind),
                            *box_kind,
                            eat,
                        )),
                        _ => p_move.push(ChessTurnAction::new_eat(
                            ChessPieceKind::Pawn(*color_kind),
                            *box_kind,
                            eat,
                        )),
                    },
                    Some(_) => (),
                    None => (),
                }
            } else {
                let eat = vec![
                    self.get_box(&ChessBoxKind::location_to_box_kind((line, column + 1))),
                    self.get_box(&ChessBoxKind::location_to_box_kind((line, column - 1))),
                ];

                for (i, v) in eat.iter().enumerate() {
                    match v.piece {
                        Some(v) if ChessPieceKind::is_edible(&v, color_kind) => match v {
                            ChessPieceKind::King(_) => p_move.push(ChessTurnAction::new_check(
                                ChessPieceKind::Pawn(*color_kind),
                                *box_kind,
                                eat[i].clone(),
                            )),
                            _ => p_move.push(ChessTurnAction::new_eat(
                                ChessPieceKind::Pawn(*color_kind),
                                *box_kind,
                                eat[i].clone(),
                            )),
                        },
                        Some(_) => (),
                        None => (),
                    }
                }
            }
        }

        p_move
    }

    fn chess_possible_bishop_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
        bishop_kind: &ChessBishopKind,
    ) -> Vec<ChessTurnAction> {
        let mut b_move = vec![];

        let line = ChessBoxKind::get_line_code(box_kind);
        let column = ChessBoxKind::get_column_code(box_kind);

        fn bishop_move(
            tray: &Tray,
            box_kind: &ChessBoxKind,
            b_move: &mut Vec<ChessTurnAction>,
            color_kind: &ChessPieceColor,
            bishop_kind: &ChessBishopKind,
            line: usize,
            column: usize,
        ) -> bool {
            let m = tray.get_box(&ChessBoxKind::location_to_box_kind((line, column)));

            match m.piece {
                Some(v) if ChessPieceKind::is_edible(&v, color_kind) => match v {
                    ChessPieceKind::King(_) => {
                        b_move.push(ChessTurnAction::new_check(
                            ChessPieceKind::Bishop(*color_kind, *bishop_kind),
                            *box_kind,
                            m,
                        ));
                        true
                    }
                    _ => {
                        b_move.push(ChessTurnAction::new_eat(
                            ChessPieceKind::Bishop(*color_kind, *bishop_kind),
                            *box_kind,
                            m,
                        ));
                        true
                    }
                },
                Some(_) => true,
                None => {
                    b_move.push(ChessTurnAction::new_move(
                        ChessPieceKind::Bishop(*color_kind, *bishop_kind),
                        *box_kind,
                        m,
                    ));
                    false
                }
            }
        }

        // GET MOVE
        for i in 1..7 {
            if line + i <= 8 && column + i <= 8 {
                if bishop_move(
                    &self,
                    box_kind,
                    &mut b_move,
                    color_kind,
                    bishop_kind,
                    line + i,
                    column + i,
                ) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if line + i <= 8 && (column - i) as isize >= 1 {
                if bishop_move(
                    &self,
                    box_kind,
                    &mut b_move,
                    color_kind,
                    bishop_kind,
                    line + i,
                    column - i,
                ) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if (line - i) as isize >= 1 && column + i <= 8 {
                if bishop_move(
                    &self,
                    box_kind,
                    &mut b_move,
                    color_kind,
                    bishop_kind,
                    line - i,
                    column + i,
                ) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if (line - i) as isize >= 1 && (column - i) as isize >= 1 {
                if bishop_move(
                    &self,
                    box_kind,
                    &mut b_move,
                    color_kind,
                    bishop_kind,
                    line - i,
                    column - i,
                ) {
                    break;
                }
            } else {
                break;
            }
        }

        b_move
    }

    fn chess_possible_knight_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction> {
        let mut n_move = vec![];

        let line = ChessBoxKind::get_line_code(box_kind);
        let column = ChessBoxKind::get_column_code(box_kind);

        fn knight_move(
            tray: &Tray,
            box_kind: &ChessBoxKind,
            n_move: &mut Vec<ChessTurnAction>,
            color_kind: &ChessPieceColor,
            line: usize,
            column: usize,
        ) {
            let m = tray.get_box(&ChessBoxKind::location_to_box_kind((line, column)));

            match m.piece {
                Some(v) if ChessPieceKind::is_edible(&v, color_kind) => match v {
                    ChessPieceKind::King(_) => n_move.push(ChessTurnAction::new_check(
                        ChessPieceKind::Knight(*color_kind),
                        *box_kind,
                        m,
                    )),
                    _ => n_move.push(ChessTurnAction::new_eat(
                        ChessPieceKind::Knight(*color_kind),
                        *box_kind,
                        m,
                    )),
                },
                Some(_) => (),
                None => n_move.push(ChessTurnAction::new_move(
                    ChessPieceKind::Knight(*color_kind),
                    *box_kind,
                    m,
                )),
            }
        }

        if line + 2 <= 8 && column + 1 <= 8 {
            knight_move(
                &self,
                box_kind,
                &mut n_move,
                color_kind,
                line + 2,
                column + 1,
            )
        }

        if line + 2 <= 8 && (column) as isize - 1 >= 1 {
            knight_move(
                &self,
                box_kind,
                &mut n_move,
                color_kind,
                line + 2,
                column - 1,
            )
        }

        if line + 1 <= 8 && column + 2 <= 8 {
            knight_move(
                &self,
                box_kind,
                &mut n_move,
                color_kind,
                line + 1,
                column + 2,
            )
        }

        if (line) as isize - 1 >= 1 && column + 2 <= 8 {
            knight_move(
                &self,
                box_kind,
                &mut n_move,
                color_kind,
                line - 1,
                column + 2,
            )
        }

        if (line) as isize - 2 >= 1 && column + 1 <= 8 {
            knight_move(
                &self,
                box_kind,
                &mut n_move,
                color_kind,
                line - 2,
                column + 1,
            )
        }

        if (line) as isize - 2 >= 1 && (column) as isize - 1 >= 1 {
            knight_move(
                &self,
                box_kind,
                &mut n_move,
                color_kind,
                line - 2,
                column - 1,
            )
        }

        if line + 1 <= 8 && (column) as isize - 2 >= 1 {
            knight_move(
                &self,
                box_kind,
                &mut n_move,
                color_kind,
                line + 1,
                column - 2,
            )
        }

        if (line) as isize - 1 >= 1 && (column) as isize - 2 >= 1 {
            knight_move(
                &self,
                box_kind,
                &mut n_move,
                color_kind,
                line - 1,
                column - 2,
            )
        }

        n_move
    }

    fn chess_possible_rook_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction> {
        let mut r_move = vec![];

        let line = ChessBoxKind::get_line_code(box_kind);
        let column = ChessBoxKind::get_column_code(box_kind);

        fn rook_move(
            tray: &Tray,
            box_kind: &ChessBoxKind,
            r_move: &mut Vec<ChessTurnAction>,
            color_kind: &ChessPieceColor,
            line: usize,
            column: usize,
        ) -> bool {
            let m = tray.get_box(&ChessBoxKind::location_to_box_kind((line, column)));

            match m.piece {
                Some(v) if ChessPieceKind::is_edible(&v, color_kind) => match v {
                    ChessPieceKind::King(_) => {
                        r_move.push(ChessTurnAction::new_check(
                            ChessPieceKind::Rook(*color_kind),
                            *box_kind,
                            m,
                        ));
                        true
                    }
                    _ => {
                        r_move.push(ChessTurnAction::new_eat(
                            ChessPieceKind::Rook(*color_kind),
                            *box_kind,
                            m,
                        ));
                        true
                    }
                },
                Some(_) => true,
                None => {
                    r_move.push(ChessTurnAction::new_move(
                        ChessPieceKind::Rook(*color_kind),
                        *box_kind,
                        m,
                    ));
                    false
                }
            }
        }

        for i in 1..7 {
            if line + i <= 8 {
                if rook_move(&self, box_kind, &mut r_move, color_kind, line + i, column) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if (line - i) as isize >= 1 {
                if rook_move(&self, box_kind, &mut r_move, color_kind, line - i, column) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if column + i <= 8 {
                if rook_move(&self, box_kind, &mut r_move, color_kind, line, column + i) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if (column - i) as isize >= 1 {
                if rook_move(&self, box_kind, &mut r_move, color_kind, line, column - i) {
                    break;
                }
            } else {
                break;
            }
        }

        r_move
    }

    fn chess_possible_queen_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction> {
        let mut q_move = vec![];

        let line = ChessBoxKind::get_line_code(box_kind);
        let column = ChessBoxKind::get_column_code(box_kind);

        fn queen_move(
            tray: &Tray,
            box_kind: &ChessBoxKind,
            q_move: &mut Vec<ChessTurnAction>,
            color_kind: &ChessPieceColor,
            line: usize,
            column: usize,
        ) -> bool {
            let m = tray.get_box(&ChessBoxKind::location_to_box_kind((line, column)));

            match m.piece {
                Some(v) if ChessPieceKind::is_edible(&v, color_kind) => match v {
                    ChessPieceKind::King(_) => {
                        q_move.push(ChessTurnAction::new_check(
                            ChessPieceKind::Queen(*color_kind),
                            *box_kind,
                            m,
                        ));
                        true
                    }
                    _ => {
                        q_move.push(ChessTurnAction::new_eat(
                            ChessPieceKind::Queen(*color_kind),
                            *box_kind,
                            m,
                        ));
                        true
                    }
                },
                Some(_) => true,
                None => {
                    q_move.push(ChessTurnAction::new_move(
                        ChessPieceKind::Queen(*color_kind),
                        *box_kind,
                        m,
                    ));
                    false
                }
            }
        }

        // LIKE ROOK MOVE

        for i in 1..7 {
            if line + i <= 8 {
                if queen_move(&self, box_kind, &mut q_move, color_kind, line + i, column) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if (line - i) as isize >= 1 {
                if queen_move(&self, box_kind, &mut q_move, color_kind, line - i, column) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if column + i <= 8 {
                if queen_move(&self, box_kind, &mut q_move, color_kind, line, column + i) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if (column - i) as isize >= 1 {
                if queen_move(&self, box_kind, &mut q_move, color_kind, line, column - i) {
                    break;
                }
            } else {
                break;
            }
        }

        // LIKE BISHOP MOVE

        for i in 1..7 {
            if line + i <= 8 && column + i <= 8 {
                if queen_move(
                    &self,
                    box_kind,
                    &mut q_move,
                    color_kind,
                    line + i,
                    column + i,
                ) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if line + i <= 8 && (column - i) as isize >= 1 {
                if queen_move(
                    &self,
                    box_kind,
                    &mut q_move,
                    color_kind,
                    line + i,
                    column - i,
                ) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if (line - i) as isize >= 1 && column + i <= 8 {
                if queen_move(
                    &self,
                    box_kind,
                    &mut q_move,
                    color_kind,
                    line - i,
                    column + i,
                ) {
                    break;
                }
            } else {
                break;
            }
        }

        for i in 1..7 {
            if (line - i) as isize >= 1 && (column - i) as isize <= 1 {
                if queen_move(
                    &self,
                    box_kind,
                    &mut q_move,
                    color_kind,
                    line - i,
                    column - i,
                ) {
                    break;
                }
            } else {
                break;
            }
        }

        q_move
    }

    fn chess_possible_king_move(
        &self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction> {
        let mut k_move = vec![];

        let line = ChessBoxKind::get_line_code(box_kind);
        let column = ChessBoxKind::get_column_code(box_kind);

        fn king_move(
            tray: &Tray,
            box_kind: &ChessBoxKind,
            k_move: &mut Vec<ChessTurnAction>,
            color_kind: &ChessPieceColor,
            line: usize,
            column: usize,
        ) {
            let m = tray.get_box(&ChessBoxKind::location_to_box_kind((line, column)));

            match m.piece {
                Some(v) if ChessPieceKind::is_edible(&v, color_kind) => match v {
                    ChessPieceKind::King(_) => {
                        k_move.push(ChessTurnAction::new_check(
                            ChessPieceKind::King(*color_kind),
                            *box_kind,
                            m,
                        ));
                    }
                    _ => {
                        k_move.push(ChessTurnAction::new_eat(
                            ChessPieceKind::King(*color_kind),
                            *box_kind,
                            m,
                        ));
                    }
                },
                Some(_) => (),
                None => {
                    k_move.push(ChessTurnAction::new_move(
                        ChessPieceKind::King(*color_kind),
                        *box_kind,
                        m,
                    ));
                }
            }
        }

        // LIKE ROOK MOVE

        if line + 1 <= 8 {
            king_move(&self, box_kind, &mut k_move, color_kind, line + 1, column)
        }

        if (line - 1) as isize >= 1 {
            king_move(&self, box_kind, &mut k_move, color_kind, line - 1, column)
        }

        if column + 1 <= 8 {
            king_move(&self, box_kind, &mut k_move, color_kind, line, column + 1)
        }

        if (column - 1) as isize >= 1 {
            king_move(&self, box_kind, &mut k_move, color_kind, line, column - 1)
        }

        // LIKE BISHOP MOVE

        if line + 1 <= 8 && column + 1 <= 8 {
            king_move(
                &self,
                box_kind,
                &mut k_move,
                color_kind,
                line + 1,
                column + 1,
            )
        }

        if line + 1 <= 8 && (column - 1) as isize >= 1 {
            king_move(
                &self,
                box_kind,
                &mut k_move,
                color_kind,
                line + 1,
                column - 1,
            )
        }

        if (line - 1) as isize >= 1 && column + 1 <= 8 {
            king_move(
                &self,
                box_kind,
                &mut k_move,
                color_kind,
                line - 1,
                column + 1,
            )
        }

        if (line - 1) as isize >= 1 && (column - 1) as isize >= 1 {
            king_move(
                &self,
                box_kind,
                &mut k_move,
                color_kind,
                line - 1,
                column - 1,
            )
        }

        k_move
    }

    fn chess_modify_box(&mut self, box_kind: &ChessBoxKind, piece: Option<ChessPieceKind>) {
        match box_kind {
            &ChessBoxKind::A1
            | &ChessBoxKind::B1
            | &ChessBoxKind::C1
            | &ChessBoxKind::D1
            | &ChessBoxKind::E1
            | &ChessBoxKind::F1
            | &ChessBoxKind::G1
            | &ChessBoxKind::H1 => {
                self.tray.0[ChessBoxKind::get_column_code(box_kind) - 1].piece = piece
            }
            &ChessBoxKind::A2
            | &ChessBoxKind::B2
            | &ChessBoxKind::C2
            | &ChessBoxKind::D2
            | &ChessBoxKind::E2
            | &ChessBoxKind::F2
            | &ChessBoxKind::G2
            | &ChessBoxKind::H2 => {
                self.tray.1[ChessBoxKind::get_column_code(box_kind) - 1].piece = piece
            }
            &ChessBoxKind::A3
            | &ChessBoxKind::B3
            | &ChessBoxKind::C3
            | &ChessBoxKind::D3
            | &ChessBoxKind::E3
            | &ChessBoxKind::F3
            | &ChessBoxKind::G3
            | &ChessBoxKind::H3 => {
                self.tray.2[ChessBoxKind::get_column_code(box_kind) - 1].piece = piece
            }
            &ChessBoxKind::A4
            | &ChessBoxKind::B4
            | &ChessBoxKind::C4
            | &ChessBoxKind::D4
            | &ChessBoxKind::E4
            | &ChessBoxKind::F4
            | &ChessBoxKind::G4
            | &ChessBoxKind::H4 => {
                self.tray.3[ChessBoxKind::get_column_code(box_kind) - 1].piece = piece
            }
            &ChessBoxKind::A5
            | &ChessBoxKind::B5
            | &ChessBoxKind::C5
            | &ChessBoxKind::D5
            | &ChessBoxKind::E5
            | &ChessBoxKind::F5
            | &ChessBoxKind::G5
            | &ChessBoxKind::H5 => {
                self.tray.4[ChessBoxKind::get_column_code(box_kind) - 1].piece = piece
            }
            &ChessBoxKind::A6
            | &ChessBoxKind::B6
            | &ChessBoxKind::C6
            | &ChessBoxKind::D6
            | &ChessBoxKind::E6
            | &ChessBoxKind::F6
            | &ChessBoxKind::G6
            | &ChessBoxKind::H6 => {
                self.tray.5[ChessBoxKind::get_column_code(box_kind) - 1].piece = piece
            }
            &ChessBoxKind::A7
            | &ChessBoxKind::B7
            | &ChessBoxKind::C7
            | &ChessBoxKind::D7
            | &ChessBoxKind::E7
            | &ChessBoxKind::F7
            | &ChessBoxKind::G7
            | &ChessBoxKind::H7 => {
                self.tray.6[ChessBoxKind::get_column_code(box_kind) - 1].piece = piece
            }
            &ChessBoxKind::A8
            | &ChessBoxKind::B8
            | &ChessBoxKind::C8
            | &ChessBoxKind::D8
            | &ChessBoxKind::E8
            | &ChessBoxKind::F8
            | &ChessBoxKind::G8
            | &ChessBoxKind::H8 => {
                self.tray.7[ChessBoxKind::get_column_code(box_kind) - 1].piece = piece
            }
            &ChessBoxKind::Unknown => (),
        }
    }

    fn chess_possible_move(
        &self,
        piece_kind: ChessPieceKind,
        box_kind: ChessBoxKind,
    ) -> Vec<ChessTurnAction> {
        match piece_kind {
            ChessPieceKind::Pawn(c) => self.chess_possible_pawn_move(&box_kind, &c),
            ChessPieceKind::Bishop(c, b) => self.chess_possible_bishop_move(&box_kind, &c, &b),
            ChessPieceKind::Knight(c) => self.chess_possible_knight_move(&box_kind, &c),
            ChessPieceKind::Rook(c) => self.chess_possible_rook_move(&box_kind, &c),
            ChessPieceKind::Queen(c) => self.chess_possible_queen_move(&box_kind, &c),
            ChessPieceKind::King(c) => self.chess_possible_king_move(&box_kind, &c),
        }
    }

    fn get_all_possible_move(&self, color_kind: &ChessPieceColor) -> Vec<ChessTurnAction> {
        let mut moves = vec![];

        if color_kind == &ChessPieceColor::White {
            for f in &self.active_white_piece {
                let result = self.chess_possible_move(f.piece.unwrap(), f.kind);
                for r in result {
                    moves.push(r);
                }
            }
        } else {
            for f in &self.active_black_piece {
                let result = self.chess_possible_move(f.piece.unwrap(), f.kind);
                for r in result {
                    moves.push(r);
                }
            }
        }

        moves
    }
}

#[cfg(test)]
mod test {
    use crate::r#move::ChessMove;
    use crate::tray::Tray;

    #[test]
    fn test_possible_pawn_move() {
        let mut tray = Tray::new();
        tray.build_new_tray();
        tray.build_default_game_tray();
    }
}
