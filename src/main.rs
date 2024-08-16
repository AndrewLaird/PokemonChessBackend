use axum::{
    extract::{Json, Query},
    routing::get,
    Router,
};

use log::info;

use serde::{Deserialize, Serialize};

/***
 * Current todo:
 * Undo button
 */

pub mod websockets;
pub mod chess;
pub mod chess_history;
pub mod chess_state;
pub mod chess_state_history;
pub mod chess_structs;
pub mod database;
pub mod game;
pub mod messages;
pub mod moves;
pub mod name_generator;
pub mod pieces;
pub mod pokemon_names;
pub mod pokemon_types;
pub mod settings;
pub mod app_state;

use crate::chess_state_history::ChessStateHistory;
use crate::chess_structs::{ChessState, Move, Winner};
use crate::game::Game;
use crate::name_generator::generate_game_name;
use crate::settings::Settings;
use tower_http::cors::CorsLayer;
use crate::websockets::handler;
use crate::app_state::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;


#[tokio::main]
async fn main() {
    let app_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new()));
    // Initialize the logger
    env_logger::init();
    let app = Router::new()
        .route("/", get(root))
        // can be kept as static
        .route("/start", get(start_game))
        .route("/generate_name", get(get_game_name))
        .route("/get_game_state", get(get_game_state))
        // should be made into websocket connections
        .route("/ws", get(handler))
        .with_state(app_state)
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
    pub local_play: bool,
    pub critical_hits: bool,
    pub misses: bool,
}

#[derive(Deserialize)]
pub struct GetGame {
    pub name: String,
}

async fn start_game(Query(params): Query<StartGame>) -> Json<ChessState> {
    let settings = Settings::new(params.local_play, params.critical_hits, params.misses);
    let chess_state = ChessState::new();
    let chess_state_history = ChessStateHistory::new_with_initial_state(chess_state.clone());
    let name = params.name.clone();
    let game = Game::new(name, settings, chess_state_history);

    // Save the board
    if game.save().await {
        info!("Saved board");
    }

    Json(chess_state)
}

async fn get_game_state(Query(params): Query<GetGame>) -> Json<Option<ChessState>> {
    let game = Game::load(&params.name).await;
    let chess_state = game.get_current_state();
    return Json(chess_state);
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

#[derive(Serialize)]
pub struct MoveResponse {
    pub is_valid: bool,
    pub chess_state: ChessState,
}

async fn get_moves(Query(params): Query<GetMoves>) -> Json<Vec<Move>> {
    let game = Game::load(&params.name).await;
    let chess_state = game.get_current_state().unwrap();
    if chess_state.winner != Winner::NoneYet {
        return Json(vec![]);
    }
    let valid_moves = chess_state.get_valid_moves(params.row, params.col);
    return Json(valid_moves);
}

async fn move_piece(Query(params): Query<UserMove>) -> Json<ChessState> {
    let name = params.name.clone();
    let mut game = Game::load(&name).await;
    let board_changed = game.move_piece(
        params.from_row,
        params.from_col,
        params.to_row,
        params.to_col,
    );
    if board_changed {
        game.save().await;
    }
    return Json(game.get_current_state().unwrap());
}

async fn select_pawn_promotion_piece(
    Query(params): Query<SelectPawnPromotionPiece>,
) -> Json<ChessState> {
    let name = params.name.clone();
    let mut game = Game::load(&name).await;
    game.select_pawn_promotion_piece(params.piece_str).unwrap();
    game.save().await;
    return Json(game.get_current_state().unwrap());
}

async fn get_game_name() -> Json<String> {
    let name = generate_game_name().await.unwrap();
    return Json(name);
}

async fn get_previous_state(Query(params): Query<GetGame>) -> Json<ChessState> {
    let mut game = Game::load(&params.name).await;
    if !game.get_previous_state().is_none() {
        game.save().await;
    }
    return Json(game.get_current_state().unwrap());
}

async fn get_next_state(Query(params): Query<GetGame>) -> Json<ChessState> {
    let mut game = Game::load(&params.name).await;
    if !game.get_next_state().is_none() {
        game.save().await;
    }
    return Json(game.get_current_state().unwrap());
}
