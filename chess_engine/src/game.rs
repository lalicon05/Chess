use crate::board::{BitBoard, Color, Piece, PieceKind};

// Game status enum for checkmate detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
}

// structure of the game itself, saves bitboard, halfmove clock and number of moves.
#[derive(Debug, Clone)]
pub struct Game {
    bitboard: BitBoard,
    side_to_move: Color,
    halfmove_clock: u32,
    fullmove_number: u32,
}

// Implementation of game where white starts and board is in start pos
impl Game {
    pub fn new() -> Self {
        Self {
            bitboard: BitBoard::startpos(),
            side_to_move: Color::White,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    // Reset function, creates a new board
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    // Gets which side that has a turn
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    // Allows for FEN notation
    pub fn fen(&self) -> String {
        // Board
        let mut out = String::new();
        for rank in (0..8u8).rev() {
            let mut empty = 0u8;
            for file in 0..8u8 {
                let square = rank * 8 + file; // a1 = 0, h8 = 63
                if let Some(piece) = self.bitboard.piece_at(square) {
                    if empty > 0 {
                        out.push(char::from(b'0' + empty));
                        empty = 0;
                    }
                    out.push(piece.fen_char());
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                out.push(char::from(b'0' + empty));
            }
            if rank != 0 {
                out.push('/');
            }
        }

        // Side to move
        out.push(' ');
        out.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        // Castling, en-passant, clocks (not implemented yet)
        out.push_str(" - - ");
        out.push_str(&self.halfmove_clock.to_string());
        out.push(' ');
        out.push_str(&self.fullmove_number.to_string());
        out
    }

    pub fn make_move_uci(&mut self, mv: &str) -> Result<(), String> {
        let mv = mv.trim();
        if mv.len() < 4 {
            return Err("UCI move must be at least 4 chars (e2e4)".to_string());
        }
        let from = parse_square(&mv[0..2])?;
        let to = parse_square(&mv[2..4])?;
        let promotion = if mv.len() >= 5 {
            Some(parse_promotion(mv.as_bytes()[4] as char)?)
        } else {
            None
        };

        let moving_piece = self
            .bitboard
            .piece_at(from)
            .ok_or_else(|| format!("No piece on {}", &mv[0..2]))?;
        if moving_piece.color != self.side_to_move {
            return Err("It's not that side's turn".to_string());
        }

        // Check destination isn't occupied by own piece
        if let Some(captured) = self.bitboard.piece_at(to) {
            if captured.color == moving_piece.color {
                return Err("Cannot capture your own piece".to_string());
            }
        }

        // Validate move is pseudo-legal
        if !self.is_move_pseudo_legal(from, to, moving_piece) {
            return Err("Illegal move: piece cannot move there".to_string());
        }

        // Check if move would leave king in check
        let mut temp_game = self.clone();
        temp_game.apply_move_unchecked(from, to, promotion);
        if temp_game.is_in_check(self.side_to_move) {
            return Err("Illegal move: king would be in check".to_string());
        }

        // Move is legal, apply it
        if !self.apply_move_unchecked(from, to, promotion) {
            return Err("Internal move application failed".to_string());
        }
        Ok(())
    }

    // Apply a move without validation
    fn apply_move_unchecked(&mut self, from: u8, to: u8, promotion: Option<PieceKind>) -> bool {
        let Some(moving_piece) = self.bitboard.piece_at(from) else {
            return false;
        };

        // Clear capture square
        self.bitboard.clear_square(to);

        // Move the piece
        self.bitboard.clear_square(from);

        let final_kind = if moving_piece.kind == PieceKind::Pawn {
            promotion.unwrap_or(PieceKind::Pawn)
        } else {
            moving_piece.kind
        };

        self.bitboard.set_piece(to, Piece {
            color: moving_piece.color,
            kind: final_kind,
        });

        // Update clocks
        self.halfmove_clock = 0;
        if self.side_to_move == Color::Black {
            self.fullmove_number += 1;
        }
        self.side_to_move = self.side_to_move.opposite();

        true
    }

    // Check if a move is pseudo-legal
    fn is_move_pseudo_legal(&self, from: u8, to: u8, piece: Piece) -> bool {
        if from == to {
            return false;
        }

        let from_file = from % 8;
        let from_rank = from / 8;
        let to_file = to % 8;
        let to_rank = to / 8;
        let file_diff = (to_file as i8 - from_file as i8).abs();
        let rank_diff = (to_rank as i8 - from_rank as i8).abs();

        let occupancy = self.bitboard.all_occupancy();

        match piece.kind {
            PieceKind::Pawn => {
                let forward = match piece.color {
                    Color::White => 1i8,
                    Color::Black => -1i8,
                };
                let start_rank = match piece.color {
                    Color::White => 1,
                    Color::Black => 6,
                };

                // Single push
                if to_file == from_file && to_rank as i8 == from_rank as i8 + forward {
                    return self.bitboard.piece_at(to).is_none();
                }

                // Double push
                if to_file == from_file 
                    && from_rank == start_rank
                    && to_rank as i8 == from_rank as i8 + 2 * forward {
                    let mid_square = ((from_rank as i8 + forward) * 8 + from_file as i8) as u8;
                    return self.bitboard.piece_at(mid_square).is_none()
                        && self.bitboard.piece_at(to).is_none();
                }

                // Capture
                if file_diff == 1 && to_rank as i8 == from_rank as i8 + forward {
                    return self.bitboard.piece_at(to).is_some();
                }

                false
            }

            PieceKind::Knight => {
                (file_diff == 2 && rank_diff == 1) || (file_diff == 1 && rank_diff == 2)
            }

            PieceKind::King => {
                file_diff <= 1 && rank_diff <= 1
            }

            PieceKind::Bishop => {
                if file_diff != rank_diff {
                    return false;
                }
                self.is_path_clear(from, to, occupancy)
            }

            PieceKind::Rook => {
                if from_file != to_file && from_rank != to_rank {
                    return false;
                }
                self.is_path_clear(from, to, occupancy)
            }

            PieceKind::Queen => {
                if from_file != to_file && from_rank != to_rank && file_diff != rank_diff {
                    return false;
                }
                self.is_path_clear(from, to, occupancy)
            }
        }
    }

    // Check if path between two squares is clear (for sliding pieces)
    fn is_path_clear(&self, from: u8, to: u8, occupancy: u64) -> bool {
        let from_file = from % 8;
        let from_rank = from / 8;
        let to_file = to % 8;
        let to_rank = to / 8;

        let file_step: i8 = (to_file as i8 - from_file as i8).signum();
        let rank_step: i8 = (to_rank as i8 - from_rank as i8).signum();

        let mut current_file = from_file as i8 + file_step;
        let mut current_rank = from_rank as i8 + rank_step;

        while current_file != to_file as i8 || current_rank != to_rank as i8 {
            let sq = (current_rank * 8 + current_file) as u8;
            if occupancy & (1u64 << sq) != 0 {
                return false;
            }
            current_file += file_step;
            current_rank += rank_step;
        }

        true
    }

    // Check if a given color's king is currently in check
    fn is_in_check(&self, color: Color) -> bool {
        if let Some(king_sq) = self.bitboard.king_square(color) {
            self.bitboard.is_square_attacked(king_sq, color.opposite())
        } else {
            false
        }
    }

    // Check game status
    // Now checks for Checkmate or stalemate or if the check is wscapable
    pub fn game_status(&self) -> GameStatus {
        let has_legal_moves = self.has_any_legal_move();
        let in_check = self.is_in_check(self.side_to_move);
        
        if !has_legal_moves {
            if in_check {
                GameStatus::Checkmate // Current side is checkmated
            } else {
                GameStatus::Stalemate // Draw
            }
        } else if in_check {
            GameStatus::Check // In check but can escape
        } else {
            GameStatus::InProgress
        }
    }

    // Check if current side has any legal move
    fn has_any_legal_move(&self) -> bool {
        // Try every piece of current side
        for from in 0..64 {
            if let Some(piece) = self.bitboard.piece_at(from) {
                if piece.color != self.side_to_move {
                    continue;
                }
                
                // Try all possible destinations
                for to in 0..64 {
                    if self.is_move_legal_internal(from, to, piece, None) {
                        return true; // Found at least one legal move
                    }
                }
            }
        }
        false
    }

    // Internal: check if move is legal (doesn't modify board)
    fn is_move_legal_internal(&self, from: u8, to: u8, piece: Piece, promotion: Option<PieceKind>) -> bool {
        // Check destination
        if let Some(captured) = self.bitboard.piece_at(to) {
            if captured.color == piece.color {
                return false;
            }
        }
        
        // Check pseudo-legality
        if !self.is_move_pseudo_legal(from, to, piece) {
            return false;
        }
        
        // Check king safety
        let mut temp = self.clone();
        if !temp.apply_move_unchecked(from, to, promotion) {
            return false;
        }
        !temp.is_in_check(self.side_to_move)
    }

    pub fn generate_legal_moves(&self) -> Vec<String> {
        let mut moves = Vec::new();

        for from in 0u8..64u8 {
            let Some(piece) = self.bitboard.piece_at(from) else {
                continue;
            };
            if piece.color != self.side_to_move {
                continue;
            }

            for to in 0u8..64u8 {
                // First check base legality
                if !self.is_move_legal_internal(from, to, piece, None) {
                    continue;
                }

                // Handle pawn promotions as explicit UCI moves (q/r/b/n)
                if piece.kind == PieceKind::Pawn {
                    let to_rank = to / 8;
                    let promotion_rank = match piece.color {
                        Color::White => 7,
                        Color::Black => 0,
                    };

                    if to_rank == promotion_rank {
                        for promo in [PieceKind::Queen, PieceKind::Rook, PieceKind::Bishop, PieceKind::Knight] {
                            if self.is_move_legal_internal(from, to, piece, Some(promo)) {
                                moves.push(move_to_uci(from, to, Some(promo)));
                            }
                        }
                        continue;
                    }
                }

                moves.push(move_to_uci(from, to, None));
            }
        }

        moves
    }
}

fn parse_square(s: &str) -> Result<u8, String> {
    let bytes = s.as_bytes();
    if bytes.len() != 2 {
        return Err("Square must be 2 chars".to_string());
    }
    let file = bytes[0];
    let rank = bytes[1];
    if !(b'a'..=b'h').contains(&file) {
        return Err("File must be a-h".to_string());
    }
    if !(b'1'..=b'8').contains(&rank) {
        return Err("Rank must be 1-8".to_string());
    }
    let f = file - b'a';
    let r = rank - b'1';
    Ok(r * 8 + f)
}

fn parse_promotion(c: char) -> Result<PieceKind, String> {
    match c.to_ascii_lowercase() {
        'q' => Ok(PieceKind::Queen),
        'r' => Ok(PieceKind::Rook),
        'b' => Ok(PieceKind::Bishop),
        'n' => Ok(PieceKind::Knight),
        _ => Err("Promotion must be one of q,r,b,n".to_string()),
    }
}

fn move_to_uci(from: u8, to: u8, promotion: Option<PieceKind>) -> String {
    let mut s = String::with_capacity(5);
    s.push((b'a' + (from % 8)) as char);
    s.push((b'1' + (from / 8)) as char);
    s.push((b'a' + (to % 8)) as char);
    s.push((b'1' + (to / 8)) as char);

    if let Some(p) = promotion {
        let promo = match p {
            PieceKind::Queen => Some('q'),
            PieceKind::Rook => Some('r'),
            PieceKind::Bishop => Some('b'),
            PieceKind::Knight => Some('n'),
            _ => None,
        };
        if let Some(ch) = promo {
            s.push(ch);
        }
    }

    s
}
