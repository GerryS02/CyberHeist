/*
DATA TYPES USED:

1. GameResult       -> Enum: Success, Failure(String)
2. name             -> String, player name
3. difficulty       -> Difficulty enum: Easy, Normal, Hard
4. player           -> Player struct
5. rows, cols       -> usize, size of the firewall grid
6. system           -> SecuritySystem struct
7. last_scan        -> Option<String>, last scan hint
8. code_cracked     -> bool, whether code has been guessed
9. start_time       -> Instant, game start time
10. input           -> String, player input
11. action          -> Action enum, parsed player action
12. choice          -> String, hack target choice
13. coords          -> String, row and column input for hack
14. parts           -> Vec<&str>, split coordinates
15. r, c            -> usize, row and column indices
16. idx             -> usize, index in firewall vector
17. success         -> bool, result of hack or brute force
18. reveal_roll, pos -> u32 / usize, random numbers for hints
19. elapsed         -> Duration, time elapsed
*/

use std::io::{self, Write}; // For input/output and flushing stdout
use std::time::Instant;      // For timing the game

use crate::actions::Action; // Enum for player actions
use crate::player::Player;  // Player struct
use crate::security::{SecuritySystem, Difficulty}; // Security system and difficulty
use crate::security::firewall::{display_firewall_map, attempt_hack_node}; // Firewall functions
use crate::security::code::{mastermind_feedback, read_code_guess}; // Code functions
use crate::ui::{display_header, show_help}; // UI helper functions
use crate::utils::{read_input}; // Utility functions
use rand::Rng; // Random number generator

// Enum for game outcome
pub enum GameResult {
    Success,                // Player wins
    Failure(String),        // Player loses with message
}

// Main game loop
pub fn run_game() -> GameResult {
    display_header(); // Display game header

    // Get player name
    println!("Enter your handle (player name): ");
    let name = match read_input() {
        Ok(s) if !s.is_empty() => s,
        _ => "Operator".to_string(), // Default name
    };

    // Select difficulty
    println!("Choose difficulty: easy / normal / hard (e/n/h). Default: normal");
    let difficulty = match read_input() {
        Ok(s) => match s.trim().to_lowercase().as_str() {
            "e" | "easy" => Difficulty::Easy,
            "h" | "hard" => Difficulty::Hard,
            _ => Difficulty::Normal,
        },
        Err(_) => Difficulty::Normal,
    };

    // Initialize player
    let mut player = Player::new(&name, difficulty);

    // Set firewall grid size
    let rows = 5usize;
    let cols = 5usize;

    // Initialize security system
    let mut system = SecuritySystem::new(difficulty, rows, cols);

    println!();
    println!("Welcome, {}. You have {} energy.", player.name, player.energy);
    println!("Your objective: Compromise the firewall and retrieve the 4-digit vault code before the lock timer expires.");
    println!("Type 'help' to see commands.");
    println!();

    // Track last scan hint and code guess status
    let mut last_scan: Option<String> = None;
    let mut code_cracked = false;
    let start_time = Instant::now(); // Start timer

    loop {
        // Check for loss conditions
        if system.lock_timer <= 0 {
            return GameResult::Failure("Alarm triggered — you were detected!".to_string());
        }
        if player.energy <= 0 {
            return GameResult::Failure("You ran out of energy!".to_string());
        }

        // Display turn status
        println!("\n--- Turn status ---");
        println!(
            "Lock timer: {} | Energy: {} | Hacks: {} | BruteUsed: {}",
            system.lock_timer, player.energy, player.hacks_done, player.brute_force_used
        );
        println!(
            "Firewall compromised: {}/{}",
            system.compromised_nodes(),
            system.firewall.len()
        );
        println!("Actions: scan(s), hack(h), brute(b), status, help, retreat(r)");
        print!("> ");
        io::stdout().flush().ok();

        // Read player input
        let input = match read_input() {
            Ok(s) => s,
            Err(_) => return GameResult::Failure("Input error".to_string()),
        };

        // Parse input into Action enum
        let action = match input.parse::<Action>() {
            Ok(a) => a,
            Err(_) => {
                println!("Unknown action. Type 'help' for options.");
                continue;
            }
        };

        match action {
            // Show help
            Action::Help => show_help(),

            // Show player and system status
            Action::Status => {
                println!("Player: {:?}", player);
                println!(
                    "System: difficulty={:?}, timer={}, honeypot={}",
                    system.difficulty, system.lock_timer, system.honeypot_active
                );
                display_firewall_map(&system.firewall, rows, cols);
            }

            // Retreat / exit
            Action::Retreat => return GameResult::Failure("Retreated from the heist.".to_string()),

            // Scan for hints
            Action::Scan => {
                let maybe_hint = system.scan(&mut player, last_scan.as_ref());
                if let Some(h) = maybe_hint {
                    println!("{}", h);
                    last_scan = Some(h);
                } else {
                    println!("Scan failed / not enough energy.");
                }
                system.lock_timer -= 1;
            }

            // Hack node or code
            Action::Hack => {
                println!("Hack target: (1) firewall node  (2) guess code");
                print!("choose 1 or 2 > ");
                io::stdout().flush().ok();

                let choice = read_input().unwrap_or("1".to_string());

                if choice.trim() == "2" {
                    // Code guessing
                    if let Some(guess) = read_code_guess() {
                        let (cp, cw) = mastermind_feedback(&system.code_digits, &guess);
                        println!("Feedback: {} correct pos, {} correct but wrong position", cp, cw);
                        if cp == 4 {
                            println!("You cracked the code!");
                            code_cracked = true;
                        } else {
                            println!("Code guess failed.");
                        }
                        player.energy -= 6;
                        system.lock_timer -= 1;
                    }
                } else {
                    // Hack firewall node
                    display_firewall_map(&system.firewall, rows, cols);
                    println!("Enter coordinates as 'r c': ");

                    let coords = read_input().unwrap_or_default();
                    let parts: Vec<_> = coords.split_whitespace().collect();

                    if parts.len() != 2 {
                        println!("Invalid coords.");
                        continue;
                    }

                    let r: usize = parts[0].parse().unwrap_or(99);
                    let c: usize = parts[1].parse().unwrap_or(99);

                    if r >= rows || c >= cols {
                        println!("Coordinates out of range.");
                        continue;
                    }

                    let idx = r * cols + c;

                    let success = attempt_hack_node(&mut player, &mut system.firewall[idx]);
                    if success {
                        let reveal_roll = rand::thread_rng().gen_range(0..100);
                        if reveal_roll < 20 {
                            let pos = rand::thread_rng().gen_range(0..4);
                            println!(
                                "Node data leak: digit at position {} = {}",
                                pos + 1,
                                system.code_digits[pos]
                            );
                        }
                    }

                    system.lock_timer -= 1;
                }
            }

            // Attempt brute force
            Action::BruteForce => {
                let success = system.brute_force(&mut player);
                if success {
                    code_cracked = true;
                }
                system.lock_timer -= 2;
            }
        };

        // Check victory condition
        if code_cracked && system.check_victory() {
            let elapsed = start_time.elapsed();
            println!("Success! Time taken: {:.2?}", elapsed);
            return GameResult::Success;
        }

        // Apply penalties and regeneration
        system.apply_explosion_penalty();
        system.battery_regen(&mut player);
    }
}
