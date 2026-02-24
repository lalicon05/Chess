use crate::board::{BitBoard, Color, Piece, PieceKind};

// structure of the game itself, saves bitboard, halfmove clock and number of moves.
#[derive(Debug, Clone)]
pub struct Game {
    bitboard: BitBoard,
    side_to_move: Color,
    halfmove_clock: u32,
    fullmove_number: u32,
}

// Implementation of game wherw white starts.
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

        // Clear capture square (if any).
        if let Some(captured) = self.bitboard.piece_at(to) {
            if captured.color == moving_piece.color {
                return Err("Cannot capture your own piece".to_string());
            }
            self.bitboard.clear_square(to);
        }

        // Move the piece.
        self.bitboard.clear_square(from);

        let final_kind = if moving_piece.kind == PieceKind::Pawn {
            if let Some(promo) = promotion {
                // Promotion only makes sense on last rank, but we keep it simple.
                promo
            } else {
                PieceKind::Pawn
            }
        } else {
            if promotion.is_some() {
                return Err("Promotion is only valid for pawns".to_string());
            }
            moving_piece.kind
        };

        self.bitboard.set_piece(to, Piece {
            color: moving_piece.color,
            kind: final_kind,
        });

        // Update clocks (very simplified)
        self.halfmove_clock = 0;
        if self.side_to_move == Color::Black {
            self.fullmove_number += 1;
        }
        self.side_to_move = self.side_to_move.opposite();
        Ok(())
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
