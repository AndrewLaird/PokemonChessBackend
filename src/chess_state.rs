use crate::chess_structs::{ChessState, InfoMessage, InteractionType};

impl ChessState {
    /**
     * Update the following fields from a move
     * - chessboard
     * - player
     * - winner
     * - info_message
     * - require_piece_selection ( does the player need to select a piece for a pawn promotion )
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
        let (chessboard, interaction_type) =
            self.chessboard
                .move_piece(from_row, from_col, to_row, to_col, self.player.clone());
        self.chessboard = chessboard.clone();
        let mut moves_available = true;
        let is_super_effective = interaction_type == Some(InteractionType::SuperEffective);
        let pawn_promotion = self.chessboard.history.last_move_requires_pawn_promotion();
        if is_super_effective {
            // check if the piece has moves available
            let moves = chessboard.possible_moves_for_piece(to_row, to_col, self.player.clone());
            if moves.is_empty() && !pawn_promotion {
                // flip it over to the other player and update the info message to match
                self.player = self.player.other_player();
                moves_available = false;
            }
        }
        else {
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
        return true;
    }
}
