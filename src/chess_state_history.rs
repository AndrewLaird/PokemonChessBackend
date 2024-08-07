use crate::chess_structs::ChessState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChessStateHistory {
    pub state_history: Vec<ChessState>,
    pub current_state_index: usize,
}

impl ChessStateHistory {
    pub fn new() -> Self {
        return ChessStateHistory {
            state_history: Vec::new(),
            current_state_index: 0,
        };
    }
    pub fn new_with_initial_state(state: ChessState) -> Self {
        return ChessStateHistory {
            state_history: vec![state],
            current_state_index: 0,
        };
    }

    pub fn get_current_state(&self) -> Option<ChessState> {
        let current_state_index: usize = self.current_state_index;
        if current_state_index < self.state_history.len() {
            return Some(self.state_history[current_state_index].clone());
        }
        return None;
    }

    pub fn add_state(&mut self, state: ChessState) -> &ChessStateHistory {
        // cut off states after the current state
        self.state_history.truncate(self.current_state_index + 1);
        // push new state
        self.state_history.push(state);
        self.current_state_index = self.state_history.len() - 1;
        return self;
    }

    pub fn get_previous_state(&mut self) -> Option<ChessState> {
        let current_state_index: usize = self.current_state_index;
        if current_state_index > 0 {
            self.current_state_index = self.current_state_index - 1;
            return Some(self.state_history[current_state_index - 1].clone());
        }
        return None;
    }
    pub fn get_next_state(&mut self) -> Option<ChessState> {
        let current_state_index: usize = self.current_state_index;
        if current_state_index < self.state_history.len().saturating_sub(1) {
            self.current_state_index += 1;
            return Some(self.state_history[current_state_index + 1].clone());
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_previous_state_no_states() {
        let mut chess_state_history = ChessStateHistory::new();
        assert_eq!(chess_state_history.get_previous_state(), None);
    }
    #[test]
    fn test_get_next_state_no_states() {
        let mut chess_state_history = ChessStateHistory::new();
        assert_eq!(chess_state_history.get_next_state(), None);
    }

    #[test]
    fn test_get_previous_state() {
        // test that previous state is different by the different settings
        // normally we would only care about the difference in the chessboard
        let mut chess_state_history = ChessStateHistory::new();
        let mut chess_state = ChessState::new();
        chess_state_history.add_state(chess_state.clone());
        // move pawns
        chess_state.move_piece(1, 0, 3, 0);
        chess_state_history.add_state(chess_state.clone());
        chess_state.move_piece(6, 0, 4, 0);
        chess_state_history.add_state(chess_state.clone());
        // assert that a move was made
        assert_eq!(chess_state.turn_count, 2);
        // no move back one turn
        println!("{}", chess_state.chessboard.display_board_str());
        chess_state = chess_state_history.get_previous_state().unwrap();
        println!("{}", chess_state.chessboard.display_board_str());
        assert_eq!(chess_state.turn_count, 1);
    }
}
