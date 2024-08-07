use crate::chess_state_history::ChessStateHistory;
use crate::chess_structs::ChessState;
use crate::database::{load_game, save_game};
use crate::settings::Settings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub settings: Settings,
    pub chess_state_history: ChessStateHistory,
}

impl Game {
    pub fn new(name: String, settings: Settings, chess_state_history: ChessStateHistory) -> Self {
        Game {
            name,
            settings,
            chess_state_history,
        }
    }

    pub async fn save(&self) -> bool {
        let result = save_game(self.clone()).await;
        let err = result.err();
        if err.is_some() {
            println!("Failed to save game: {:?}", err);
            return false;
        }
        return true;
    }

    pub async fn load(name: &String) -> Self {
        let result = load_game(name).await;
        match result {
            Ok(game) => return game,
            Err(err) => println!("Failed to load game: {:?}", err),
        }
        return Game::new(
            String::new(),
            Settings::new(false, false, false),
            ChessStateHistory::new(),
        );
    }

    pub fn get_current_state(&self) -> Option<ChessState> {
        return self.chess_state_history.get_current_state().clone();
    }

    pub fn move_piece(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        let mut chess_state = self.get_current_state().unwrap().clone();
        let change_made = chess_state.move_piece(from_row, from_col, to_row, to_col);
        if change_made {
            self.chess_state_history.add_state(chess_state);
            return true;
        }
        return false;
    }

    pub fn select_pawn_promotion_piece(&mut self, piece_str: String) -> Result<(), String> {
        let mut chess_state = self.get_current_state().unwrap().clone();
        let result = chess_state.select_pawn_promotion_piece(piece_str);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        self.chess_state_history.add_state(chess_state);
        return Ok(());
    }

    pub fn get_previous_state(&mut self) -> Option<ChessState> {
        // if we have a previous state, return it
        // otherwise return current state
        return self.chess_state_history.get_previous_state();
    }

    pub fn get_next_state(&mut self) -> Option<ChessState> {
        return self.chess_state_history.get_next_state();
    }
}
