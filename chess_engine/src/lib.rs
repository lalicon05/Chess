pub mod board;
pub mod game;

pub use game::Game;

// WebAssembly bridge.
// Keeps the core engine usable from native Rust (tests/CLI) while exposing a small API to JS.
#[cfg(target_arch = "wasm32")]
mod wasm_api {
    use wasm_bindgen::prelude::*;

    use crate::game::Game;

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
    }
}
