/*
DATA TYPES USED:

1.  GameResult    -> Enum: Success, Failure(String)
*/

mod player;    // Player module
mod actions;   // Player action parsing module
mod ui;        // User interface helpers
mod utils;     // Utility functions (input, clamp, etc.)
mod game;      // Main game logic
mod security;  // Security system, firewall, and code modules

use game::run_game;      // Import main game function
use game::GameResult;    // Import game result enum

fn main() {
    // Run the game and handle the outcome
    match run_game() {
        GameResult::Success => {
            // Player succeeded
            println!("\n--- MISSION ACCOMPLISHED: VAULT OPENED ---");
            println!("Congratulations!");
        }
        GameResult::Failure(reason) => {
            // Player failed
            println!("\n--- MISSION FAILED ---");
            println!("Reason: {}", reason);
            println!("Try again or review tactics.");
        }
    }

    // End of game message
    println!("Thanks for playing CyberHeist!");
}
