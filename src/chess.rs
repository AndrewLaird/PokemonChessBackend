// Rust chess class that holds a board
// and the logic for playing the game

use rand::seq::SliceRandom;
use rand::thread_rng;

pub const BOARD_SIZE: usize = 8;

use crate::chess_structs::{
    Capture, ChessBoard, ChessHistory, ChessPieceType, InteractionType, Move, Piece, Player,
    PokemonType, Winner,
};

impl ChessBoard {
    pub fn new() -> Self {
        let board = ChessBoard::initialize_board();
        return board;
    }
    pub fn new_normal_type_only() -> Self {
        let board = ChessBoard::initialize_board_all_normal();
        return board;
    }

    

    pub fn is_king_in_check(&self, player: Player) -> bool {
        let king_position = self.find_king_position(player.clone());
        match king_position {
            Some(king_position) => self.pieces_attacking_king(king_position, player),
            None => false,
        }
    }

    pub fn last_move_interaction_type(&self) -> Option<InteractionType> {
        let last_move = self.history.last_move();
        if let Some(last_move) = last_move {
            return last_move.type_interaction
        }
        return None
    }

    pub fn get_winner(&self, current_player: Player) -> Winner {
        let opponent = match current_player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        let current_king_position = self.find_king_position(current_player);
        let opponent_king_position = self.find_king_position(opponent);

        match (current_king_position, opponent_king_position) {
            (None, None) => Winner::Tie,
            (Some(_), None) => Winner::from_player(current_player),
            (None, Some(_)) => Winner::from_player(opponent),
            (Some(current_king_pos), Some(_)) => {
                let current_king_in_check = self.pieces_attacking_king(current_king_pos, current_player);
                let current_king_has_moves = !self
                    .possible_moves_for_piece_unfiltered(current_king_pos.0, current_king_pos.1, current_player)
                    .is_empty();

                if current_king_in_check && !current_king_has_moves {
                    Winner::from_player(opponent)
                } else {
                    Winner::NoneYet
                }
            }
        }
    }

    fn possible_moves_for_piece_unfiltered(&self, row: usize, col: usize, player: Player) -> Vec<Move> {

        let piece: Piece = self.get_piece(row, col);
        // make sure they are the same type, white or black
        if piece.piece_type.is_white() != (player == Player::White) {
            return vec![];
        }
        let mut moves = piece.piece_type.available_moves(row, col, self);
        for move_obj in &mut moves {
            // update with type interactions
            let other_piece = self.get_piece(move_obj.to_row, move_obj.to_col);
            let type_matchup: InteractionType =
                PokemonType::type_matchup(piece.pokemon_type, other_piece.pokemon_type);
            move_obj.type_interaction = Some(type_matchup);
        }
        return moves;
    }

    pub fn possible_moves_for_piece(&self, row: usize, col: usize, player: Player) -> Vec<Move> {
        let mut moves = self.possible_moves_for_piece_unfiltered(row, col, player);
        if let Some(position) = self.history.last_move_super_effective() {
            moves = ChessBoard::filter_moves_if_super_effective(moves, position);
        }

        return moves;
    }

    fn filter_moves_if_super_effective(moves: Vec<Move>, position: (usize, usize)) -> Vec<Move> {
        let mut filtered_moves = Vec::new();
        for m in moves {
            if m.from_row == position.0 && m.from_col == position.1 {
                filtered_moves.push(m);
            }
        }
        return filtered_moves;
    }

    // Separate because the piece does not always cover the captured piece
    // specifically for en passant
    pub fn capture_piece(&mut self, capture: Capture, _move_to_execute: Move) {
        self.board[capture.row][capture.col] = Piece::empty();
    }

    pub fn move_piece(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
        player: Player,
    ) -> ChessBoard {
        if self.is_move_valid(from_row, from_col, to_row, to_col, player) {
            return self.clone().execute_move(from_row, from_col, to_row, to_col);
        } 
        return self.clone();
    }

    pub fn is_move_valid(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
        player: Player,
    ) -> bool {
        let possible_moves = self.possible_moves_for_piece(from_row, from_col, player);
        possible_moves
            .iter()
            .any(|pm| pm.to_row == to_row && pm.to_col == to_col)
    }

