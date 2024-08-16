use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::{Extension, debug_handler};
use serde::{Deserialize, Serialize};
use crate::game::Game;
use crate::chess_structs::{ChessState, Move};
use crate::app_state::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};



#[derive(Deserialize)]
#[serde(tag = "action", content = "payload")]
enum ClientMessage {
    SubscribeToGame(GetGamePayload),
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

#[debug_handler]
pub async fn handler(
    ws: WebSocketUpgrade, 
    Extension(app_state): Extension<Arc<Mutex<AppState>>>,
    ) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, app_state))
}

pub async fn handle_socket(mut socket: WebSocket, app_state: Arc<Mutex<AppState>>) {
    // we want to put the sender into our AppState so that when
    // a move is made, we can send it to everyone in the room
    let (mut sender, receiver) = socket.split();
    
    tokio::spawn(handle_reciever(receiver, app_state.clone()));
    // send a message to the client saying that they connected
    let _ = sender.send(Message::Text(String::from("{Connected}"))).await;

}

async fn handle_reciever(mut receiver: SplitStream<WebSocket>, app_state: Arc<Mutex<AppState>>){
    while let Some(msg) = receiver.next().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };
        if let Message::Text(text) = msg {
            let (room_name, response) = handle_message(text, app_state.clone()).await;
            // send the response to everyone in the room
            // need state to do that
            let room_tx = app_state.lock().await.get_room_tx(&room_name);
            // have to send as string not message
            let _ = room_tx.send(serde_json::to_string(&response).unwrap());
        }
    }

}

async fn handle_message(message: String, app_state: Arc<Mutex<AppState>>) -> (String, ServerMessage) {
    match serde_json::from_str::<ClientMessage>(&message) {
        Ok(msg) => match msg {
            ClientMessage::SubscribeToGame(payload) => (payload.name.clone(), subscribe_to_game(payload, app_state.clone()).await),
            ClientMessage::GetMoves(payload) => (payload.name.clone(), get_moves(payload).await),
            ClientMessage::MovePiece(payload) => (payload.name.clone(), move_piece(payload).await),
            ClientMessage::SelectPawnPromotionPiece(payload) => (payload.name.clone(), select_pawn_promotion_piece(payload).await),
            ClientMessage::GetPreviousState(payload) => (payload.name.clone(), get_previous_state(payload).await),
            ClientMessage::GetNextState(payload) => (payload.name.clone(), get_next_state(payload).await),
            ClientMessage::GetCurrentState(payload) => (payload.name.clone(), get_current_state(payload).await),
        },
        Err(_) => ("".to_string(), ServerMessage::Error { message: "Invalid message format".to_string() }),
    }
}

async fn subscribe_to_game(payload: GetGamePayload, app_state: Arc<Mutex<AppState>>) -> ServerMessage {
    let game = Game::load(&payload.name).await;
    // app_state.subscribe_to_game(&payload.name);
    ServerMessage::Success(ServerMessageData::ChessState { chess_state: game.get_current_state().unwrap() })
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
