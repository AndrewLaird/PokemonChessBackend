use crate::chess_structs::{
    Capture, Castle, ChessBoard, ChessPieceType, Move, Piece, PokemonType, BLACK_EN_PASSANT_ROW,
    BOARD_SIZE, WHITE_EN_PASSANT_ROW,
};

impl Piece {
    pub fn empty() -> Self {
        Self {
            piece_type: ChessPieceType::Empty,
            pokemon_type: PokemonType::Normal,
        }
    }
}

impl ChessPieceType {
    pub fn is_valid_position(to_row: i32, to_col: i32) -> bool {
        let row_valid: bool = to_row >= 0 && to_row < BOARD_SIZE as i32;
        let col_valid: bool = to_col >= 0 && to_col < BOARD_SIZE as i32;
        return row_valid && col_valid;
    }
    pub fn available_moves(&self, row: usize, col: usize, board: &ChessBoard) -> Vec<Move> {
        match self {
            ChessPieceType::WhitePawn | ChessPieceType::BlackPawn => {
                return self.pawn_moves(row, col, &board);
            }
            ChessPieceType::WhiteKnight | ChessPieceType::BlackKnight => {
                return self.knight_moves(row, col, board);
            }
            ChessPieceType::WhiteBishop | ChessPieceType::BlackBishop => {
                return self.bishop_moves(row, col, board);
            }
            ChessPieceType::WhiteRook | ChessPieceType::BlackRook => {
                return self.rook_moves(row, col, board);
            }
            ChessPieceType::WhiteQueen | ChessPieceType::BlackQueen => {
                return self.queen_moves(row, col, board);
            }
            ChessPieceType::WhiteKing | ChessPieceType::BlackKing => {
                return self.king_moves(row, col, board);
            }
            _ => {
                return vec![];
            }
        }
    }

    pub fn is_white(&self) -> bool {
        match self {
            ChessPieceType::WhitePawn
            | ChessPieceType::WhiteKnight
            | ChessPieceType::WhiteBishop
            | ChessPieceType::WhiteRook
            | ChessPieceType::WhiteQueen
            | ChessPieceType::WhiteKing => true,
            _ => false,
        }
    }
    pub fn is_opposite_color(&self, other: ChessPieceType) -> bool {
        return self.is_white() != other.is_white();
    }

    pub fn pawn_moves(&self, row: usize, col: usize, board: &ChessBoard) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        // assert it's a pawn
        assert!(self == &ChessPieceType::WhitePawn || self == &ChessPieceType::BlackPawn);
        // Check if pawn is white or black
        let is_white = match self {
            ChessPieceType::WhitePawn => true,
            ChessPieceType::BlackPawn => false,
            _ => panic!("Not a pawn"),
        };

        // Check if pawn is in starting position
        let is_starting_position = (is_white && row == 1) || (!is_white && row == 6);
        let direction: i32 = if is_white { 1 } else { -1 };

        // Check if pawn can move forward
        let to_row: i32 = (row as i32) + direction;
        if to_row < 0 || to_row > 7 {
            // can't move here
        }
        if Self::is_valid_position(to_row, col as i32)
            && board.get_piece(to_row as usize, col).piece_type == ChessPieceType::Empty
        {
            // Pawn can move forward
            moves.push(Move {
                piece_type: *self,
                from_row: row,
                from_col: col,
                to_row: to_row as usize,
                to_col: col,
                type_interaction: None,
                capture: None,
                castle: None,
            });
        }

        // Check if pawn can capture diagonally
        let to_row_attack = (row as i32) + direction;
        let to_col_attack_left = (col as i32) - 1;
        let to_col_attack_right = (col as i32) + 1;

        if Self::is_valid_position(to_row_attack, to_col_attack_left) {
            let piece = board
                .get_piece(to_row_attack as usize, to_col_attack_left as usize)
                .piece_type;
            if piece.is_opposite_color(*self) {
                let captured_piece =
                    board.get_piece(to_row_attack as usize, to_col_attack_left as usize);
                moves.push(Move {
                    piece_type: *self,
                    from_row: row,
                    from_col: col,
                    to_row: to_row_attack as usize,
                    to_col: to_col_attack_left as usize,
                    type_interaction: None,
                    capture: Some(Capture {
                        row: to_row_attack as usize,
                        col: to_col_attack_left as usize,
                        piece: captured_piece,
                    }),
                    castle: None,
                });
            }
        }