    fn find_move(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> Option<Move> {
        // Since possible_moves_for_piece might add type interactions or other state,
        // we fetch the current piece and check its valid moves.
        let player = if self.get_piece(from_row, from_col).piece_type.is_white() {
            Player::White
        } else {
            Player::Black
        };

        let possible_moves = self.possible_moves_for_piece(from_row, from_col, player);
        return possible_moves
            .into_iter()
            .find(|m| m.to_row == to_row && m.to_col == to_col);
    }

    fn execute_move(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> ChessBoard {
        let mut new_board = self.clone();
        let piece = self.get_piece(from_row, from_col);
        let move_to_execute = self.find_move(from_row, from_col, to_row, to_col).unwrap(); 
        println!("{:?}", move_to_execute);

        // Check the interaction type and handle "Not Very Effective" outcome
        if let Some(type_interaction) = move_to_execute.type_interaction {
            match type_interaction {
                InteractionType::NotVeryEffective => {
                    // Destroy both pieces if the interaction is "Not Very Effective"
                    new_board.board[from_row][from_col] = Piece::empty(); // Remove the attacking piece
                    new_board.board[to_row][to_col] = Piece::empty(); // Remove the defending piece
                }
                InteractionType::NoEffect => {
                    // Neither piece is destroyed if the interaction is "No Effect"
                }
                InteractionType::SuperEffective => {
                    // handle the piece normally but don't update the player unless there are no
                    // available moves for the piece that landed here
                    new_board.handle_captures_and_special_moves(&move_to_execute);
                    new_board.board[to_row][to_col] = piece; // Place the attacking piece in the new position
                    new_board.board[from_row][from_col] = Piece::empty(); // Remove the attacking piece from the old position
                }
                _ => {
                    // Handle other types of interactions
                    new_board.handle_captures_and_special_moves(&move_to_execute);
                    new_board.board[to_row][to_col] = piece; // Place the attacking piece in the new position
                    new_board.board[from_row][from_col] = Piece::empty(); // Remove the attacking piece from the old position
                }
            }
        } else {
            // If there's no type interaction, proceed with the move normally
            new_board.handle_captures_and_special_moves(&move_to_execute);
            new_board.board[to_row][to_col] = piece; // Place the attacking piece in the new position
            new_board.board[from_row][from_col] = Piece::empty(); // Remove the attacking piece from the old position
        }
        new_board.history.add_move(move_to_execute);
        // if a pawn has moved to the end of the board, promote it to a queen
        return new_board
    }


    pub fn get_piece(&self, row: usize, col: usize) -> Piece {
        return self.board[row][col];
    }
    // For testing normal chess rules
    #[allow(dead_code)]
    fn initialize_board_all_normal() -> Self {
        let white_types = vec![
            PokemonType::Normal; 2*BOARD_SIZE
        ];

        let black_types = vec![
            PokemonType::Normal; 2*BOARD_SIZE
        ];
        let board = Self::create_board_with_types(white_types.clone(), black_types.clone());

        return ChessBoard {
            board,
            history: ChessHistory::new(),
        };

    }

    fn initialize_board() -> Self {
        let mut rng = thread_rng();

        // Initialize pieces to a random pokemon type but don't repeat
        let mut white_types = vec![
            PokemonType::Normal,
            PokemonType::Fire,
            PokemonType::Water,
            PokemonType::Electric,
            PokemonType::Grass,
            PokemonType::Ice,
            PokemonType::Fighting,
            PokemonType::Poison,
            PokemonType::Ground,
            PokemonType::Flying,
            PokemonType::Psychic,
            PokemonType::Bug,
            PokemonType::Rock,
            PokemonType::Ghost,
            PokemonType::Dragon,
            PokemonType::Dark,
            PokemonType::Steel,
            PokemonType::Fairy,
        ];
        let mut black_types = white_types.clone();

        white_types.shuffle(&mut rng);
        black_types.shuffle(&mut rng);
        let board = Self::create_board_with_types(white_types.clone(), black_types.clone());


        ChessBoard {
            board,
            history: ChessHistory::new(),
        }
    }

    fn create_board_with_types(mut white_types: Vec<PokemonType>, mut black_types: Vec<PokemonType>) -> [[Piece; BOARD_SIZE]; BOARD_SIZE] {
        let empty_piece = Piece::empty();
        let mut board = [[empty_piece; BOARD_SIZE]; BOARD_SIZE];
        // Initialize white pieces
        board[0] = [
            Piece {
                piece_type: ChessPieceType::WhiteRook,
                pokemon_type: white_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::WhiteKnight,
                pokemon_type: white_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::WhiteBishop,
                pokemon_type: white_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::WhiteQueen,
                pokemon_type: white_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::WhiteKing,
                pokemon_type: white_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::WhiteBishop,
                pokemon_type: white_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::WhiteKnight,
                pokemon_type: white_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::WhiteRook,
                pokemon_type: white_types.pop().unwrap(),
            },
        ];
        for col in 0..BOARD_SIZE {
            board[1][col] = Piece {
                piece_type: ChessPieceType::WhitePawn,
                pokemon_type: white_types.pop().unwrap(),
            };
        }

        // Initialize black pieces
        board[7] = [
            Piece {
                piece_type: ChessPieceType::BlackRook,
                pokemon_type: black_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::BlackKnight,
                pokemon_type: black_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::BlackBishop,
                pokemon_type: black_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::BlackQueen,
                pokemon_type: black_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::BlackKing,
                pokemon_type: black_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::BlackBishop,
                pokemon_type: black_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::BlackKnight,
                pokemon_type: black_types.pop().unwrap(),
            },
            Piece {
                piece_type: ChessPieceType::BlackRook,
                pokemon_type: black_types.pop().unwrap(),
            },
        ];
        for col in 0..BOARD_SIZE {
            board[6][col] = Piece {
                piece_type: ChessPieceType::BlackPawn,
                pokemon_type: black_types.pop().unwrap(),
            };
        }
        return board;
        
    }

    fn format_piece(piece: Piece) -> String {
        let color = match piece.piece_type {
            ChessPieceType::WhitePawn
            | ChessPieceType::WhiteKnight
            | ChessPieceType::WhiteBishop
            | ChessPieceType::WhiteRook
            | ChessPieceType::WhiteQueen
            | ChessPieceType::WhiteKing => "W",
            ChessPieceType::BlackPawn
            | ChessPieceType::BlackKnight
            | ChessPieceType::BlackBishop
            | ChessPieceType::BlackRook
            | ChessPieceType::BlackQueen
            | ChessPieceType::BlackKing => "B",
            _ => " ",
        };
        let chess_type = match piece.piece_type {
            ChessPieceType::Empty => "",
            ChessPieceType::WhitePawn | ChessPieceType::BlackPawn => "P",
            ChessPieceType::WhiteKnight | ChessPieceType::BlackKnight => "N",
            ChessPieceType::WhiteBishop | ChessPieceType::BlackBishop => "B",
            ChessPieceType::WhiteRook | ChessPieceType::BlackRook => "R",
            ChessPieceType::WhiteQueen | ChessPieceType::BlackQueen => "Q",
            ChessPieceType::WhiteKing | ChessPieceType::BlackKing => "K",
        };
        let pokemon_type = match piece.piece_type {
            ChessPieceType::Empty => "     ".to_string(),
            _ => {
                let full_str = format!("{:?}", piece.pokemon_type);
                let truncated_str = &full_str[0..full_str.len().min(4)];
                format!("{:4}", truncated_str)
            }
        };
        format!("{}{} {}", color, chess_type, pokemon_type)
    }

    pub fn display_board_str(&self) -> String {
        let mut result = String::new();

        for i in (0..BOARD_SIZE).rev() {
            for j in 0..BOARD_SIZE {
                let piece = self.board[i][j];
                result += &format!("|{}| ", ChessBoard::format_piece(piece));
            }
            result += "\n";
            result +=
                "-------------------------------------------------------------------------------\n";
        }
        result += "\n";

        return result;
    }

    pub fn display_board(&self) {
        for i in (0..BOARD_SIZE).rev() {
            for j in 0..BOARD_SIZE {
                let piece = self.board[i][j];
                print!("|{}| ", ChessBoard::format_piece(piece));
            }
            println!();
            println!(
                "-------------------------------------------------------------------------------"
            );
        }
        println!();
    }

    pub fn select_pawn_promotion_piece(
        &mut self,
        piece_str: String,
        player: Player,
    ) -> Result<(), String> {
        if let Some(location) = self.history.last_move() {
            let piece_type =
                ChessPieceType::select_piece_from_string_and_player(&piece_str, player);
            self.board[location.to_row][location.to_col] = Piece {
                piece_type,
                pokemon_type: self.board[location.to_row][location.to_col].pokemon_type,
            };
            return Ok(());
        }
        return Err("No last move".to_string());
    }

    fn piece_same_as_player(piece: Piece, player: &Player) -> bool {
        return piece.piece_type.is_white() == (player == &Player::White);
    }

    fn piece_opposite_to_player(piece: Piece, player: &Player) -> bool {
        return piece.piece_type.is_white() != (player == &Player::White);
    }

    fn find_king_position(&self, player: Player) -> Option<(usize, usize)> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let piece = self.get_piece(row, col);
                if piece.piece_type.is_king() && ChessBoard::piece_same_as_player(piece, &player) {
                    return Some((row, col));
                }
            }
        }
        return None;
    }

    fn pieces_attacking_king(&self, king_position: (usize, usize), player: Player) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let piece = self.get_piece(row, col);
                if ChessBoard::piece_opposite_to_player(piece, &player) {
                    let moves = piece.piece_type.available_moves(row, col, self);
                    for piece_move in moves {
                        if piece_move.to_row == king_position.0
                            && piece_move.to_col == king_position.1
                        {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    fn handle_captures_and_special_moves(&mut self, move_to_execute: &Move) {
        self.capture_piece_if_applicable(move_to_execute);
        self.handle_castling_if_applicable(move_to_execute);
    }

    fn capture_piece_if_applicable(&mut self, move_to_execute: &Move) {
        if let Some(capture) = move_to_execute.capture {
            self.capture_piece(capture, *move_to_execute);
        }
    }

    fn handle_castling_if_applicable(&mut self, move_to_execute: &Move) {
        if let Some(castle) = move_to_execute.castle {
            self.board[castle.rook_to_row][castle.rook_to_col] =
                self.get_piece(castle.rook_from_row, castle.rook_from_col);
            self.board[castle.rook_from_row][castle.rook_from_col] = Piece::empty();
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_castling_success() {
        // move white pieces out of the way so the king can castle king-side
        let mut board = ChessBoard::new_normal_type_only();
        // Move pawn so the bishop can move
        board = board.move_piece(1, 4, 3, 4, Player::White);
        // move the bishop out of the way
        board = board.move_piece(0, 5, 1, 4, Player::White);
        // move the knight out of the way
        board = board.move_piece(0, 6, 2, 5, Player::White);
        // castle the king
        board = board.move_piece(0, 4, 0, 6, Player::White);
        // assert that 0, 6 is the white king
        assert!(board.get_piece(0, 6).piece_type == ChessPieceType::WhiteKing);
        // and 0, 5 is the white rook
        assert!(board.get_piece(0, 5).piece_type == ChessPieceType::WhiteRook);
    }

    #[test]
    fn test_history_castling_unsucessful_rook_moved() {
        // Same scenario as above but we move the rook out and back to it's position
        // making the castle invalid, and we should see that it's invalid 
        let mut board = ChessBoard::new_normal_type_only();
        // Move pawn so the bishop can move
        board = board.move_piece(1, 4, 3, 4, Player::White);
        // move the bishop out of the way
        board = board.move_piece(0, 5, 1, 4, Player::White);
        // move the knight out of the way
        board = board.move_piece(0, 6, 2, 5, Player::White);
        // move the rook next to the king and then back
        board = board.move_piece(0, 7, 0, 5, Player::White);
        board = board.move_piece(0, 5, 0, 7, Player::White);

        // Try to castle the king
        board = board.move_piece(0, 4, 0, 6, Player::White);
        // see that the king is still the same spot because the castling is invalid
        assert!(board.get_piece(0, 4).piece_type == ChessPieceType::WhiteKing);

    }

    #[test]
    fn test_valid_en_passant() {
        let mut board = ChessBoard::new_normal_type_only();
        
        // Move white pawn to set up en passant
        board = board.move_piece(1, 4, 3, 4, Player::White);
        board = board.move_piece(3, 4, 4, 4, Player::White);
        
        // Move black pawn two squares forward, enabling en passant
        board = board.move_piece(6, 3, 4, 3, Player::Black);
        
        // take the en_passant, esnure the black pawn is no longer there
        println!("{}", board.display_board_str());
        board = board.move_piece(4, 4, 5, 3, Player::White);
        println!("{}", board.display_board_str());
        assert!(board.get_piece(4, 3).piece_type == ChessPieceType::Empty);
        
    }

    #[test]
    fn test_invalid_en_passant_with_piece_to_left() {
        let mut board = ChessBoard::new_normal_type_only();
        
        // Move white pawn to set up the scenario
        board = board.move_piece(1, 4, 3, 4, Player::White);
        board = board.move_piece(3, 4, 4, 4, Player::White);
        
        // Move black pawn one square forward (not enabling en passant)
        board = board.move_piece(6, 3, 5, 3, Player::Black);
        
        // Move a black piece to the left of the white pawn
        board = board.move_piece(7, 0, 4, 3, Player::Black);

        // Capture diagonally with the white pawn making sure that the pieces that should be are
        // still there
        board = board.move_piece(4, 4, 5, 3, Player::White);
        board = board.move_piece(5, 3, 6, 2, Player::White);
        board = board.move_piece(6, 2, 7, 1, Player::White);
        println!("{}", board.display_board_str());
        
        
        // Verify that we didn't capture the black pawn that was in front of the piece we did
        // capture
        assert!(matches!(board.get_piece(6, 1).piece_type, ChessPieceType::BlackPawn));
    }
}
