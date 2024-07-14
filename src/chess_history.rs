use crate::chess_structs::{ChessHistory, ChessPieceType, InteractionType, Move};

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

    // Check if the last move was a two-square pawn move, which enables en passant
    pub fn last_move_enables_en_passant(&self) -> Option<(usize, usize)> {
        if let Some(last_move) = self.move_history.last() {
            // Check if the last move was made by a pawn
            let is_pawn_move = matches!(
                last_move.piece_type,
                ChessPieceType::WhitePawn | ChessPieceType::BlackPawn
            );

            // Check if the move was a two-square advance from the starting position
            let is_two_square_advance =
                (last_move.from_row as isize - last_move.to_row as isize).abs() == 2;

            // Check if the pawn has not moved from its original row to qualify for en passant
            let is_initial_move = match last_move.piece_type {
                ChessPieceType::WhitePawn => last_move.from_row == 1,
                ChessPieceType::BlackPawn => last_move.from_row == 6,
                _ => false,
            };

            if is_pawn_move && is_two_square_advance && is_initial_move {
                // Return the position to which the pawn moved, enabling en passant
                return Some((last_move.to_row, last_move.to_col));
            }
        }

        // If the last move does not enable en passant, return None
        None
    }

    /**
     * returns None if the last move wasn't super effective
     * otherwise returns the position of the piece that made the super effective move
     *
     */
    pub fn last_move_super_effective(&self) -> Option<(usize, usize)> {
        if let Some(last_move) = self.move_history.last() {
            if last_move
                .type_interaction
                .unwrap_or(InteractionType::Normal)
                == InteractionType::SuperEffective
            {
                return Some((last_move.to_row, last_move.to_col));
            }
        }

        return None;
    }

    pub fn last_move_requires_pawn_promotion(&self) -> bool {
        if let Some(last_move) = self.move_history.last() {
            if last_move.piece_type == ChessPieceType::WhitePawn
                && last_move.to_row == ChessHistory::BLACK_KING_START_ROW
            {
                return true;
            }
            if last_move.piece_type == ChessPieceType::BlackPawn
                && last_move.to_row == ChessHistory::WHITE_KING_START_ROW
            {
                return true;
            }
        }
        return false;
    }

    pub fn last_move(&self) -> Option<Move> {
        return self.move_history.last().cloned();
    }
}
