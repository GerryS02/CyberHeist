/*
DATA TYPES USED:

1. Player            -> Struct representing the player
2. name              -> String, player handle
3. energy            -> i32, current energy level
4. hacks_done        -> u32, number of hacks performed
5. brute_force_used  -> u8, number of brute-force attempts
6. inventory         -> Vec<String>, items held by the player
7. position          -> (usize, usize), player's current grid position
8. difficulty        -> Enum Difficulty: Easy, Normal, Hard
*/

use crate::security::Difficulty; // Import difficulty enum

// Struct representing the player
#[derive(Debug)]
pub struct Player {
    pub name: String,            // Player handle / name
    pub energy: i32,             // Current energy
    pub hacks_done: u32,         // Number of successful hacks
    pub brute_force_used: u8,    // Number of brute force attempts used
    pub inventory: Vec<String>,  // Player's items
}

impl Player {
    // Constructor for creating a new player
    pub fn new(name: &str, difficulty: Difficulty) -> Self {
        // Set initial energy based on game difficulty
        let energy = match difficulty {
            Difficulty::Easy => 120,
            Difficulty::Normal => 100,
            Difficulty::Hard => 80,
        };

        Player {
            name: name.to_string(),               // Convert &str to String
            energy,                               // Set initial energy
            hacks_done: 0,                        // No hacks yet
            brute_force_used: 0,                  // No brute force used
            inventory: vec!["USB exploit".to_string()], // Start with a basic item
        }
    }
}
