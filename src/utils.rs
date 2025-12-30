use std::io; 

// Generic clamp function to constrain a value between low and high
pub fn clamp_generic<T: Ord>(val: T, low: T, high: T) -> T {
    if val < low {
        low // Return lower bound if val is less
    } else if val > high {
        high // Return upper bound if val is greater
    } else {
        val // Return val if within bounds
    }
}

// Function to read a line of input from the user
pub fn read_input() -> Result<String, io::Error> {
    let mut buf = String::new();         // Buffer to store input
    std::io::stdin().read_line(&mut buf)?; // Read a line from stdin
    Ok(buf.trim().into())                 // Trim whitespace and return as String
}
