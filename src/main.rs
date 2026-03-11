mod minimax;

use crate::minimax::best_move;
use rand::Rng;
use std::cmp::PartialEq;
use std::fmt::Formatter;
use std::{fmt, io};

/// All possible winning patterns on a tic-tac-toe board.
///
/// Each number represents a 9-bit mask where:
///
/// 0 | 1 | 2
/// ---------
/// 3 | 4 | 5
/// ---------
/// 6 | 7 | 8
///
/// Example:
/// Top row = 0b000000111
/// Middle row = 0b000111000
/// Left column = 0b001001001
const WIN_MASKS: [u16; 8] = [
    0b000000111, // row 1
    0b000111000, // row 2
    0b111000000, // row 3
    0b001001001, // col 1
    0b010010010, // col 2
    0b100100100, // col 3
    0b100010001, // diag
    0b001010100, // diag
];

#[derive(Copy, Clone, Debug)]
struct Cu8(u8);

impl Cu8 {
    fn get_stamp(&self) -> &str {
        match self {
            Cu8(1) => "X",
            Cu8(2) => "O",
            _ => " ",
        }
    }

    fn negate(&self) -> Cu8 {
        match self {
            Cu8(1) => Cu8(2),
            Cu8(2) => Cu8(1),
            _ => Cu8(0),
        }
    }
}

impl fmt::Display for Cu8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.get_stamp())
    }
}

impl PartialEq for Cu8 {
    fn eq(&self, other: &Self) -> bool {
        if self.0 != other.0 { false } else { true }
    }
}

struct State {
    tiles: [Cu8; 9],
    turn: u8, // 0 -> bot | 1 -> player
    player_stamp: Cu8,
    player_mask: u16,
    bot_mask: u16,
}

impl State {
    fn new() -> Self {
        let turn = rand::thread_rng().gen_range(0..=1);
        State {
            tiles: [Cu8(0u8); 9],
            turn,
            player_stamp: if turn == 0 { Cu8(2) } else { Cu8(1) },
            player_mask: 0b000000000,
            bot_mask: 0b000000000,
        }
    }

    fn get_board(&self) -> String {
        // better to use named params here?
        format!(
            "{} | {} | {}\n---------\n{} | {} | {}\n---------\n{} | {} | {}",
            self.tiles[0],
            self.tiles[1],
            self.tiles[2],
            self.tiles[3],
            self.tiles[4],
            self.tiles[5],
            self.tiles[6],
            self.tiles[7],
            self.tiles[8]
        )
    }

    fn update_board(&mut self, tile: usize, is_player: bool) {
        let stamp = if is_player {
            self.player_stamp
        } else {
            self.player_stamp.negate()
        };
        self.tiles[tile - 1] = stamp;
        if is_player {
            self.player_mask |= 1 << (tile - 1) //I messed up, it should've been 0 based indexing
        } else {
            self.bot_mask |= 1 << (tile - 1)
        };
    }

    fn validate_move(&self, tile: &str) -> Result<usize, &'static str> {
        let tile = tile.parse::<usize>();
        if tile.is_err() {
            return Err("Invalid move, Please input a number from 1 to 9!");
        }

        let tile = tile.unwrap();
        if tile < 1 || tile > 9 {
            Err("Invalid move, Please input a number from 1 to 9!")
        } else if self.tiles[tile - 1] != Cu8(0) {
            Err("Invalid move, tile is already occupied!")
        } else {
            Ok(tile)
        }
    }

    fn bot_move(&mut self) {
        let pos = best_move(self.player_mask, self.bot_mask);
        self.update_board(pos + 1, false);
    }

    fn change_turn(&mut self) {
        match self.turn {
            0 => self.turn = 1,
            1 => self.turn = 0,
            _ => self.turn = 0, // invalid case but let's not add Result as response
        }
    }

    fn _winner_tag(&self) -> &'static str {
        if self.turn == 1 {
            "You won!"
        } else {
            "Bot won, try again!"
        }
    }

    fn check_winner(&self) -> Option<&'static str> {
        let mask = if self.turn == 1 {
            self.player_mask
        } else {
            self.bot_mask
        };

        if WIN_MASKS.iter().any(|&win| win & mask == win) {
            Some(self._winner_tag())
        } else if !self.tiles.contains(&Cu8(0)) {
            Some("Draw!")
        } else {
            None
        }
    }
}

// can we just use bit masks for all operations?
fn main() {
    let mut state = State::new();

    println!("{}", "Tic Tac Toe! [type quit to exit]");
    println!("Your stamp is {}\n\n", state.player_stamp);

    loop {
        match state.turn {
            0 => {
                println!("{}", "Bot's turn");
                state.bot_move();
            }
            1 => {
                println!("{}", "Your turn, choose your move [0-9]");

                let mut tile = String::new();

                io::stdin()
                    .read_line(&mut tile)
                    .expect("Failed to read line");

                let tile = tile.trim();
                if tile == "quit" {
                    break;
                }

                let tile = match state.validate_move(tile) {
                    Ok(x) => x,
                    Err(x) => {
                        println!("{}\n\n", x);
                        continue;
                    }
                };

                state.update_board(tile, true);
            }
            _ => {}
        }

        println!("{}\n\n", state.get_board());
        match state.check_winner() {
            Some(tag) => {
                println!("{}", tag);
                break;
            }
            None => {}
        }
        state.change_turn();
    }
}
