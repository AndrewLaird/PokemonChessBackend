use axum::extract::{Json, Query};
use axum::{routing::get, Router};
use log::info;

use serde::{Deserialize, Serialize};

pub mod chess;
pub mod chess_history;
pub mod chess_structs;
pub mod database;
pub mod pieces;
pub mod pokemon_types;

use crate::chess_structs::{ChessBoard, ChessState, Move, Player, Position};
use crate::database::{load_board, save_board};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();
    info!("Heyo");

    let app = Router::new()
        .route("/", get(root))
        .route("/start", get(start_game))
        .route("/get_moves", get(get_moves))
        .route("/move_piece", get(move_piece))
        .route("/chessboard", get(get_board))
        .layer(CorsLayer::permissive());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// The query parameters for todos index
#[derive(Deserialize)]
pub struct StartGame {
    pub name: String,
}

async fn start_game(Query(params): Query<StartGame>) -> Json<ChessState> {
    let chessboard = ChessBoard::new();
    let player = Player::White;
    let chess_state = ChessState { chessboard, player };
    let result = save_board(params.name, chess_state.clone()).await;
    match result {
        Ok(_) => info!("saved board"),
        Err(_) => info!("failed to save board"),
    }
    return Json(chess_state);
}

// The query parameters for todos index
#[derive(Deserialize)]
pub struct GetMoves {
    pub name: String,
    pub row: usize,
    pub col: usize,
}

// List of valid moves for a piece
async fn valid_moves(Query(params): Query<GetMoves>) -> Json<Vec<Position>> {
    let board_state = load_board(&params.name).await.unwrap();
    let board = board_state.chessboard;
    let row = params.row;
    let col = params.col;
    let piece = board.board[row][col];
    let valid_positions = piece.piece_type.available_moves(row, col, &board);
    let valid_moves = valid_positions
        .iter()
        .map(|position| Position {
            row: position.to_row,
            col: position.to_col,
        })
        .collect::<Vec<Position>>();
    return Json(valid_moves);
}

async fn root() -> &'static str {
    return "hello world";
}

// The query parameters for todos index
#[derive(Deserialize, Debug)]
pub struct UserMove {
    pub name: String,
    pub from_row: usize,
    pub from_col: usize,
    pub to_row: usize,
    pub to_col: usize,
}

#[derive(Serialize)]
pub struct SerializeObject {
    pub result: Option<usize>,
}

async fn get_moves(Query(params): Query<GetMoves>) -> Json<Vec<Move>> {
    let chess_state = load_board(&params.name).await.unwrap();
    let chess_board = chess_state.chessboard;
    let moves = chess_board.possible_moves_for_piece(params.row, params.col, chess_state.player);
    // if the person is in check, only allow moves that get them out of check
    let mut valid_moves = Vec::new();
    for m in moves {
        let mut new_board = chess_board.clone();
        new_board = new_board.move_piece(
            m.from_row,
            m.from_col,
            m.to_row,
            m.to_col,
            chess_state.player,
        );
        if !new_board.is_king_in_check(chess_state.player) {
            valid_moves.push(m);
        }
    }
    return Json(valid_moves);
}

#[derive(Serialize)]
pub struct MoveResponse {
    pub is_valid: bool,
    pub chess_state: ChessState,
}

async fn move_piece(Query(params): Query<UserMove>) -> Json<ChessState> {
    let name = params.name.clone();
    let mut chess_state: ChessState = load_board(&name).await.unwrap();
    let mut chessboard = chess_state.chessboard.clone();
    chessboard = chessboard.move_piece(
        params.from_row,
        params.from_col,
        params.to_row,
        params.to_col,
        chess_state.player.clone(),
    );
    info!("{:?}", chessboard.display_board_str());
    chess_state.chessboard = chessboard;
    chess_state.player = chess_state.player.other_player();
    save_board(name, chess_state.clone()).await.unwrap();
    return Json(chess_state);
}

async fn get_board() -> Json<ChessBoard> {
    let chessboard = ChessBoard::new();
    return Json(chessboard);
}
