// File that will contain Dennis the chess engine

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::game::Game;

pub struct DennisEngine;

impl DennisEngine {
    pub fn pick_move(game: &Game) -> Option<String> {
        let moves = game.generate_legal_moves();
        if moves.is_empty() {
            return None;
        }

        let mut rng = thread_rng();
        moves.choose(&mut rng).cloned()
    }
}