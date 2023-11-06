use crate::chess_structs::{ChessBoard, ChessState};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt , BufReader};
use std::error::Error;

pub async fn save_board(
    file_name: String,
    board: ChessState,
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = String::from("games/");
    let full_path = directory + &file_name + ".pchess";
    println!("Saving: {}", board.chessboard.display_board_str()); // Debug print statement
    println!("Saving to: {}", full_path); // Debug print statement

    let mut file = File::create(&full_path).await?;

    // Convert the board to a JSON string
    let j = serde_json::to_string(&board)?;

    // Write the JSON string to the file
    file.write_all(j.as_bytes()).await?;
    file.flush().await?; // Explicitly flush the file buffer

    Ok(())
}

pub async fn load_board(file_name: &String) -> Result<ChessState, Box<dyn std::error::Error>> {
    let directory = String::from("games/");
    let full_path = directory + &file_name + ".pchess";
    let mut file = File::open(&full_path).await?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    // Deserialize the JSON string to a ChessState
    let board: ChessState = serde_json::from_str(&contents)?;

    Ok(board)
}




pub async fn read_names_from_file() -> Result<Vec<String>, Box<dyn Error>> {
    let filename = "game_names.txt";
    let file = File::open(&filename).await?;
    let reader = BufReader::new(file);
    let mut names = Vec::new();

    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        names.push(line);
    }

    Ok(names)
}

pub async fn write_name_to_file(name: String) -> Result<(), Box<dyn Error>> {
    let filename = "game_names.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)
        .await?;

    file.write_all(name.as_bytes()).await?;
    file.write_all(b"\n").await?;

    file.flush().await?;  // Ensure all writes are flushed to the file.

    Ok(())
}
