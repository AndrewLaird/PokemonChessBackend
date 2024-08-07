use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use serde::{Deserialize, Serialize};
use crate::game::Game;
use crate::chess_structs::{ChessState, Move};
use crate::app_state::AppState;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(tag = "action", content = "payload")]
enum ClientMessage {
    GetMoves(GetMovesPayload),
    MovePiece(MovePiecePayload),
    SelectPawnPromotionPiece(SelectPawnPromotionPiecePayload),
    GetPreviousState(GetGamePayload),
    GetNextState(GetGamePayload),
    GetCurrentState(GetGamePayload),
}

#[derive(Deserialize)]
struct GetMovesPayload {
    name: String,
    row: usize,
    col: usize,
}

#[derive(Deserialize)]
struct MovePiecePayload {
    name: String,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
}

#[derive(Deserialize)]
struct SelectPawnPromotionPiecePayload {
    name: String,
    piece_str: String,
}

#[derive(Deserialize)]
struct GetGamePayload {
    name: String,
}

#[derive(Serialize)]
#[serde(tag = "status", content = "data")]
enum ServerMessage {
    Success(ServerMessageData),
    Error { message: String },
}

#[derive(Serialize)]
#[serde(untagged)]
enum ServerMessageData {
    Moves { moves: Vec<Move> },
    ChessState { chess_state: ChessState },
}

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

pub async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };
        if let Message::Text(text) = msg {
            let response = handle_message(text).await;
            // send the response to everyone in the room
            // need state to do that
            if socket.send(Message::Text(serde_json::to_string(&response).unwrap())).await.is_err() {
                // client disconnected
                return;
            }
        }
    }
}

async fn handle_message(message: String) -> ServerMessage {
    match serde_json::from_str::<ClientMessage>(&message) {
        Ok(msg) => match msg {
            ClientMessage::GetMoves(payload) => get_moves(payload).await,
            ClientMessage::MovePiece(payload) => move_piece(payload).await,
            ClientMessage::SelectPawnPromotionPiece(payload) => select_pawn_promotion_piece(payload).await,
            ClientMessage::GetPreviousState(payload) => get_previous_state(payload).await,
            ClientMessage::GetNextState(payload) => get_next_state(payload).await,
            ClientMessage::GetCurrentState(payload) => get_current_state(payload).await,
        },
        Err(_) => ServerMessage::Error { message: "Invalid message format".to_string() },
    }
}

async fn get_moves(payload: GetMovesPayload) -> ServerMessage {
    let game = Game::load(&payload.name).await;
    let chess_state = game.get_current_state().unwrap();
    let valid_moves = chess_state.get_valid_moves(payload.row, payload.col);
    ServerMessage::Success(ServerMessageData::Moves { moves: valid_moves })
}

async fn move_piece(payload: MovePiecePayload) -> ServerMessage {
    let mut game = Game::load(&payload.name).await;

    let board_changed = game.move_piece(payload.from_row, payload.from_col, payload.to_row, payload.to_col);
    if board_changed {
        game.save().await;
    }
    ServerMessage::Success(ServerMessageData::ChessState { chess_state: game.get_current_state().unwrap() })
}

async fn select_pawn_promotion_piece(payload: SelectPawnPromotionPiecePayload) -> ServerMessage {
    let mut game = Game::load(&payload.name).await;

    match game.select_pawn_promotion_piece(payload.piece_str) {
        Ok(_) => {
            game.save().await;
            ServerMessage::Success(ServerMessageData::ChessState { chess_state: game.get_current_state().unwrap() })
        },
        Err(_) => ServerMessage::Error { message: "Invalid pawn promotion piece".to_string() },
    }
}

async fn get_previous_state(payload: GetGamePayload) -> ServerMessage {
    let mut game = Game::load(&payload.name).await;
    if game.get_previous_state().is_some() {
        game.save().await;
    }
    ServerMessage::Success(ServerMessageData::ChessState { chess_state: game.get_current_state().unwrap() })
}

async fn get_next_state(payload: GetGamePayload) -> ServerMessage {
    let mut game = Game::load(&payload.name).await;

    if game.get_next_state().is_some() {
        game.save().await;
    }
    ServerMessage::Success(ServerMessageData::ChessState { chess_state: game.get_current_state().unwrap() })
}

async fn get_current_state(payload: GetGamePayload) -> ServerMessage {
    let game = Game::load(&payload.name).await;
    ServerMessage::Success(ServerMessageData::ChessState { chess_state: game.get_current_state().unwrap() })
}
