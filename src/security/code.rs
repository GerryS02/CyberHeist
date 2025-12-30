/*
DATA TYPES USED:

1. arr : [u8; 4]       -> Array of 4 unsigned 8-bit integers (used for code or guess)
2. rng : ThreadRng      -> Random number generator object
3. correct : usize      -> Counter for digits correct in value & position
4. misplaced : usize    -> Counter for digits correct but in wrong position
5. code_counts : HashMap<u8, usize>  -> Counts of unmatched digits in secret code
6. guess_counts : HashMap<u8, usize> -> Counts of unmatched digits in user guess
7. input : String       -> Raw input string from user
8. raw : &str           -> Trimmed string slice of input
9. digit : &u8          -> Key from HashMap iteration
10. gcount : &usize     -> Value from HashMap iteration
11. i : usize           -> Loop index
12. ch : char           -> Character from input string
*/

// Read input
use crate::utils::read_input;
// Import HashMap type
use std::collections::HashMap;
// Import random number generator trait
use rand::Rng;

// Function to generate a random 4-digit code
pub fn generate_code() -> [u8; 4] {
    let mut rng = rand::thread_rng(); // Thread-local random number generator
    let mut arr = [0; 4]; // 4-element array of u8, initialized to 0

    for i in 0..4 { // Loop through indices 0 to 3
        arr[i] = rng.gen_range(0..10); 
        // Assign a random number 0-9 to arr[i]
    }
    arr // Return the array
}

// Function to provide feedback on a code guess
// Returns a tuple (correct_position, correct_digit_wrong_position)
pub fn mastermind_feedback(code: &[u8; 4], guess: &[u8; 4]) -> (usize, usize) {
    let mut correct = 0; // Count of digits correct in value & position
    let mut code_counts = HashMap::new(); // Counts of unmatched code digits
    let mut guess_counts = HashMap::new(); // Counts of unmatched guess digits

    // Compare each position in code and guess
    for i in 0..4 {
        if code[i] == guess[i] { // Exact match
            correct += 1;
        } else { // Not an exact match
            *code_counts.entry(code[i]).or_insert(0) += 1;
            *guess_counts.entry(guess[i]).or_insert(0) += 1;
        }
    }

    let mut misplaced = 0; // Count of correct digits in wrong position

    // Compare unmatched digits
    for (digit, &gcount) in guess_counts.iter() {
        if let Some(&ccount) = code_counts.get(digit) {
            misplaced += std::cmp::min(ccount, gcount);
            // Add the minimum occurrence between code and guess
        }
    }

    (correct, misplaced) // Return feedback
}

// Function to read a 4-digit guess from user
pub fn read_code_guess() -> Option<[u8; 4]> {
    println!("Enter 4-digit guess:"); // Prompt user
    let input = read_input().ok()?; // Read input -> Option<String>
    let raw = input.trim(); // Trim whitespace -> &str

    // Validate input length and ensure all characters are digits
    if raw.len() != 4 || !raw.chars().all(|c| c.is_ascii_digit()) {
        println!("Invalid format.");
        return None;
    }

    let mut arr = [0u8; 4]; // Array to store digits
    for (i, ch) in raw.chars().enumerate() {
        arr[i] = ch.to_digit(10)? as u8; // Convert char to u8 digit
    }
    Some(arr) // Return valid guess
}
