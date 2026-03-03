# Chess
In this project I attempt to make a chess engine and a neural network in rust to get to know the programming language better.

## Web UI (Rust engine + HTML/CSS/JS)

The idea is:

- Rust takes care of the game state itself to later be expanded to a chess engine and Lichess compatability
- Rust uses a small API:
	- `fen()` returns the current position as a FEN string
	- `make_move_uci("e2e4")` applies a move (UCI format)
- js only draws and gets clicks: it asks Rust for `fen()`, draws the board using `web_thing/assets/*.png`, and sends moves back to Rust.

Right now move application is intentionally simple (no legality checking, castling, or en-passant yet) — it’s just enough to visualize moves coming from Rust.

### Build the Rust engine to WebAssembly

1) Install the wasm target (one-time):

- `rustup target add wasm32-unknown-unknown`

2) Install `wasm-pack` (one-time):
1. 
- `cargo install wasm-pack`

3) Build the package into `web_thing/pkg/`:

- From `chess_engine/` run:
	- `wasm-pack build --target web --out-dir ../web_thing/pkg`

### Run the web UI

must serve web_thing with a local server to play with graphics
The live server does not compile so in order to play, please follow instructions above

- `cd web_thing`
- `python -m http.server 8000`
- Open `http://localhost:8000/` in your browser