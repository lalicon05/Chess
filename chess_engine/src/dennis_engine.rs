// File that will contain Dennis the chess engine

use std::time::{Duration, Instant};

use crate::eval::{evaluate, MATE_SCORE};
use crate::game::{Game, GameStatus};

const INF: i32 = 1_000_000;

#[derive(Debug, Clone, Copy)]
pub struct SearchLimits {
    pub max_depth: u8,
    pub max_time: Duration,
}

impl Default for SearchLimits {
    fn default() -> Self {
        Self {
            max_depth: 4,
            max_time: Duration::from_millis(2000),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub best_move: Option<String>,
    pub score: i32,
    pub depth_reached: u8,
    pub nodes: u64,
    pub timed_out: bool,
}

pub struct DennisEngine;

impl DennisEngine {
    pub fn pick_move(game: &Game) -> Option<String> {
        Self::search(game, SearchLimits::default()).best_move
    }

    pub fn pick_move_with_limits(game: &Game, max_depth: u8, max_time_ms: u64) -> Option<String> {
        let limits = SearchLimits {
            max_depth,
            max_time: Duration::from_millis(max_time_ms),
        };
        Self::search(game, limits).best_move
    }

    pub fn search(game: &Game, limits: SearchLimits) -> SearchResult {
        let deadline = Instant::now() + limits.max_time;
        let mut nodes = 0u64;

        let mut root_moves = game.generate_legal_moves();
        if root_moves.is_empty() {
            let score = terminal_score(game, 0);
            return SearchResult {
                best_move: None,
                score,
                depth_reached: 0,
                nodes,
                timed_out: false,
            };
        }

        let mut best_move_overall = None;
        let mut best_score_overall = -INF;
        let mut depth_reached = 0u8;
        let mut timed_out = false;

        for depth in 1..=limits.max_depth {
            if Instant::now() >= deadline {
                timed_out = true;
                break;
            }

            if let Some(ref pv_move) = best_move_overall {
                if let Some(idx) = root_moves.iter().position(|m| m == pv_move) {
                    root_moves.swap(0, idx);
                }
            }

            let mut alpha = -INF;
            let beta = INF;
            let mut best_move_this_depth: Option<String> = None;
            let mut best_score_this_depth = -INF;
            let mut completed_depth = true;

            for mv in &root_moves {
                if Instant::now() >= deadline {
                    completed_depth = false;
                    timed_out = true;
                    break;
                }

                let mut child = game.clone();
                if child.make_move_uci(mv).is_err() {
                    continue;
                }

                let (child_score, timeout) =
                    Self::negamax(&child, depth.saturating_sub(1), -beta, -alpha, deadline, &mut nodes);

                if timeout {
                    completed_depth = false;
                    timed_out = true;
                    break;
                }

                let score = -child_score;
                if score > best_score_this_depth {
                    best_score_this_depth = score;
                    best_move_this_depth = Some(mv.clone());
                }

                if score > alpha {
                    alpha = score;
                }
            }

            if completed_depth {
                depth_reached = depth;
                best_move_overall = best_move_this_depth;
                best_score_overall = best_score_this_depth;
            } else {
                break;
            }
        }

        SearchResult {
            best_move: best_move_overall,
            score: best_score_overall,
            depth_reached,
            nodes,
            timed_out,
        }
    }

    fn negamax(
        game: &Game,
        depth: u8,
        mut alpha: i32,
        beta: i32,
        deadline: Instant,
        nodes: &mut u64,
    ) -> (i32, bool) {
        if Instant::now() >= deadline {
            return (0, true);
        }

        *nodes += 1;

        match game.game_status() {
            GameStatus::Checkmate => return (terminal_score(game, depth), false),
            GameStatus::Stalemate => return (0, false),
            _ => {}
        }

        if depth == 0 {
            return (evaluate(game), false);
        }

        let moves = game.generate_legal_moves();
        if moves.is_empty() {
            return (terminal_score(game, depth), false);
        }

        let mut best = -INF;

        for mv in moves {
            if Instant::now() >= deadline {
                return (0, true);
            }

            let mut child = game.clone();
            if child.make_move_uci(&mv).is_err() {
                continue;
            }

            let (child_score, timeout) =
                Self::negamax(&child, depth.saturating_sub(1), -beta, -alpha, deadline, nodes);

            if timeout {
                return (0, true);
            }

            let score = -child_score;
            if score > best {
                best = score;
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        }

        if best == -INF {
            (evaluate(game), false)
        } else {
            (best, false)
        }
    }
}

fn terminal_score(game: &Game, ply_from_root: u8) -> i32 {
    match game.game_status() {
        // Side to move is checkmated => losing for side to move
        GameStatus::Checkmate => -MATE_SCORE + ply_from_root as i32,
        GameStatus::Stalemate => 0,
        _ => evaluate(game),
    }
}