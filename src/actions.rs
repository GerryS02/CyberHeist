/*
DATA TYPES USED:

1. Action           -> Enum: Scan, Hack, BruteForce, Retreat, Status, Help
2. s                 -> &str, input string
3. Err               -> (), unit type for error
4. Result<Self, Self::Err> -> Result type returned by from_str
*/

use std::str::FromStr; // Import FromStr trait to convert strings into enum

// Enum representing possible player actions
#[derive(Debug, Clone, Copy)]
pub enum Action { // enum
    Scan,        // Scan the security system
    Hack,        // Attempt to hack a node or code
    BruteForce,  // Attempt brute force attack
    Retreat,     // Exit or retreat
    Status,      // Check player/system status
    Help,        // Display help/instructions
}

// Implement conversion from string to Action enum
impl FromStr for Action {
    type Err = (); // Unit type () used as error

    fn from_str(s: &str) -> Result<Self, Self::Err> { // returns Result<Action, ()>
        match s.trim().to_lowercase().as_str() { // convert input string to lowercase and trim whitespace
            "scan" | "s" => Ok(Action::Scan),               // map "scan" or "s" to Action::Scan
            "hack" | "h" => Ok(Action::Hack),               // map "hack" or "h" to Action::Hack
            "brute" | "b" => Ok(Action::BruteForce),       // map "brute" or "b" to Action::BruteForce
            "retreat" | "r" => Ok(Action::Retreat),        // map "retreat" or "r" to Action::Retreat
            "status" => Ok(Action::Status),                // map "status" to Action::Status
            "help" => Ok(Action::Help),                    // map "help" to Action::Help
            _ => Err(()),                                  // if none match, return error ()
        }
    }
}
