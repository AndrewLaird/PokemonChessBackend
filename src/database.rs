use crate::chess_structs::{ChessBoard, ChessState};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
