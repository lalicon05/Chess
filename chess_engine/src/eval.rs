use crate::board::Color;
use crate::game::Game;

pub const MATE_SCORE: i32 = 100_000;

// Positive score means "good for side to move".
pub fn evaluate(game: &Game) -> i32 {
    let fen = game.fen();
    let board_part = fen.split_whitespace().next().unwrap_or("");

    let mut material = 0i32;

    for ch in board_part.chars() {
        material += match ch {
            'P' => 100,
            'N' => 320,
            'B' => 330,
            'R' => 500,
            'Q' => 900,
            'K' => 0,
            'p' => -100,
            'n' => -320,
            'b' => -330,
            'r' => -500,
            'q' => -900,
            'k' => 0,
            _ => 0, // digits, slashes, spaces
        };
    }

    match game.side_to_move() {
        Color::White => material,
        Color::Black => -material,
    }
}