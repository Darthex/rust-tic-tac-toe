use crate::WIN_MASKS;

/// Check if a given board mask contains any winning pattern.
///
/// `mask` represents all tiles chosen by one player.
/// If the mask fully contains any WIN_MASK, that player has won.
///
/// Example:
///
/// mask      = 0b000000111
/// win_mask  = 0b000000111
///
/// mask & win_mask == win_mask → win
fn is_win(mask: u16) -> bool {
    WIN_MASKS.iter().any(|&w| mask & w == w)
}

/// Detect if the board is full (draw).
///
/// We OR the player and bot masks to combine all occupied tiles.
/// Then we count how many bits are set.
///
/// If all 9 tiles are occupied → draw.
fn is_draw(p_mask: u16, b_mask: u16) -> bool {
    (p_mask | b_mask).count_ones() == 9 // cz nine tiles
}

/// Compute all currently available board positions.
///
/// We check each position from 0-8.
/// If the bit is not set in the occupied mask, the tile is free.
///
/// Returns a list of tile indices.
fn available_moves(p_mask: u16, b_mask: u16) -> Vec<u16> {
    let occupied = p_mask | b_mask;
    (0..9).filter(|&i| occupied & (1 << i) == 0).collect()
}

/// Core minimax algorithm.
///
/// This recursively simulates all possible future game states
/// assuming both players play optimally.
///
/// Return values:
///
///  1  → bot eventually wins
///  0  → draw
/// -1  → player eventually wins
///
/// Arguments:
///
/// player_mask → all moves taken by the human
/// bot_mask    → all moves taken by the bot
/// maximize    → if it's bot's turn we have to maximize gains and minimize loss
///
/// The algorithm alternates between:
///
/// bot  → maximize score
/// user → minimize score
fn minimax(p_mask: u16, b_mask: u16, maximize: bool) -> i16 {
    if is_win(b_mask) {
        return 1;
    }
    if is_win(p_mask) {
        return -1;
    }
    if is_draw(p_mask, b_mask) {
        return 0;
    }

    if maximize {
        let mut best_score = i16::MIN;
        for pos in available_moves(p_mask, b_mask) {
            let new_b_mask = b_mask | (1 << pos);
            let score = minimax(p_mask, new_b_mask, !maximize);
            best_score = best_score.max(score);
        }
        best_score
    } else {
        let mut best_score = i16::MAX;
        for pos in available_moves(p_mask, b_mask) {
            let new_p_mask = p_mask | (1 << pos);
            let score = minimax(new_p_mask, b_mask, !maximize);
            best_score = best_score.min(score);
        }
        best_score
    }
}

/// Determine the optimal move for the bot.
///
/// This function tries every possible move and uses
/// the minimax algorithm to evaluate the outcome.
///
/// The move with the highest score is selected.
pub fn best_move(player_mask: u16, bot_mask: u16) -> usize {
    let mut best_score = i16::MIN;
    let mut best_move = 0;

    for pos in available_moves(player_mask, bot_mask) {
        // Simulate bot placing at this position
        let new_bot_mask = bot_mask | (1 << pos);

        // Evaluate the resulting game state
        let score = minimax(player_mask, new_bot_mask, false);

        // If this move is better, store it
        if score > best_score {
            best_score = score;
            best_move = pos as usize;
        }
    }

    best_move
}
