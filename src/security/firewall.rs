/*
DATA TYPES USED:

1. NodeState            -> Enum: Secure, Vulnerable, Compromised, Exploded
2. FirewallNode          -> Struct
3. nodes                 -> Vec<FirewallNode>
4. r, c                  -> usize, loop indices for rows and columns
5. chance                -> u32, random number to determine node state
6. state                 -> NodeState, current state of the node
7. node_diff             -> u8, difficulty of the node
8. player                -> &mut Player
9. energy_cost           -> i32, energy required to hack a node
10. rng                  -> ThreadRng, random number generator
11. base                 -> u8, base chance of hack success
12. energy_bonus          -> u8, bonus based on player energy
13. roll                  -> u32, random number roll to determine hack success
14. threshold             -> u8, calculated success threshold
15. explode_roll          -> u32, random roll for honeypot explosion
16. fw                    -> &[FirewallNode], slice of nodes to display
*/

// Import random number generator
use rand::Rng;

// Import clamp function and Player struct
use crate::utils::clamp_generic;
use crate::player::Player;
use super::Difficulty;

// Enum to represent the state of a firewall node
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Secure,      // Node is safe, normal difficulty
    Vulnerable,  // Node is easier to hack
    Compromised, // Node has been hacked
    Exploded,    // Node triggered honeypot and is now dangerous
}

// Struct to represent a firewall node
#[derive(Debug, Clone)]
pub struct FirewallNode {
    pub coord: (usize, usize), // Coordinates (row, column)
    pub state: NodeState,      // Current state
    pub difficulty: u8,        // Difficulty value for hacking
}

// Function to create the firewall grid
pub fn create_firewall_grid(
    rows: usize,           // Number of rows
    cols: usize,           // Number of columns
    difficulty: Difficulty // Game difficulty
) -> Vec<FirewallNode> {
    let mut nodes = Vec::with_capacity(rows * cols); // Vector to hold all nodes
    let mut rng = rand::thread_rng(); // Random number generator

    for r in 0..rows {
        for c in 0..cols {
            let chance = rng.gen_range(0..100); // Random chance for node state

            // Decide node state based on difficulty and chance
            let state = match difficulty {
                Difficulty::Easy => if chance < 45 { NodeState::Vulnerable } else { NodeState::Secure },
                Difficulty::Normal => if chance < 30 { NodeState::Vulnerable } else { NodeState::Secure },
                Difficulty::Hard => if chance < 15 { NodeState::Vulnerable } else { NodeState::Secure },
            };

            // Assign difficulty value based on game difficulty
            let node_diff = match difficulty {
                Difficulty::Easy => rng.gen_range(5..30),
                Difficulty::Normal => rng.gen_range(20..60),
                Difficulty::Hard => rng.gen_range(40..95),
            } as u8;

            // Add node to vector
            nodes.push(FirewallNode {
                coord: (r, c),
                state,
                difficulty: node_diff,
            });
        }
    }

    nodes // Return the vector of nodes
}

// Function to attempt hacking a firewall node
pub fn attempt_hack_node(player: &mut Player, node: &mut FirewallNode) -> bool {
    let (r, c) = node.coord; // Get node coordinates

    // If already compromised
    match node.state {
        NodeState::Compromised => {
            println!("Node ({},{}) already compromised.", r, c);
            return true;
        }
        NodeState::Exploded => {
            println!("Node ({},{}) exploded. Avoid.", r, c);
            return false;
        }
        _ => {}
    }

    // Calculate energy cost to hack
    let energy_cost = (node.difficulty as i32 / 10) + 5;

    if player.energy < energy_cost {
        println!("Not enough energy for hack: need {}", energy_cost);
        return false;
    }

    // Deduct energy and increase hacks done
    player.energy -= energy_cost;
    player.hacks_done += 1;

    let mut rng = rand::thread_rng();

    // Base chance of success
    let base = if node.state == NodeState::Vulnerable { 50 } else { 20 };

    // Energy bonus improves chance
    let energy_bonus = clamp_generic(player.energy, 0, 50) as u8;

    let roll = rng.gen_range(0..100); // Random roll
    let threshold = base + (energy_bonus / 2) + (100 - node.difficulty) / 5; // Success threshold

    println!(
        "Hack ({},{}): diff={}, cost={}, roll={}, threshold={}",
        r, c, node.difficulty, energy_cost, roll, threshold
    );

    if roll < threshold as u32 {
        node.state = NodeState::Compromised; // Hack succeeded
        println!("Hack succeeded.");
        true
    } else {
        let explode_roll = rng.gen_range(0..100);
        if explode_roll > 85 { // Chance of honeypot triggering
            node.state = NodeState::Exploded;
            println!("Honeypot triggered! Node exploded.");
        } else {
            println!("Hack failed.");
        }
        false
    }
}

// Function to display the firewall grid
pub fn display_firewall_map(fw: &[FirewallNode], rows: usize, cols: usize) {
    println!("Firewall (C=Comp, V=Vuln, .=Secure, X=Exploded):");
    for r in 0..rows {
        for c in 0..cols {
            let idx = r * cols + c;
            let ch = match fw[idx].state {
                NodeState::Compromised => 'C',
                NodeState::Vulnerable => 'V',
                NodeState::Secure => '.',
                NodeState::Exploded => 'X',
            };
            print!("{} ", ch);
        }
        println!();
    }
}
