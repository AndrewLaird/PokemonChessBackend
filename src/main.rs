use axum::extract::{Json, Query};
use axum::{routing::get, Router};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub mod chess;
pub mod chess_history;
pub mod chess_structs;
pub mod pieces;
pub mod pokemon_types;

use crate::chess_structs::ChessBoard;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/move", get(make_move))
        .route("/move_piece", get(move_piece))
        .route("/chessboard", get(get_board));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn play_game() {
    let mut chess_board = chess::ChessBoard::new();
    // move the white piece forward 2
    chess_board = chess_board.move_piece(1, 3, 3, 3);
    chess_board.display_board();
    // move the black piece to be captured
    chess_board = chess_board.move_piece(6, 4, 4, 4);
    chess_board.display_board();
    // capture the black piece
    chess_board = chess_board.move_piece(3, 3, 4, 4);
    chess_board.display_board();
    // black double moves
    chess_board = chess_board.move_piece(6, 5, 4, 5);
    chess_board.display_board();
    // white en passant
    chess_board = chess_board.move_piece(4, 4, 5, 5);
    chess_board.display_board();
    // black knight move
    chess_board = chess_board.move_piece(7,1, 5, 2);
    chess_board.display_board();
    // white bishop move
    chess_board = chess_board.move_piece(0,2, 2, 4);
    chess_board.display_board();
    // black rook moves 
    chess_board = chess_board.move_piece(7,0, 7, 1);
    chess_board.display_board();
    // white queen moves
    chess_board = chess_board.move_piece(0,3, 6, 3);
    chess_board.display_board();
    // black king captures white queen
    chess_board = chess_board.move_piece(7,4, 6, 3);
    chess_board.display_board();
    // white piece moves to allow castle
    chess_board = chess_board.move_piece(0,1, 2, 2);
    chess_board.display_board();
    // black piece skips turn
    chess_board = chess_board.move_piece(6,7, 5, 7);
    chess_board.display_board();
    // white queen side castle
    chess_board = chess_board.move_piece(0,4, 0, 2);
    chess_board.display_board();
}

async fn root() -> &'static str {
    return "hello world";
}

async fn make_move() -> Json<Value> {
    return Json(json!({"data":"hi"}));
}

// The query parameters for todos index
#[derive(Deserialize)]
pub struct UserMove {
    pub from_row: usize,
    pub from_col: usize,
    pub to_row: usize,
    pub to_col: usize,
}

#[derive(Serialize)]
pub struct SerializeObject {
    pub result: Option<usize>,
}

async fn move_piece(Query(params): Query<UserMove>) -> Json<ChessBoard> {
    let chessboard = ChessBoard::new();
    chessboard.move_piece(params.from_row, params.from_col, params.to_row, params.to_col);
    return Json(chessboard);
}

async fn get_board() -> Json<ChessBoard> {
    let chessboard = ChessBoard::new();
    return Json(chessboard);
}

// Real functions for chess
