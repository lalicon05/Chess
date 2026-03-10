pub mod board;
pub mod game;

pub use game::{Game, GameStatus};

// WebAssembly bridge.
// Keeps the core engine usable from native Rust (tests/CLI) while exposing a small API to JS.
#[cfg(target_arch = "wasm32")]
mod wasm_api {
    use wasm_bindgen::prelude::*;

    use crate::game::{Game, GameStatus};

    #[wasm_bindgen]
    pub struct WasmGame {
        inner: Game,
    }

    #[wasm_bindgen]
    impl WasmGame {
        #[wasm_bindgen(constructor)]
        pub fn new() -> WasmGame {
            WasmGame { inner: Game::new() }
        }

        pub fn reset(&mut self) {
            self.inner.reset();
        }

        pub fn fen(&self) -> String {
            self.inner.fen()
        }

        // Accepts a UCI move like "e2e4" or "e7e8q".
        // For now this is a simple piece move (no legality checking, castling, or en-passant yet).
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
    }
}
