use crate::chess_structs::{
    ChessBoard, ChessState, InfoMessage, InteractionType, Move, Player, Winner,
};


/**
 * Main Difference between chess_state and chess_board
 * is that chess_state holds onto the player, and ui information like info_message
 * or if the user needs to select a piece and can't play until they do
 *
 */
impl ChessState {
    pub fn new() -> Self {
        let chessboard = ChessBoard::new();
        let player = Player::White;
        let winner = Winner::NoneYet;
        let info_message = None;
        let chess_state = ChessState {
            chessboard,
            player,
            winner,
            info_message,
            require_piece_selection: false,
            turn_count: 0,
        };
        return chess_state;
    }
    /**
     * Update the following fields from a move
     * - chessboard (updated by chessboard class)
     * - player
     * - winner
     * - info_message
     * - require_piece_selection ( does the player need to select a piece for a pawn promotion )
     * - turn_count
     */
    pub fn move_piece(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        if self.require_piece_selection {
            return false;
        }
        if !self
            .chessboard
            .is_move_valid(from_row, from_col, to_row, to_col, self.player)
        {
            return false;
        }
        self.chessboard =
            self.chessboard
                .move_piece(from_row, from_col, to_row, to_col, self.player.clone());
        let interaction_type = self.chessboard.last_move_interaction_type();
        let mut moves_available = true;
        let is_super_effective = interaction_type == Some(InteractionType::SuperEffective);
        let pawn_promotion = self.chessboard.history.last_move_requires_pawn_promotion();
        if is_super_effective {
            // check if the piece has moves available
            let moves =
                self.chessboard
                    .possible_moves_for_piece(to_row, to_col, self.player.clone());
            if moves.is_empty() && !pawn_promotion {
                // flip it over to the other player and update the info message to match
                self.player = self.player.other_player();
                moves_available = false;
            }
        } else {
            if !pawn_promotion {
                self.player = self.player.other_player();
            }
        }
        self.info_message = InfoMessage::get_message_from_interaction_type(
            interaction_type.unwrap_or(InteractionType::Normal),
            moves_available,
        );
        self.require_piece_selection = pawn_promotion;
        // check if the game is over
        // after new player is set
        self.winner = self.chessboard.get_winner(self.player);
        self.turn_count += 1;
        return true;
    }

    pub fn get_valid_moves(&self, row: usize, col: usize) -> Vec<Move> {
        if self.chessboard.get_winner(self.player) != Winner::NoneYet {
            return vec![];
        }
        let moves = self
            .chessboard
            .possible_moves_for_piece(row, col, self.player);
        let current_player = self.player.clone();
        let mut valid_moves = Vec::new();
        for m in moves {
            let mut new_board = self.chessboard.clone();
            new_board =
                new_board.move_piece(m.from_row, m.from_col, m.to_row, m.to_col, self.player);
            if !new_board.is_king_in_check(current_player) {
                valid_moves.push(m);
            }
        }
        return valid_moves;
    }

    pub fn other_player_considering_board(&mut self) -> Player {
        return self.player.other_player_considering_board(&self.chessboard);
    }

    pub fn select_pawn_promotion_piece(&mut self, piece_str: String) -> Result<(), String> {
        let result = self
            .chessboard
            .select_pawn_promotion_piece(piece_str, self.player);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        self.player = self.other_player_considering_board();
        self.require_piece_selection = false;
        return Ok(());
    }
}

// add a quick test on get_valid_moves for the first pawn
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid_moves() {
        let chess_state = ChessState::new();
        let valid_moves = chess_state.get_valid_moves(1, 0);
        assert!(valid_moves.len() == 2);
        assert!(valid_moves[0].to_row == 2);
        assert!(valid_moves[0].to_col == 0);
        assert!(valid_moves[1].to_row == 3);
        assert!(valid_moves[1].to_col == 0);
    }
}
