use crate::chess_structs::{
    Move, ChessPieceType, Capture, PokemonType, Piece
};

impl Move {
    // Create a new en passant move. This function assumes that the move is legal
    // and that the calling function has already verified the move's legality.
    pub fn new_en_passant(
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
        captured_pawn_row: usize,
        captured_pawn_col: usize,
        piece_type: ChessPieceType,
    ) -> Self {
        Move {
            piece_type,
            from_row,
            from_col,
            to_row,
            to_col,
            type_interaction: None, // Set this based on game logic if needed
            capture: Some(Capture {
                row: captured_pawn_row,
                col: captured_pawn_col,
                piece: Piece {
                    piece_type: if piece_type == ChessPieceType::WhitePawn {
                        ChessPieceType::BlackPawn
                    } else {
                        ChessPieceType::WhitePawn
                    },
                    pokemon_type: PokemonType::NoType, // Replace with actual PokemonType if needed
                },
            }),
            castle: None,
        }
    }
}
