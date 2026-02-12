# Chess Engine & UI Task List

Use `- [ ]` for todos and change to `- [x]` when done.

## 0. Repo / Project Setup

- [x] Confirm `.gitignore` is correct and `target/` is not tracked (use `git ls-files target`)
- [x] Make sure project structure is:
  - `chess_engine/` (Rust crate)
  - `web_thing/` (HTML/CSS/JS UI)

## 1. Core Types & Board Representation (Rust)

Work in `chess_engine/src/board.rs`.

- [ ] Define `Color` type (e.g., white/black)
- [ ] Define piece kind type (pawn/knight/bishop/rook/queen/king)
- [x] Decide if `BitBoard` is your main `Board` or part of it
- [x] Extend `BitBoard` or create `Board` to include:
  - [x] Castling rights
  - [ ] En passant square (if any)
  - [ ] Halfmove clock (for 50-move rule)
  - [ ] Fullmove number
- [ ] Fix derive attribute on `BitBoard` (use proper Rust `#[derive(...)]`)
- [ ] Remove unused imports from `board.rs`

## 2. Starting Position & Printing (Rust)

- [ ] Implement a constructor or function that builds the standard starting position
- [ ] Implement a way to convert the board to a human-readable string (ASCII or similar)
- [ ] In `chess_engine/src/main.rs`, replace `Hello, world!` with:
  - [ ] Create a starting board
  - [ ] Print it to stdout
- [ ] Run `cargo run` and verify the printed position matches the standard chess start

## 3. Move Representation & Application (Rust)

- [ ] Define a move type that at least contains:
  - [ ] From-square
  - [ ] To-square
  - [ ] Optional promotion piece
- [ ] Implement a method on the board to apply a move:
  - [ ] Move piece from source to destination
  - [ ] Remove captured piece (if any)
  - [ ] Handle promotions
  - [ ] Update castling rights
  - [ ] Handle en passant (setting and capturing)
  - [ ] Update halfmove/fullmove counters
  - [ ] Switch `side_to_move`
- [ ] In `main.rs`, hard-code a short sequence of moves, apply them, and print the board after each

## 4. Move Generation (Pseudo-Legal, Rust)

- [ ] For each piece type, define how it moves in terms of directions/steps
- [ ] Implement pseudo-legal move generation for the side to move:
  - [ ] Pawn moves (single, double, captures, promotions, en passant targets)
  - [ ] Knight moves
  - [ ] Bishop moves
  - [ ] Rook moves
  - [ ] Queen moves
  - [ ] King moves (including castling in pseudo-legal sense)
- [ ] Add helper functions/tests for some specific positions to verify the move lists

## 5. Legal Move Filtering (Rust)

- [ ] Implement a way to detect if a given side is in check
- [ ] Implement legal move generation by filtering pseudo-legal moves:
  - [ ] For each pseudo-legal move, apply it temporarily
  - [ ] Discard moves that leave own king in check

## 6. CLI Driver (Rust)

Work in `chess_engine/src/main.rs`.

- [ ] Implement a simple text interface:
  - [ ] Print the board
  - [ ] Read a move from stdin (your own notation is fine, e.g. `e2e4`)
  - [ ] Parse the move into your internal move type
  - [ ] Check if it is in the legal move list
  - [ ] Apply it, loop until game over or user exit

## 7. Basic Web UI (HTML/CSS/JS)

Work in `web_thing/`.

- [ ] `index.html`: basic page layout with an area for the chess board
- [ ] `style.css`: styling for an 8x8 grid and pieces
- [ ] `main.js`: logic to draw the board using a simple in-memory position (initially hard-coded)
- [ ] Ability to click pieces and squares and show selected move (no backend yet)

## 8. HTTP API in Rust

- [ ] Decide on a simple JSON format for:
  - [ ] Board state
  - [ ] Moves (from/to/promotion)
- [ ] Add an HTTP server in `chess_engine` that exposes endpoints:
  - [ ] `new_game` with mode: human vs human, human vs engine, human vs NN, engine vs NN
  - [ ] `get_state` to get current board and game info
  - [ ] `make_move` for human moves
  - [ ] `engine_move` to let engine play a move (later, when engine search exists)
  - [ ] `nn_move` to let NN play a move (later)

## 9. Connect Web UI to Rust Backend

- [ ] In `main.js`, replace hard-coded board with data from the HTTP API
- [ ] Implement human move sending via `make_move`
- [ ] Request `engine_move` or `nn_move` when it is their turn
- [ ] Support the different play modes from the UI

## 10. Engine Search & Evaluation (Later)

- [ ] Add a simple evaluation function (material + basic piece-square tables)
- [ ] Implement a basic search (minimax with alpha-beta)
- [ ] Integrate search into `engine_move`
- [ ] Design and integrate a neural network module for evaluation or policy

You can now go through this file and mark items as done by changing `- [ ]` to `- [x]`. This gives you a clear checklist and progress marker.
