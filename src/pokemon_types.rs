use crate::chess_structs::{InteractionType, PokemonType};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

// all messed up
impl PokemonType {
    pub fn get_type_chart() -> [[InteractionType; 19]; 19] {
        let type_chart: [[InteractionType; 19]; 19] = [
            // Normal
            [
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::NoEffect,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Fire
            [
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Water
            [
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Electric
            [
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NoEffect,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Grass
            [
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Ice
            [
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Fighting
            [
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::SuperEffective,
                InteractionType::NoEffect,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
            ],
            // Poison
            [
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NoEffect,
                InteractionType::SuperEffective,
                InteractionType::Normal,
            ],
            // Ground
            [
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NoEffect,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Flying
            [
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Psychic
            [
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NoEffect,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Bug
            [
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
            ],
            // Rock
            [
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Ghost
            [
                InteractionType::NoEffect,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // Dragon
            [
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::NoEffect,
                InteractionType::Normal,
            ],
            // Dark
            [
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
            ],
            // Steel
            [
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::SuperEffective,
                InteractionType::Normal,
            ],
            // Fairy
            [
                InteractionType::Normal,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::SuperEffective,
                InteractionType::SuperEffective,
                InteractionType::NotVeryEffective,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
            // NoType (for empty spaces)
            [
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
                InteractionType::Normal,
            ],
        ];
        return type_chart;
    }

    pub fn type_matchup(type1: PokemonType, type2: PokemonType) -> InteractionType {
        // 2d array to implement
        // Normal Fighting Flying Poison Ground Rock Bug Ghost Steel Fire Water Grass Electric Psychic Ice Dragon Dark Fairy
        return Self::get_type_chart()[type1 as usize][type2 as usize];
    }

    pub fn random() -> Self {
        // duplication sucks but there's no way to count enum variants without a macro
        let variants = [
            PokemonType::Normal,
            PokemonType::Fire,
            PokemonType::Water,
            PokemonType::Electric,
            PokemonType::Grass,
            PokemonType::Ice,
            PokemonType::Fighting,
            PokemonType::Poison,
            PokemonType::Ground,
            PokemonType::Flying,
            PokemonType::Psychic,
            PokemonType::Bug,
            PokemonType::Rock,
            PokemonType::Ghost,
            PokemonType::Dragon,
            PokemonType::Dark,
            PokemonType::Steel,
            PokemonType::Fairy,
        ];

        let mut rng = thread_rng();
        return *variants.choose(&mut rng).expect("Variant array is empty!");
    }
}

impl fmt::Display for PokemonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
