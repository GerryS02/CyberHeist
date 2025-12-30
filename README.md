# CyberHeist: Terminal Infiltration

## Description of the Game

**CyberHeist: Terminal Infiltration** is a  hacking strategy game written entirely in Rust. Players act as cyber-operators attempting to compromise a security system, hack firewall nodes, and retrieve a 4-digit vault code before the lock timer expires. The game includes energy management, random events, and strategy.

The player must compromise a sufficient portion of a firewall grid and correctly guess the vault’s secret code before a countdown timer reaches zero.


---

## How to Run

1. Clone or download the project folder.
2. Open a terminal inside the project directory.
3. Run the following command:

```bash
cargo run
```

Cargo will compile the project and launch the game in the terminal.

---

## How to Play

### Starting the Game

1. When prompted, enter a **player handle** (any name).
2. Choose a difficulty level: `easy`, `normal`, or `hard`.

You will then see the main game screen showing:

* Lock timer (turns remaining)
* Player energy
* Number of firewall nodes compromised
* Available actions

### Objective

To **win** the game, you must:

1. Compromise enough firewall nodes (at least ~30% of the grid), and
2. Correctly guess the hidden 4‑digit vault code

Both objectives must be completed **before** the lock timer reaches zero.

### Commands

The game is controlled entirely through typed commands:

* `scan` (or `s`)
  Reveals hints about the vault code or firewall status. Costs a small amount of energy.

* `hack` (or `h`)
  Allows the player to either:

  * Hack a firewall node by choosing grid coordinates, or
  * Guess the 4‑digit vault code

* `brute` (or `b`)
  A high‑risk, high‑cost action that may crack the code faster but drains significant energy.

* `status`
  Displays full player and system information, including the firewall grid.

* `help`
  Displays instructions and command descriptions.

* `retreat` (or `r`)
  Abandons the mission and ends the game.

## Data Types (Chapter 6 Concepts)

This project intentionally applies multiple Rust data‑type concepts from Chapter 6:

* **Structs**
  Used to model complex entities such as the player and the security system.

* **Enums**
  Used for actions, node states, difficulty levels, and game results.

* **Arrays**
  Used to store the fixed‑size 4‑digit vault code.

* **Vectors (`Vec<T>`)**
  Used for the firewall grid, inventory, and dynamic collections of nodes.

* **Tuples**
  Used to represent coordinates within the firewall grid.

* **Option<T>**
  Used when actions may or may not return a value, such as scan hints.

* **Result<T, E>**
  Used for input handling and error management.

* **Pattern Matching (`match`)**
  Used extensively to handle user actions, node states, and difficulty logic.

* **Generics**
  Demonstrated through utility functions that operate on multiple data types.

----

![screenshot](https://github.com/user-attachments/assets/781ebf5d-2d99-4b0f-a00d-0fb13b691f87)