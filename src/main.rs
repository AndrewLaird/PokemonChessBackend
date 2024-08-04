use axum::extract::{Json, Query};
use axum::{routing::get, Router};
use log::{error, info};

use serde::{Deserialize, Serialize};

/***
 * Current todo:
 * Undo button
 */

pub mod chess;
pub mod chess_state;
pub mod chess_history;
pub mod chess_structs;
pub mod database;
pub mod messages;
pub mod moves;
pub mod name_generator;
pub mod pieces;
pub mod pokemon_names;
pub mod pokemon_types;
pub mod settings;

use crate::chess_structs::{ChessState, Move, Winner};
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
        // can be kept as static
        .route("/start", get(start_game))
        .route("/generate_name", get(get_game_name))
        .route("/get_game_state", get(get_game_state))
        // should be made into websocket connections
        .route("/get_moves", get(get_moves))
        .route("/move_piece", get(move_piece))
        .route(
            "/select_pawn_promotion_piece",
            get(select_pawn_promotion_piece),
        )
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
    let settings = Settings::new(
        params.local_play,
        params.critical_hits,
        params.misses,
    );
    let chess_state = ChessState::new(settings.clone());

    // Save the board
    match save_board(params.name.clone(), chess_state.clone()).await {
        Ok(_) => info!("Saved board"),
        Err(e) => {
            error!("Failed to save board: {:?}", e);
        }
    }

    Json(chess_state)
}

async fn get_game_state(Query(params): Query<GetGame>) -> Json<Option<ChessState>> {
    let chess_state = load_board(&params.name).await;
    return match chess_state {
        Ok(state) => Json(Some(state)),
        _ => Json(None),
    };
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
    let chess_state = load_board(&params.name).await.unwrap();
    if chess_state.winner != Winner::NoneYet {
        return Json(vec![]);
    }
    let valid_moves = chess_state.get_valid_moves(params.row, params.col);
    return Json(valid_moves);
}



async fn move_piece(Query(params): Query<UserMove>) -> Json<ChessState> {
    let name = params.name.clone();
    let mut chess_state: ChessState = load_board(&name).await.unwrap();
    let board_changed = chess_state.move_piece(params.from_row, params.from_col, params.to_row, params.to_col);
    if board_changed {
        save_board(name, chess_state.clone()).await.unwrap();
    }
    return Json(chess_state);
}

async fn select_pawn_promotion_piece(
    Query(params): Query<SelectPawnPromotionPiece>,
) -> Json<ChessState> {
    let name = params.name.clone();
    let original_chess_state = load_board(&name).await.unwrap();
    let mut chess_state: ChessState = original_chess_state.clone();
    let result = chess_state
        .chessboard
        .select_pawn_promotion_piece(params.piece_str.clone(), chess_state.player.clone());
    if result.is_err() {
        return Json(original_chess_state);
    }
    chess_state.require_piece_selection = false;
    chess_state.player = chess_state
        .player
        .other_player_considering_board(&chess_state.chessboard);
    save_board(name, chess_state.clone()).await.unwrap();
    return Json(chess_state);
}

async fn get_game_name() -> Json<String> {
    let name = generate_game_name().await.unwrap();
    return Json(name);
}
