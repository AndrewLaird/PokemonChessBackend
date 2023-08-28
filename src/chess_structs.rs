use serde::Serialize;

pub const BOARD_SIZE: usize = 8;
pub const WHITE_EN_PASSANT_ROW: usize = 4;
pub const BLACK_EN_PASSANT_ROW: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Piece {
    pub piece_type: ChessPieceType,
    pub pokemon_type: PokemonType,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ChessHistory {
    pub move_history: Vec<Move>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ChessBoard {
    pub board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    pub history: ChessHistory,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Move {
    pub piece_type: ChessPieceType,
    pub from_row: usize,
    pub from_col: usize,
    pub to_row: usize,
    pub to_col: usize,
    pub type_interaction: Option<InteractionType>,
    pub capture: Option<Capture>,
    pub castle: Option<Castle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Castle {
    pub rook_from_row: usize,
    pub rook_from_col: usize,
    pub rook_to_row: usize,
    pub rook_to_col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Capture {
    pub row: usize,
    pub col: usize,
    pub piece: Piece
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ChessPieceType {
    Empty,
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum InteractionType {
    SuperEffective,
    NotVeryEffective,
    NoEffect,
    Normal,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}
