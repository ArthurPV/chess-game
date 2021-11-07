use crate::piece::ChessPieceColor;
use crate::piece::ChessPieceKind;
use crate::tray::ChessBox;
use crate::tray::ChessBoxKind;
use crate::tray::Tray;
use crate::turn::ChessTurnAction;

pub trait ChessMove {
    fn chess_possible_pawn_move(
        &mut self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction>;
    fn chess_possible_bishop_move(&self) -> Vec<ChessTurnAction>;
    fn chess_possible_knight_move(&self) -> Vec<ChessTurnAction>;
    fn chess_possible_rook_move(&self) -> Vec<ChessTurnAction>;
    fn chess_possible_queen_move(&self) -> Vec<ChessTurnAction>;
    fn chess_possible_king_move(&self) -> Vec<ChessTurnAction>;
    fn chess_modify_box(&mut self, box_kind: &ChessBoxKind, piece: Option<ChessPieceKind>);
    fn chess_possible_move(
        &mut self,
        piece_kind: ChessPieceKind,
        box_kind: ChessBoxKind,
    ) -> Vec<ChessTurnAction>;
}

impl ChessMove for Tray {
    fn chess_possible_pawn_move(
        &mut self,
        box_kind: &ChessBoxKind,
        color_kind: &ChessPieceColor,
    ) -> Vec<ChessTurnAction> {
        let mut p_move = vec![];

        let line = ChessBoxKind::get_line_code(box_kind);
        let column = ChessBoxKind::get_column_code(box_kind);

        // advance one case
        {
            let box_above = self.get_box(&ChessBoxKind::location_to_box_kind((line + 1, column)));

            match box_above.piece {
                Some(_) => (),
                None => p_move.push(ChessTurnAction::new_move(
                    ChessPieceKind::Pawn(*color_kind),
                    box_above,
                )),
            }
        }

        // advance two cases
        {
            let box_above = self.get_box(&ChessBoxKind::location_to_box_kind((line + 2, column)));

            match box_above.piece {
                Some(_) => (),
                None if color_kind == &ChessPieceColor::Black => {
                    match ChessBoxKind::location_to_box_kind((line, column)) {
                        ChessBoxKind::A7
                        | ChessBoxKind::B7
                        | ChessBoxKind::C7
                        | ChessBoxKind::D7
                        | ChessBoxKind::E7
                        | ChessBoxKind::F7
                        | ChessBoxKind::G7
                        | ChessBoxKind::H7 => p_move.push(ChessTurnAction::new_move(
                            ChessPieceKind::Pawn(*color_kind),
                            box_above,
                        )),
                        _ => (),
                    }
                }
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
                        box_above,
                    )),
                    _ => (),
                },
            }
        }

        // eat piece
        {
            if column == 0 {
                let eat = self.get_box(&ChessBoxKind::location_to_box_kind((line, column + 1)));

                match eat.piece {
                    Some(v) if ChessPieceKind::is_edible(&v, color_kind) => p_move.push(
                        ChessTurnAction::new_eat(ChessPieceKind::Pawn(*color_kind), eat),
                    ),
                    Some(_) => (),
                    None => (),
                }
            } else if column == 7 {
                let eat = self.get_box(&ChessBoxKind::location_to_box_kind((line, column - 1)));

                match eat.piece {
                    Some(v) if ChessPieceKind::is_edible(&v, color_kind) => p_move.push(
                        ChessTurnAction::new_eat(ChessPieceKind::Pawn(*color_kind), eat),
                    ),
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
                        Some(v) if ChessPieceKind::is_edible(&v, color_kind) => {
                            p_move.push(ChessTurnAction::new_eat(ChessPieceKind::Pawn(*color_kind), eat[i].clone()))
                        }
                        Some(_) => (),
                        None => (),
                    }
                }
            }
        }

        p_move
    }

    fn chess_possible_bishop_move(&self) -> Vec<ChessTurnAction> {
        todo!()
    }

    fn chess_possible_knight_move(&self) -> Vec<ChessTurnAction> {
        todo!()
    }

    fn chess_possible_rook_move(&self) -> Vec<ChessTurnAction> {
        todo!()
    }

    fn chess_possible_queen_move(&self) -> Vec<ChessTurnAction> {
        todo!()
    }

    fn chess_possible_king_move(&self) -> Vec<ChessTurnAction> {
        todo!()
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
                self.tray.0[ChessBoxKind::get_column_code(box_kind)].piece = piece
            }
            &ChessBoxKind::A2
            | &ChessBoxKind::B2
            | &ChessBoxKind::C2
            | &ChessBoxKind::D2
            | &ChessBoxKind::E2
            | &ChessBoxKind::F2
            | &ChessBoxKind::G2
            | &ChessBoxKind::H2 => {
                self.tray.1[ChessBoxKind::get_column_code(box_kind)].piece = piece
            }
            &ChessBoxKind::A3
            | &ChessBoxKind::B3
            | &ChessBoxKind::C3
            | &ChessBoxKind::D3
            | &ChessBoxKind::E3
            | &ChessBoxKind::F3
            | &ChessBoxKind::G3
            | &ChessBoxKind::H3 => {
                self.tray.2[ChessBoxKind::get_column_code(box_kind)].piece = piece
            }
            &ChessBoxKind::A4
            | &ChessBoxKind::B4
            | &ChessBoxKind::C4
            | &ChessBoxKind::D4
            | &ChessBoxKind::E4
            | &ChessBoxKind::F4
            | &ChessBoxKind::G4
            | &ChessBoxKind::H4 => {
                self.tray.3[ChessBoxKind::get_column_code(box_kind)].piece = piece
            }
            &ChessBoxKind::A5
            | &ChessBoxKind::B5
            | &ChessBoxKind::C5
            | &ChessBoxKind::D5
            | &ChessBoxKind::E5
            | &ChessBoxKind::F5
            | &ChessBoxKind::G5
            | &ChessBoxKind::H5 => {
                self.tray.4[ChessBoxKind::get_column_code(box_kind)].piece = piece
            }
            &ChessBoxKind::A6
            | &ChessBoxKind::B6
            | &ChessBoxKind::C6
            | &ChessBoxKind::D6
            | &ChessBoxKind::E6
            | &ChessBoxKind::F6
            | &ChessBoxKind::G6
            | &ChessBoxKind::H6 => {
                self.tray.5[ChessBoxKind::get_column_code(box_kind)].piece = piece
            }
            &ChessBoxKind::A7
            | &ChessBoxKind::B7
            | &ChessBoxKind::C7
            | &ChessBoxKind::D7
            | &ChessBoxKind::E7
            | &ChessBoxKind::F7
            | &ChessBoxKind::G7
            | &ChessBoxKind::H7 => {
                self.tray.6[ChessBoxKind::get_column_code(box_kind)].piece = piece
            }
            &ChessBoxKind::A8
            | &ChessBoxKind::B8
            | &ChessBoxKind::C8
            | &ChessBoxKind::D8
            | &ChessBoxKind::E8
            | &ChessBoxKind::F8
            | &ChessBoxKind::G8
            | &ChessBoxKind::H8 => {
                self.tray.7[ChessBoxKind::get_column_code(box_kind)].piece = piece
            }
            &ChessBoxKind::Unknown => (),
        }
    }

    fn chess_possible_move(
        &mut self,
        piece_kind: ChessPieceKind,
        box_kind: ChessBoxKind,
    ) -> Vec<ChessTurnAction> {
        match piece_kind {
            ChessPieceKind::Pawn(ChessPieceColor::Black) => {
                self.chess_possible_pawn_move(&box_kind, &ChessPieceColor::Black)
            }
            ChessPieceKind::Pawn(ChessPieceColor::White) => {
                self.chess_possible_pawn_move(&box_kind, &ChessPieceColor::White)
            }
            ChessPieceKind::Bishop(_, _) => self.chess_possible_bishop_move(),
            ChessPieceKind::Knight(_) => self.chess_possible_knight_move(),
            ChessPieceKind::Rook(_) => self.chess_possible_rook_move(),
            ChessPieceKind::Queen(_) => self.chess_possible_queen_move(),
            ChessPieceKind::King(_) => self.chess_possible_king_move(),
        }
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
