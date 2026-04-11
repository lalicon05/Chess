pub mod board;
pub mod dennis_engine;
pub mod eval;
pub mod game;

pub use game::{Game, GameStatus};

// WebAssembly bridge.
// Keeps the core engine usable from native Rust while exposing a small API to JS.
#[cfg(target_arch = "wasm32")]
mod wasm_api {
    use wasm_bindgen::prelude::*;

    use crate::dennis_engine::DennisEngine;
    use crate::game::{Game, GameStatus};

    #[wasm_bindgen]
    pub struct WasmGame {
        inner: Game,
    }

    #[wasm_bindgen]
    impl WasmGame {
        #[wasm_bindgen(constructor)]
        pub fn new() -> WasmGame {
            console_error_panic_hook::set_once();
            WasmGame { inner: Game::new() }
        }

        pub fn reset(&mut self) {
            self.inner.reset();
        }

        pub fn fen(&self) -> String {
            self.inner.fen()
        }

        // Accepts a UCI move like "e2e4" or "e7e8q".
        // For now this is a simple piece move, legality is checked in game.rs
        pub fn make_move_uci(&mut self, mv: &str) -> Result<(), JsValue> {
            self.inner
                .make_move_uci(mv)
                .map_err(|e| JsValue::from_str(&e))
        }

        // Returns game status: "in_progress", "check", "checkmate", or "stalemate"
        pub fn game_status(&self) -> String {
            match self.inner.game_status() {
                GameStatus::InProgress => "in_progress".to_string(),
                GameStatus::Check => "check".to_string(),
                GameStatus::Checkmate => "checkmate".to_string(),
                GameStatus::Stalemate => "stalemate".to_string(),
            }
        }

        pub fn make_engine_move(&mut self) -> Result<String, JsValue> {
            let mv = DennisEngine::pick_move(&self.inner)
                .ok_or_else(|| JsValue::from_str("No legal moves available"))?;

            self.inner
                .make_move_uci(&mv)
                .map_err(|e| JsValue::from_str(&e))?;

            Ok(mv)
        }

        pub fn make_engine_move_limited(&mut self, max_depth: u8, max_time_ms: u64) -> Result<String, JsValue> {
            let mv = DennisEngine::pick_move_with_limits(&self.inner, max_depth, max_time_ms)
                .ok_or_else(|| JsValue::from_str("No legal moves available"))?;

            self.inner
                .make_move_uci(&mv)
                .map_err(|e| JsValue::from_str(&e))?;

            Ok(mv)
        }
    }
}
