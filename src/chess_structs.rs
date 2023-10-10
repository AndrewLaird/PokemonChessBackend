use serde::{Serialize, Deserialize};

pub const BOARD_SIZE: usize = 8;
pub const WHITE_EN_PASSANT_ROW: usize = 4;
pub const BLACK_EN_PASSANT_ROW: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: ChessPieceType,
    pub pokemon_type: PokemonType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChessHistory {
    pub move_history: Vec<Move>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChessBoard {
    pub board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    pub history: ChessHistory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChessState {
    pub chessboard: ChessBoard,
    pub player: Player,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Player{
    White,
    Black,
}

impl Player {
    pub fn other_player(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Castle {
    pub rook_from_row: usize,
    pub rook_from_col: usize,
    pub rook_to_row: usize,
    pub rook_to_col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Capture {
    pub row: usize,
    pub col: usize,
    pub piece: Piece
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InteractionType {
    SuperEffective,
    NotVeryEffective,
    NoEffect,
    Normal,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
