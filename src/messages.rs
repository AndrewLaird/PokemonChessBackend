use crate::chess_structs::{InfoMessage, InteractionType};

impl InfoMessage {
    pub fn get_message_from_interaction_type(
        interaction_type: InteractionType,
        moves_available: bool,
    ) -> Option<InfoMessage> {
        match (interaction_type, moves_available) {
            (InteractionType::SuperEffective, true) => Some(InfoMessage::SuperEffective),
            (InteractionType::SuperEffective, false) => {
                Some(InfoMessage::SuperEffectiveNoMovesAvailable)
            }
            (InteractionType::NoEffect, _) => Some(InfoMessage::NoEffect),
            (InteractionType::NotVeryEffective, _) => Some(InfoMessage::NotVeryEffective),
            _ => None,
        }
    }
}
