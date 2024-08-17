use axum::{
    extract::{Json, Query},
    routing::get,
    Router,
    Server,
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
use tower_http::cors::{CorsLayer, Any};
use crate::websockets::handler;
use crate::app_state::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::http::Method;


use axum::Extension;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();
    println!("Starting server...");

    let app_state = Arc::new(Mutex::new(AppState::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/start", get(start_game))
        .route("/generate_name", get(get_game_name))
        .route("/get_game_state", get(get_game_state))
        .route("/ws", get(handler))
        .layer(Extension(app_state))
        .layer(cors);

    println!("Server starting on 0.0.0.0:3000");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
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

async fn get_game_name() -> Json<String> {
    let name = generate_game_name().await.unwrap();
    return Json(name);
}
