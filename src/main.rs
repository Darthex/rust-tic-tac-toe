use rand::Rng;
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use std::cmp::PartialEq;
use std::fmt::Formatter;
use std::{fmt, io};

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
}
impl State {
    fn new(rng: &mut ThreadRng) -> Self {
        State {
            tiles: [Cu8(0u8); 9],
            turn: rng.gen_range(0..=1),
            player_stamp: Cu8(rng.gen_range(1..=2)),
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

    fn update_board(&mut self, tile: usize, stamp: Cu8) {
        self.tiles[tile - 1] = stamp;
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

    fn bot_move(&mut self, rng: &mut ThreadRng) {
        let empty_tiles: Vec<usize> = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == Cu8(0))
            .map(|(i, _)| i)
            .collect();
        let valid_move = empty_tiles.choose(rng).unwrap();
        self.update_board(*valid_move + 1, self.player_stamp.negate());
    }

    fn change_turn(&mut self) {
        match self.turn {
            0 => self.turn = 1,
            1 => self.turn = 0,
            _ => self.turn = 0, // invalid case but let's not add Result as response
        }
    }

    fn _winner_tag(&self, stamp: Cu8) -> &'static str {
        if stamp == self.player_stamp {
            "You won!"
        } else {
            "Bot won, try again!"
        }
    }

    fn _tile_conditional(&self, t1: Cu8, t2: Cu8, t3: Cu8) -> bool {
        if t1 != Cu8(0) && t1 == t2 && t2 == t3 {
            true
        } else {
            false
        }
    }

    fn check_winner(&self) -> Option<&'static str> {
        if self._tile_conditional(self.tiles[0], self.tiles[1], self.tiles[2]) {
            Some(self._winner_tag(self.tiles[0]))
        } else if self._tile_conditional(self.tiles[3], self.tiles[4], self.tiles[5]) {
            Some(self._winner_tag(self.tiles[3]))
        } else if self._tile_conditional(self.tiles[6], self.tiles[7], self.tiles[8]) {
            Some(self._winner_tag(self.tiles[6]))
        } else if self._tile_conditional(self.tiles[0], self.tiles[3], self.tiles[6]) {
            Some(self._winner_tag(self.tiles[0]))
        } else if self._tile_conditional(self.tiles[1], self.tiles[4], self.tiles[7]) {
            Some(self._winner_tag(self.tiles[1]))
        } else if self._tile_conditional(self.tiles[2], self.tiles[5], self.tiles[8]) {
            Some(self._winner_tag(self.tiles[2]))
        } else if self._tile_conditional(self.tiles[0], self.tiles[4], self.tiles[8]) {
            Some(self._winner_tag(self.tiles[0]))
        } else if self._tile_conditional(self.tiles[2], self.tiles[4], self.tiles[6]) {
            Some(self._winner_tag(self.tiles[2]))
        } else if !self.tiles.contains(&Cu8(0)) {
            Some("Draw!")
        } else {
            None
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng(); // better to cache?
    let mut state = State::new(&mut rng);

    println!("{}", "Tic Tac Toe! [type quit to exit]");
    println!("Your stamp is {}\n\n", state.player_stamp);

    loop {
        match state.turn {
            0 => {
                println!("{}", "Bot's turn");
                state.bot_move(&mut rng);
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

                state.update_board(tile as usize, state.player_stamp);
            }
            _ => {}
        }

        println!("{}\n\n", state.get_board());
        state.change_turn();
        match state.check_winner() {
            Some(winner) => {
                println!("{}", winner);
                break;
            }
            None => continue,
        }
    }
}
