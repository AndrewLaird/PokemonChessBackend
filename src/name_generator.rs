use crate::pokemon_names::POKEMON_NAMES;
use rand::Rng;
use crate::database::read_names_from_file;
use std::collections::HashSet;

pub fn generate_name() -> String {
    // get two random index
    let random_index_1 = rand::thread_rng().gen_range(0..POKEMON_NAMES.len());
    let random_index_2 = rand::thread_rng().gen_range(0..POKEMON_NAMES.len());
    
    // Get a random Pokémon name from POKEMON_NAMES
    let random_name_1 = POKEMON_NAMES[random_index_1];
    let random_name_2 = POKEMON_NAMES[random_index_2];
    
    // Create the Pokémon name by combining the type and name
    let name = format!("{} {}", random_name_1, random_name_2);
    let name = name.replace(" ", "_");
    
    return name;
}

pub async fn generate_game_name() -> Result<String, Box<dyn std::error::Error>> {
    // We are going to generate a name, check if it already exists, and if it does, generate a new one
    let mut name = generate_name();

    let current_game_names = read_names_from_file().await.unwrap();
    let current_game_names_set: HashSet<String> = current_game_names.iter().cloned().collect();
    while current_game_names_set.contains(&name) {
        name = generate_name();
    }
    
    return Ok(name);
}


