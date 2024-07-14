use crate::chess_structs::{InfoMessage, InteractionType, Move, Player};

impl InfoMessage {
    pub fn get_message_from_interaction_type(
        interaction_type: InteractionType,
    ) -> Option<InfoMessage> {
        match interaction_type {
            InteractionType::SuperEffective => Some(InfoMessage::SuperEffective),
            InteractionType::NoEffect => Some(InfoMessage::NoEffect),
            InteractionType::NotVeryEffective => Some(InfoMessage::NotVeryEffective),
            _ => None,
        }
    }
}
