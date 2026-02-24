# Chess
In this project I attempt to make a chess engine and a neural network in rust to get to know the programming language better.

## Web UI (Rust engine + HTML/CSS/JS)

The idea is:

- **Rust owns the game state** (piece locations, side to move, etc.)
- Rust exposes a small API:
	- `fen()` returns the current position as a FEN string
	- `make_move_uci("e2e4")` applies a move (UCI format)
- **JavaScript only renders + handles clicks**: it asks Rust for `fen()`, draws the board using `web_thing/assets/*.png`, and sends moves back to Rust.

Right now move application is intentionally simple (no legality checking, castling, or en-passant yet) — it’s just enough to visualize moves coming from Rust.

### Build the Rust engine to WebAssembly

1) Install the wasm target (one-time):

- `rustup target add wasm32-unknown-unknown`

2) Install `wasm-pack` (one-time):

- `cargo install wasm-pack`

3) Build the package into `web_thing/pkg/`:

- From `chess_engine/` run:
	- `wasm-pack build --target web --out-dir ../web_thing/pkg`

### Run the web UI

Because ES modules + WASM imports need an HTTP server (not `file://`), you must serve `web_thing/`.

Note: **Live Server only serves files**; it does not compile Rust. You still need to run the WASM build step (once, or in watch mode) to keep `web_thing/pkg/` updated.

- `cd web_thing`
- `python -m http.server 8000`
- Open `http://localhost:8000/` in your browser

### VS Code workflow (recommended)

This repo includes tasks in `.vscode/tasks.json`:

- Run Task: `dev` (starts a rebuild-on-save loop + serves `web_thing/`)
- Then open `http://localhost:8000/`

If `cargo watch` is missing, install it once:

- `cargo install cargo-watch`
