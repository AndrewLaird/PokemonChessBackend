use axum::extract::{Json, Query};
use axum::{routing::get, Router};
use log::info;

use serde::{Deserialize, Serialize};

/***
 * Current todo:
 * Undo button
 * Selection of piece for pawn promotion
 */

pub mod chess;
pub mod chess_history;
pub mod chess_structs;
pub mod database;
pub mod pieces;
pub mod pokemon_types;
pub mod pokemon_names;
pub mod name_generator;
pub mod settings;
pub mod messages;
pub mod moves;


use crate::chess_structs::{ChessBoard, ChessState, Move, Player, InfoMessage, Winner};
use crate::database::{load_board, save_board};
use crate::name_generator::generate_game_name;
use crate::settings::Settings;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/start", get(start_game))
        .route("/get_moves", get(get_moves))
        .route("/move_piece", get(move_piece))
        .route("/select_pawn_promotion_piece", get(select_pawn_promotion_piece))
        .route("/chessboard", get(get_board))
        .route("/generate_name", get(get_game_name))
        .route("/get_game_state", get(get_game_state))
        .layer(CorsLayer::permissive());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
pub struct StartGame {
    pub name: String,
    pub simplified_visual: bool,
    pub online_play: bool,
    pub critical_hits: bool,
    pub misses: bool,
}


#[derive(Deserialize)]
pub struct GetGame {
    pub name: String,
}

async fn start_game(Query(params): Query<StartGame>) -> Json<ChessState> {
    let chessboard = ChessBoard::new();
    let player = Player::White;
    let settings = Settings::new();
    let winner = chess::Winner::NoneYet;
    let info_message = None;
    let chess_state = ChessState { chessboard, settings, player, winner, info_message, require_piece_selection:false};
    let result = save_board(params.name, chess_state.clone()).await;
    match result {
        Ok(_) => info!("saved board"),
        Err(_) => info!("failed to save board"),
    }
    return Json(chess_state);
}

async fn get_game_state(Query(params): Query<GetGame>) -> Json<Option<ChessState>> {
    let chess_state = load_board(&params.name).await;
    return match chess_state{
        Ok(state) => Json(Some(state)),
        _ => Json(None)
    }
}

// The query parameters for todos index
#[derive(Deserialize)]
pub struct GetMoves {
    pub name: String,
    pub row: usize,
    pub col: usize,
}

#[derive(Deserialize, Serialize)]
pub struct SelectPawnPromotionPiece {
    pub name: String,
    pub piece_str: String,
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

async fn get_valid_moves(chess_state: ChessState, chess_board: ChessBoard, params: GetMoves) -> Vec<Move> {
    let mut moves = chess_board.possible_moves_for_piece(params.row, params.col, chess_state.player);
    let current_player = chess_state.player.clone();
    let mut valid_moves = Vec::new();
    // check last moveh for super effective moves and if so, only return those that match the
    // location of the piece that last moved
    if let Some(position) = chess_board.history.last_move_super_effective() {
        moves = ChessBoard::filter_moves_if_supereffective(moves, position);
    }
    for m in moves {
        let mut new_board = chess_board.clone();
        new_board = new_board.move_piece(
            m.from_row,
            m.from_col,
            m.to_row,
            m.to_col,
            chess_state.player,
        );
        if !new_board.is_king_in_check(current_player) {
            new_board.display_board();
            valid_moves.push(m);
        }
    }
    valid_moves
}

async fn get_moves(Query(params): Query<GetMoves>) -> Json<Vec<Move>> {
    let chess_state = load_board(&params.name).await.unwrap();
    if chess_state.winner != Winner::NoneYet {
        return Json(vec![]);
    }
    let chess_board = chess_state.chessboard.clone();
    let valid_moves = get_valid_moves(chess_state, chess_board, params).await;
    return Json(valid_moves)
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
    // last move Interaction type
    let last_move_interaction_type = chessboard.get_last_move_interaction_type();
    chess_state.chessboard = chessboard;
    chess_state.info_message = InfoMessage::get_message_from_interaction_type(last_move_interaction_type);
    chess_state.winner = chess_state.chessboard.get_winner();
    let pawn_promotion = chess_state.chessboard.history.last_move_requires_pawn_promotion();
    chess_state.require_piece_selection = pawn_promotion;
    if !pawn_promotion {
        chess_state.player = chess_state.player.other_player_with_type_interaction(last_move_interaction_type);
    }
    save_board(name, chess_state.clone()).await.unwrap();
    return Json(chess_state);
}

async fn select_pawn_promotion_piece(Query(params): Query<SelectPawnPromotionPiece>) -> Json<ChessState> {
    let name = params.name.clone();
    let original_chess_state = load_board(&name).await.unwrap();
    let mut chess_state: ChessState = original_chess_state.clone();
    let result = chess_state.chessboard.select_pawn_promotion_piece(params.piece_str.clone(), chess_state.player.clone());
    if result.is_err(){
        return Json(original_chess_state);
    }
    chess_state.require_piece_selection = false;
    let last_move_interaction_type = chess_state.chessboard.get_last_move_interaction_type();
    chess_state.player = chess_state.player.other_player_with_type_interaction(last_move_interaction_type);
    save_board(name, chess_state.clone()).await.unwrap();
    return Json(chess_state);
}

async fn get_board() -> Json<ChessBoard> {
    let chessboard = ChessBoard::new();
    return Json(chessboard);
}

async fn get_game_name() -> Json<String> {
    let name = generate_game_name().await.unwrap();
    return Json(name);
}