        if Self::is_valid_position(to_row_attack, to_col_attack_right) {
            let piece = board
                .get_piece(to_row_attack as usize, to_col_attack_left as usize)
                .piece_type;
            if piece.is_opposite_color(*self) {
                let captured_piece =
                    board.get_piece(to_row_attack as usize, to_col_attack_left as usize);
                moves.push(Move {
                    piece_type: *self,
                    from_row: row,
                    from_col: col,
                    to_row: to_row_attack as usize,
                    to_col: to_col_attack_left as usize,
                    type_interaction: None,
                    capture: Some(Capture {
                        row: to_row_attack as usize,
                        col: to_col_attack_left as usize,
                        piece: captured_piece,
                    }),
                    castle: None,
                });
            }
        }

        // check for en passant
        // if there is a pawn to the left or right of the pawn,
        // check the history of the last move to see if it was a pawn moving forward twice
        // if it was, then the pawn can move diagonally and capture the pawn that moved forward twice
        moves.extend(self.en_passant_move(row, col, board));

        // Check if pawn can move forward twice
        if !is_starting_position {
            // can't move forward twice
            return moves;
        }

        let to_row_double_move = (row as i32) + 2 * direction;
        if Self::is_valid_position(to_row_double_move, col as i32)
            && board.get_piece(to_row as usize, col).piece_type == ChessPieceType::Empty
            && board.get_piece(to_row_double_move as usize, col).piece_type == ChessPieceType::Empty
        {
            // Pawn can move forward
            moves.push(Move {
                piece_type: *self,
                from_row: row,
                from_col: col,
                to_row: to_row_double_move as usize,
                to_col: col,
                type_interaction: None,
                capture: None,
                castle: None,
            });
        }

