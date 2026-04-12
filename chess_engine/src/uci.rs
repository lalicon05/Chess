use std::io::{self, BufRead, Write};

use crate::dennis_engine::DennisEngine;
use crate::game::Game;

pub fn run_uci_loop() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut game = Game::new();

    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            break;
        };
        let cmd = line.trim();

        if cmd.is_empty() {
            continue;
        }

        if cmd == "uci" {
            println!("id name Dennis v1.0");
            println!("id author lalicon05");
            println!("uciok");
            let _ = stdout.flush();
            continue;
        }

        if cmd == "isready" {
            println!("readyok");
            let _ = stdout.flush();
            continue;
        }

        if cmd == "ucinewgame" {
            game.reset();
            continue;
        }

        if cmd.starts_with("position") {
            if let Err(e) = set_position(cmd, &mut game) {
                println!("info string position error: {e}");
                let _ = stdout.flush();
            }
            continue;
        }

        if cmd.starts_with("go") {
            let (depth, movetime_ms) = parse_go_limits(cmd);
            let best = DennisEngine::pick_move_with_limits(&game, depth, movetime_ms);

            match best {
                Some(mv) => {
                    println!("bestmove {mv}");
                }
                None => {
                    println!("bestmove 0000");
                }
            }
            let _ = stdout.flush();
            continue;
        }

        if cmd == "stop" {
            // Search is synchronous right now, so nothing to stop.
            continue;
        }

        if cmd == "quit" {
            break;
        }
    }
}

fn set_position(cmd: &str, game: &mut Game) -> Result<(), String> {
    // Supported:
    // position startpos
    // position startpos moves e2e4 e7e5 ...
    //
    // FEN input can be added later when Game has FEN loading.
    let rest = cmd.strip_prefix("position").unwrap_or("").trim();

    if rest.starts_with("startpos") {
        // Build on a temp position first (atomic update).
        let mut temp = Game::new();

        let after_startpos = rest.strip_prefix("startpos").unwrap_or("").trim();
        if let Some(moves_part) = after_startpos.strip_prefix("moves") {
            for mv in moves_part.split_whitespace() {
                temp.make_move_uci(mv)
                    .map_err(|e| format!("invalid move '{mv}': {e}"))?;
            }
        }

        // Commit only if everything succeeded
        *game = temp;
        return Ok(());
    }

    if rest.starts_with("fen ") {
        return Err("FEN position not supported yet (startpos supported)".to_string());
    }

    Err("Unsupported position command".to_string())
}

fn parse_go_limits(cmd: &str) -> (u8, u64) {
    // Defaults for v1
    let mut depth: u8 = 10;
    let mut movetime_ms: u64 = 2000;

    let tokens: Vec<&str> = cmd.split_whitespace().collect();
    let mut i = 1usize; // skip "go"

    while i < tokens.len() {
        match tokens[i] {
            "depth" => {
                if i + 1 < tokens.len() {
                    if let Ok(d) = tokens[i + 1].parse::<u8>() {
                        depth = d.max(1);
                    }
                    i += 2;
                    continue;
                }
            }
            "movetime" => {
                if i + 1 < tokens.len() {
                    if let Ok(t) = tokens[i + 1].parse::<u64>() {
                        movetime_ms = t.max(1);
                    }
                    i += 2;
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }

    (depth, movetime_ms)
}