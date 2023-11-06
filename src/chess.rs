// Rust chess class that holds a board
// and the logic for playing the game

use rand::seq::SliceRandom;
use rand::thread_rng;

pub const BOARD_SIZE: usize = 8;

pub use crate::chess_structs::{
    ChessBoard, ChessHistory, ChessPieceType, Move, Piece, Player, PokemonType, InteractionType, Capture, Winner
};

impl ChessBoard {
    pub fn new() -> Self {
        let board = ChessBoard::initialize_board();
        return board;
    }

    pub fn piece_same_as_player(piece: Piece, player: &Player) -> bool {
        return piece.piece_type.is_white() == (player == &Player::White);
    }

    pub fn find_king_position(&self, player: Player) -> (usize, usize) {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let piece = self.get_piece(row, col);
                if ChessBoard::piece_same_as_player(piece, &player) {
                    return (row, col);
                }
            }
        }
        return (usize::MAX, usize::MAX);
    }

    pub fn pieces_attacking_king(&self, king_position: (usize, usize), player: Player) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let piece = self.get_piece(row, col);
                if ChessBoard::piece_same_as_player(piece, &player) {
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

    pub fn is_king_in_check(&self, player: Player) -> bool {
        let king_position = self.find_king_position(player.clone());
        // check if any of the other player's pieces are attacking the king
        return self.pieces_attacking_king(king_position, player.clone());
    }

    pub fn get_winner(&self) -> Winner {
        // we only need to check if it's max or not so lets just use the x
        let white_king = self.find_king_position(Player::White).0;
        let black_king = self.find_king_position(Player::Black).0;
        println!("white_king: {}, black_king: {}", white_king, black_king);
        match (white_king, black_king) {
            (usize::MAX, usize::MAX) => Winner::Tie,
            (usize::MAX, _) => Winner::White,
            (_, usize::MAX) => Winner::Black,
            (_, _) => Winner::NoneYet,
        }

    }

    pub fn possible_moves_for_piece(&self, row: usize, col: usize, player: Player) -> Vec<Move> {
        let piece: Piece = self.get_piece(row, col);
        // make sure they are the same type, white or black
        if piece.piece_type.is_white() != (player == Player::White) {
            return vec![];
        }
        let mut moves = piece.piece_type.available_moves(row, col, self);
        for move_obj in &mut moves {
            // update with type interactions
            let other_piece = self.get_piece(move_obj.to_row, move_obj.to_col);
            let type_matchup: InteractionType = PokemonType::type_matchup(piece.pokemon_type, other_piece.pokemon_type);
            move_obj.type_interaction = Some(type_matchup);
            
        }
        return moves;
    }

    // Separate because the piece does not always cover the captured piece
    // specifically for en passant
    pub fn capture_piece(&self, capture: Capture, move_to_execute: Move) -> ChessBoard {
        let mut new_board = self.clone();
        new_board.board[capture.row][capture.col] = Piece::empty();
        return new_board;
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
            self.execute_move(from_row, from_col, to_row, to_col)
        } else {
            self.clone()
        }
    }

    fn is_move_valid(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
        player: Player,
    ) -> bool {
        let possible_moves = self.possible_moves_for_piece(from_row, from_col, player);
        possible_moves.iter().any(|pm| pm.to_row == to_row && pm.to_col == to_col)
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
        possible_moves.into_iter().find(|m| m.to_row == to_row && m.to_col == to_col)
    }

    fn execute_move(&self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> ChessBoard {
        let mut new_board = self.clone();
        let piece = self.get_piece(from_row, from_col);
        let move_to_execute = self.find_move(from_row, from_col, to_row, to_col).unwrap(); // assuming find_move is another method returning Option<Move>

        new_board.handle_captures_and_special_moves(&move_to_execute);
        new_board.board[to_row][to_col] = piece;
        new_board.board[from_row][from_col] = Piece::empty();
        new_board.history.add_move(move_to_execute);

        new_board
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

    pub fn get_piece(&self, row: usize, col: usize) -> Piece {
        return self.board[row][col];
    }

    pub fn initialize_board() -> Self {
        let mut rng = thread_rng();
        let empty_piece = Piece::empty();
        let mut board = [[empty_piece; BOARD_SIZE]; BOARD_SIZE];

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

        ChessBoard {
            board,
            history: ChessHistory::new(),
        }
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

    pub fn get_last_move_interaction_type(&self) -> InteractionType {

        match self.history.move_history.last(){
            Some(this_move) => this_move.type_interaction.unwrap(),
            _ => InteractionType::Normal,
        }

    }
}