        return moves;
    }

    pub fn en_passant_move(&self, row: usize, col: usize, board: &ChessBoard) -> Vec<Move> {
        // Ok so the pawn to the left or the right have to be the oposite team
        // and it must have just moved twice last turn
        // and it must be in the correct row
        let mut moves: Vec<Move> = vec![];

        if row == WHITE_EN_PASSANT_ROW && self == &ChessPieceType::WhitePawn {
            // check if the pawn to the left or right of the pawn is black
            let left_piece: Option<ChessPieceType> =
                ChessPieceType::get_valid_or_empty(row as i32, (col - 1) as i32, board);
            match left_piece {
                Some(Self::BlackPawn) => {
                    moves.extend(ChessPieceType::get_valid_en_passant(
                        row,
                        col,
                        row,
                        col - 1,
                        1,
                        board,
                    ));
                }
                _ => {}
            }
            let right_piece: Option<ChessPieceType> =
                ChessPieceType::get_valid_or_empty(row as i32, (col + 1) as i32, board);
            match right_piece {
                Some(Self::BlackPawn) => {
                    moves.extend(ChessPieceType::get_valid_en_passant(
                        row,
                        col,
                        row,
                        col + 1,
                        1,
                        board,
                    ));
                }
                _ => {}
            }
        }

        if row == BLACK_EN_PASSANT_ROW && self == &ChessPieceType::BlackPawn {
            // check if the pawn to the left or right of the pawn is White
            let left_piece: Option<ChessPieceType> =
                ChessPieceType::get_valid_or_empty(row as i32, (col - 1) as i32, board);
            match left_piece {
                Some(Self::WhitePawn) => {
                    moves.extend(ChessPieceType::get_valid_en_passant(
                        row,
                        col,
                        row,
                        col - 1,
                        -1,
                        board,
                    ));
                }
                _ => {}
            }
            let right_piece: Option<ChessPieceType> =
                ChessPieceType::get_valid_or_empty(row as i32, (col + 1) as i32, board);
            match right_piece {
                Some(Self::WhitePawn) => {
                    moves.extend(ChessPieceType::get_valid_en_passant(
                        row,
                        col,
                        row,
                        col + 1,
                        -1,
                        board,
                    ));
                }
                _ => {}
            }
        }
        return moves;
    }

    pub fn get_valid_en_passant(
        row: usize,
        col: usize,
        pawn_row: usize,
        pawn_col: usize,
        direction: i32,
        board: &ChessBoard,
    ) -> Vec<Move> {
        // Simply check if this pawn moved twice last turn
        // and construct the move
        if board.history.pawn_moved_last_turn(pawn_row, pawn_col) {
            return vec![Move {
                piece_type: ChessPieceType::WhitePawn,
                from_row: row,
                from_col: col,
                to_row: row + direction as usize,
                to_col: pawn_col,
                type_interaction: None,
                capture: Some(Capture {
                    row: pawn_row,
                    col: pawn_col,
                    piece: board.get_piece(pawn_row, pawn_col),
                }),
                castle: None,
            }];
        } else {
            return vec![];
        }
    }

    pub fn get_valid_or_empty(row: i32, col: i32, board: &ChessBoard) -> Option<ChessPieceType> {
        if Self::is_valid_position(row, col) {
            return Some(board.get_piece(row as usize, col as usize).piece_type);
        }
        return None;
    }

    pub fn knight_moves(&self, row: usize, col: usize, board: &ChessBoard) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        // The knight moves in an "L" shape: two squares in one direction
        // and then one square perpendicular to the first direction
        let row_deltas = [-2, -1, 1, 2, 2, 1, -1, -2];
        let col_deltas = [1, 2, 2, 1, -1, -2, -2, -1];

        for i in 0..8 {
            let to_row = row as i32 + row_deltas[i];
            let to_col = col as i32 + col_deltas[i];

            if Self::is_valid_position(to_row, to_col) {
                let target_piece = board.get_piece(to_row as usize, to_col as usize).piece_type;

                // Check if the target square is empty or contains an enemy piece
                if target_piece == ChessPieceType::Empty || target_piece.is_opposite_color(*self) {
                    moves.push(Move {
                        piece_type: *self,
                        from_row: row,
                        from_col: col,
                        to_row: to_row as usize,
                        to_col: to_col as usize,
                        type_interaction: None,
                        capture: if target_piece == ChessPieceType::Empty {
                            None
                        } else {
                            Some(Capture {
                                row: to_row as usize,
                                col: to_col as usize,
                                piece: board.get_piece(to_row as usize, to_col as usize),
                            })
                        },
                        castle: None,
                    });
                }
            }
        }

        moves
    }

    pub fn rook_moves(&self, row: usize, col: usize, board: &ChessBoard) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        // Check moves along each direction: up, down, left, right
        let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (dx, dy) in directions {
            let mut new_row = row as i32;
            let mut new_col = col as i32;

            loop {
                new_row += dx;
                new_col += dy;

                // Break if out of bounds
                if !ChessPieceType::is_valid_position(new_row, new_col) {
                    break;
                }

                // Get the piece at the new position
                let target_square = board.get_piece(new_row as usize, new_col as usize);

                match target_square.piece_type {
                    // Empty square, so the rook can move here
                    ChessPieceType::Empty => {
                        moves.push(Move {
                            piece_type: *self,
                            from_row: row,
                            from_col: col,
                            to_row: new_row as usize,
                            to_col: new_col as usize,
                            type_interaction: None,
                            capture: None,
                            castle: None,
                        });
                    }
                    // Square occupied by another piece
                    _ => {
                        // Check if the piece is an opponent's piece
                        if self.is_opposite_color(target_square.piece_type) {
                            moves.push(Move {
                                piece_type: *self,
                                from_row: row,
                                from_col: col,
                                to_row: new_row as usize,
                                to_col: new_col as usize,
                                type_interaction: None,
                                capture: Some(Capture {
                                    row: new_row as usize,
                                    col: new_row as usize,
                                    piece: board.get_piece(new_row as usize, new_row as usize),
                                }),
                                castle: None,
                            })
                        }
                        // Stop either way, can't jump over pieces
                        break;
                    }
                }
            }
        }

        moves
    }

    pub fn bishop_moves(&self, row: usize, col: usize, board: &ChessBoard) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        // Diagonal directions: (row_change, col_change)
        let directions: Vec<(i32, i32)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];

        for (dx, dy) in directions {
            let mut new_row = row as i32;
            let mut new_col = col as i32;

            loop {
                new_row += dx;
                new_col += dy;

                // Break if out of bounds
                if !ChessPieceType::is_valid_position(new_row, new_col) {
                    break;
                }

                // Get the piece at the new position
                let target_square = board.get_piece(new_row as usize, new_col as usize);

                match target_square.piece_type {
                    // Empty square, so the bishop can move here
                    ChessPieceType::Empty => {
                        moves.push(Move {
                            piece_type: *self,
                            from_row: row,
                            from_col: col,
                            to_row: new_row as usize,
                            to_col: new_col as usize,
                            type_interaction: None,
                            capture: None,
                            castle: None,
                        });
                    }
                    // Square occupied by another piece
                    _ => {
                        // Check if the piece is an opponent's piece
                        if self.is_opposite_color(target_square.piece_type) {
                            moves.push(Move {
                                piece_type: *self,
                                from_row: row,
                                from_col: col,
                                to_row: new_row as usize,
                                to_col: new_col as usize,
                                type_interaction: None,
                                capture: Some(Capture {
                                    row: new_row as usize,
                                    col: new_col as usize,
                                    piece: target_square.clone(),
                                }),
                                castle: None,
                            });
                        }
                        break; // Bishop's path is blocked
                    }
                }
            }
        }

        moves
    }

    pub fn queen_moves(&self, row: usize, col: usize, board: &ChessBoard) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        // Queen's movement is a combination of rook and bishop moves
        moves.extend(self.rook_moves(row, col, board));
        moves.extend(self.bishop_moves(row, col, board));

        moves
    }

    pub fn king_moves(&self, row: usize, col: usize, board: &ChessBoard) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        // The king can move one square in any direction:
        // horizontally, vertically, or diagonally.
        let row_deltas = [-1, -1, -1, 0, 1, 1, 1, 0];
        let col_deltas = [-1, 0, 1, 1, 1, 0, -1, -1];

        for i in 0..8 {
            let to_row = row as i32 + row_deltas[i];
            let to_col = col as i32 + col_deltas[i];

            if Self::is_valid_position(to_row, to_col) {
                let target_piece = board.get_piece(to_row as usize, to_col as usize).piece_type;

                // Check if the target square is empty or contains an enemy piece
                if target_piece == ChessPieceType::Empty || self.is_opposite_color(target_piece) {
                    moves.push(Move {
                        piece_type: *self,
                        from_row: row,
                        from_col: col,
                        to_row: to_row as usize,
                        to_col: to_col as usize,
                        type_interaction: None,
                        capture: if target_piece == ChessPieceType::Empty {
                            None
                        } else {
                            Some(Capture {
                                row: to_row as usize,
                                col: to_col as usize,
                                piece: board.get_piece(to_row as usize, to_col as usize),
                            })
                        },
                        castle: None,
                    });
                }
            }
        }

        // check for castling
        if board.history.can_castle_kingside(self.is_white()) {
            // check if the squares between the king and rook are empty
            let mut empty = true;
            for col in 5..7 {
                if board.get_piece(row, col).piece_type != ChessPieceType::Empty {
                    empty = false;
                    break;
                }
            }
            if empty {
                moves.push(Move {
                    piece_type: *self,
                    from_row: row,
                    from_col: col,
                    to_row: row,
                    to_col: col + 2,
                    type_interaction: None,
                    capture: None,
                    castle: Some(Castle {
                        rook_from_row: row,
                        rook_from_col: 7,
                        rook_to_row: row,
                        rook_to_col: 5,
                    }),
                });
            }
        }
        if board.history.can_castle_queenside(self.is_white()) {
            // check if the squares between the king and rook are empty
            let mut empty = true;
            for col in 1..4 {
                if board.get_piece(row, col).piece_type != ChessPieceType::Empty {
                    empty = false;
                    break;
                }
            }
            if empty {
                moves.push(Move {
                    piece_type: *self,
                    from_row: row,
                    from_col: col,
                    to_row: row,
                    to_col: col - 2,
                    type_interaction: None,
                    capture: None,
                    castle: Some(Castle {
                        rook_from_row: row,
                        rook_from_col: 0,
                        rook_to_row: row,
                        rook_to_col: 3,
                    }),
                });
            }
        }
        moves
    }
}
