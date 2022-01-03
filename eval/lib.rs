use chess_game::ChessBox;
use chess_game::ChessMove;
use chess_game::ChessPieceColor;
use chess_game::ChessPieceKind;
use chess_game::Tray;

#[derive(Clone)]
pub struct Eval<'a> {
    pub tray: &'a Tray,
    pub black_eval: isize,
    pub white_eval: isize,
    pub total_eval: isize,
}

impl<'a> Eval<'a> {
    pub fn new(tray: &'a Tray) -> Eval<'a> {
        Eval {
            tray,
            black_eval: 0,
            white_eval: 0,
            total_eval: 0,
        }
    }

    fn eval_black_piece(&mut self) {
        for b in &self.tray.active_black_piece {
            match b.piece {
                Some(p) => {
                    let value = p.chess_piece_to_value();
                    let possible_move = self.tray.chess_possible_move(p, b.kind.clone());
                    self.black_eval -= (value) as isize * 10 + (possible_move.len()) as isize;
                }
                None => unreachable!(),
            }
        }
    }

    fn eval_white_piece(&mut self) {
        for b in &self.tray.active_white_piece {
            match b.piece {
                Some(p) => {
                    let value = p.chess_piece_to_value();
                    let possible_move = self.tray.chess_possible_move(p, b.kind.clone());
                    self.white_eval += (value) as isize * 10 + (possible_move.len()) as isize;
                }
                None => unreachable!(),
            }
        }
    }

    pub fn eval_total(&mut self) {
        self.eval_black_piece();
        self.eval_white_piece();
        self.total_eval = self.white_eval + self.black_eval;
    }
}
