/* 
DATA TYPES USED:

1. Difficulty               -> enum {Easy, Normal, Hard}
2. SecuritySystem           -> struct
3. lock_timer               -> i32
4. code_digits              -> [u8; 4]
5. firewall                 -> Vec<FirewallNode>
6. honeypot_active          -> bool
7. Player                   -> struct (from crate::player)
8. FirewallNode             -> struct (from firewall module)
9. NodeState                -> enum (from firewall module)
10. Option<String>          -> Option type for hints
11. usize                   -> unsigned integer for indexing/counting
12. u8                      -> unsigned 8-bit integer for code digits
13. i32                     -> signed 32-bit integer
14. Vec<T>                  -> Vector type
15. RNG                     -> random number generator
*/

pub mod firewall; // firewall logic
pub mod code;     // vault code logic

use crate::player::Player;                // struct Player
use crate::utils::{clamp_generic};        // utility function clamp_generic
use rand::Rng;                             // random number generator trait

use firewall::{create_firewall_grid, FirewallNode, NodeState}; // firewall components

// Difficulty enum (used to determine game settings)
#[derive(Debug, Clone, Copy)]
pub enum Difficulty { // enum
    Easy,
    Normal,
    Hard,
}

// SecuritySystem struct
pub struct SecuritySystem { // struct
    pub lock_timer: i32,                   // i32: countdown before lockout
    pub code_digits: [u8; 4],              // [u8; 4]: 4-digit vault code
    pub firewall: Vec<FirewallNode>,       // Vec<FirewallNode>: firewall nodes grid
    pub difficulty: Difficulty,            // Difficulty enum: current difficulty
    pub honeypot_active: bool,             // bool: are honeypots active
}

impl SecuritySystem {
    // Constructor for SecuritySystem
    pub fn new(difficulty: Difficulty, rows: usize, cols: usize) -> Self { // returns SecuritySystem
        SecuritySystem {
            lock_timer: match difficulty {       // i32
                Difficulty::Easy => 25,
                Difficulty::Normal => 20,
                Difficulty::Hard => 16,
            },
            code_digits: code::generate_code(),        // [u8; 4]: randomly generated code
            firewall: create_firewall_grid(rows, cols, difficulty), // Vec<FirewallNode>
            difficulty,                                // Difficulty
            honeypot_active: true,                     // bool
        }
    }

    // Count compromised firewall nodes
    pub fn compromised_nodes(&self) -> usize {   // usize
        self.firewall
            .iter()
            .filter(|n| n.state == NodeState::Compromised)
            .count()
    }

    // Scan system for hints
    pub fn scan(&self, player: &mut Player, last_hint: Option<&String>) -> Option<String> { // Option<String>
        if player.energy < 8 {                        // i32 comparison
            return None;
        }
        player.energy -= 8;                           // reduce player energy (i32)

        let mut rng = rand::thread_rng();             // RNG
        let pick: u8 = rng.gen_range(0..3);           // u8: select hint type

        let hint: String = match pick {               // String
            0 => {                                    // sum of digits hint
                let sum: u8 = self.code_digits.iter().sum(); // u8
                format!("Scan hint: sum of digits = {}", sum)
            }
            1 => {                                    // specific digit hint
                let pos: usize = rng.gen_range(0..4);           // usize: position
                format!("Scan hint: digit {} = {}", pos + 1, self.code_digits[pos])
            }
            _ => {                                    // compromised nodes hint
                let cmp: usize = self.compromised_nodes();     // usize
                format!("Scan hint: {} nodes compromised.", cmp)
            }
        };

        if let Some(prev) = last_hint {               // Option<&String>
            if prev == &hint {
                return Some(format!("(refined) {}", hint));
            }
        }
        Some(hint)
    }

    // Attempt brute force
    pub fn brute_force(&mut self, player: &mut Player) -> bool { // returns bool
        if player.energy < 30 {                    // i32 check
            println!("Not enough energy!");
            return false;
        }

        player.energy -= 30;                       // reduce energy (i32)
        player.brute_force_used += 1;              // increment counter (i32)

        let compromised: usize = self.compromised_nodes(); // usize

        let base: i32 = match self.difficulty {   // i32 base chance
            Difficulty::Easy => 30,
            Difficulty::Normal => 18,
            Difficulty::Hard => 8,
        };

        let extra: i32 = compromised as i32 * 5;  // extra chance based on compromised nodes
        let threshold: i32 = clamp_generic(base + extra, 0, 90); // clamp between 0-90

        let roll: u32 = rand::thread_rng().gen_range(0..100); // random roll
        println!(
            "Brute force: compromised={}, threshold={}, roll={}",
            compromised, threshold, roll
        );

        roll < threshold as u32
    }

    // Check if victory condition is met
    pub fn check_victory(&self) -> bool {       // bool
        let cmp: usize = self.compromised_nodes(); // usize
        let total: usize = self.firewall.len();   // usize

        cmp * 100 / total >= 30
    }

    // Apply penalty for exploded nodes
    pub fn apply_explosion_penalty(&mut self) { // ()
        let exploded_count: usize =
            self.firewall.iter().filter(|n| n.state == NodeState::Exploded).count(); // usize
        if exploded_count > 0 {
            let penalty: i32 = exploded_count as i32 / 2; // i32
            self.lock_timer -= penalty;                   // reduce lock timer (i32)
            if penalty > 0 {
                println!(
                    "Exploded honeypots detected — lock timer accelerated by {}.",
                    penalty
                );
            }
        }
    }

    // Battery regeneration logic
    pub fn battery_regen(&self, player: &mut Player) { // ()
        if player.inventory.contains(&"Battery pack".to_string()) // bool: check inventory
            && rand::thread_rng().gen_bool(0.07)                 // bool: random chance
        {
            player.energy += 8;                                   // i32: restore energy
            println!("Battery pack restored some energy!");
        }

        if player.energy > 150 {                                   // i32 max energy
            player.energy = 150;
        }
    }
}
