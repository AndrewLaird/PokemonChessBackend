use crate::chess_structs::{ChessHistory, ChessPieceType, Move};

// Certain Special Moves need more information
// en passant needs to know the last move
// castling needs to know if the king or rook has moved
// and if the king has line of sight to the rook
impl ChessHistory {
    const WHITE_KING_START_ROW: usize = 0;
    const BLACK_KING_START_ROW: usize = 7;
    const KING_START_COL: usize = 4;

    const QUEEN_SIDE_ROOK_COL: usize = 0;
    const KING_SIDE_ROOK_COL: usize = 7;

    pub fn new() -> Self {
        return ChessHistory {
            move_history: Vec::new(),
        };
    }

    pub fn from_vec(moves: Vec<Move>) -> Self {
        return ChessHistory {
            move_history: moves,
        };
    }

    pub fn add_move(&mut self, m: Move) {
        self.move_history.push(m);
    }

    pub fn pawn_moved_last_turn(&self, row: usize, col: usize) -> bool {
        if self.move_history.len() == 0 {
            return false;
        }
        let last_move = self.move_history.last().unwrap();

        return (last_move.piece_type == ChessPieceType::WhitePawn
            || last_move.piece_type == ChessPieceType::BlackPawn)
            && last_move.to_row == row
            && last_move.to_col == col;
    }

    // Maybe we shoud just keep data on this
    pub fn rook_has_not_moved(&self, row: usize, col: usize) -> bool {
        if self.move_history.len() == 0 {
            return true;
        }
        for m in self.move_history.iter().rev() {
            if (m.piece_type == ChessPieceType::WhiteRook
                || m.piece_type == ChessPieceType::BlackRook)
                && m.to_row == row
                && m.to_col == col
            {
                return false;
            }
        }
        return true;
    }

    pub fn king_has_not_moved(&self, row: usize, col: usize) -> bool {
        if self.move_history.len() == 0 {
            return true;
        }
        for m in self.move_history.iter().rev() {
            if (m.piece_type == ChessPieceType::WhiteKing
                || m.piece_type == ChessPieceType::BlackKing)
                && m.to_row == row
                && m.to_col == col
            {
                return false;
            }
        }
        return true;
    }

    pub fn can_castle_kingside(&self, is_white: bool) -> bool {
        if is_white {
            return self.rook_has_not_moved(
                ChessHistory::WHITE_KING_START_ROW,
                ChessHistory::KING_SIDE_ROOK_COL,
            ) && self.king_has_not_moved(
                ChessHistory::WHITE_KING_START_ROW,
                ChessHistory::KING_START_COL,
            );
        } else {
            return self.rook_has_not_moved(
                ChessHistory::BLACK_KING_START_ROW,
                ChessHistory::KING_SIDE_ROOK_COL,
            ) && self.king_has_not_moved(
                ChessHistory::BLACK_KING_START_ROW,
                ChessHistory::KING_START_COL,
            );
        }
    }
    pub fn can_castle_queenside(&self, is_white: bool) -> bool {
        if is_white {
            return self.rook_has_not_moved(
                ChessHistory::WHITE_KING_START_ROW,
                ChessHistory::QUEEN_SIDE_ROOK_COL,
            ) && self.king_has_not_moved(
                ChessHistory::WHITE_KING_START_ROW,
                ChessHistory::KING_START_COL,
            );
        } else {
            return self.rook_has_not_moved(
                ChessHistory::BLACK_KING_START_ROW,
                ChessHistory::QUEEN_SIDE_ROOK_COL,
            ) && self.king_has_not_moved(
                ChessHistory::BLACK_KING_START_ROW,
                ChessHistory::KING_START_COL,
            );
        }
    }
}
