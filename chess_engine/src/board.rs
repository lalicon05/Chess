// I am just learning rust and started coding in this file after instructions from
// the chess programming wiki. I am not yet apt with rust so bear with the comments
// that overexplain what code does.
// Maybe reading through this file when I have gotten further will help someone else
// who is also learning rust understand how to create chess.


// Color handling
// public enumeration with colors black and white
pub fn hello_from_board() {
	println!("hello from board");
}

// Enumerator for the colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
	White,
	Black,
}

// Implementation method to get opposite color
impl Color {
	pub fn opposite(&self) -> Color {
		match self {
			Color::White => Color::Black,
			Color::Black => Color::White,
		}
	}
}

// Board should include everything needed to initialize the Board
// Store all pieces in the bitboard struct
// And save different states like en passant or (the switching castle and king move thing)
#[derive(Debug, Clone, Copy)]
struct Board {
	bitboard: BitBoard,
	side_to_move: Color,
	white_can_castle_kingside: bool,
	white_can_castle_queenside: bool,
	black_can_castle_kingside: bool,
	black_can_castle_queenside: bool,
	en_passant_square: Option<u8>, // en-passant target, None if not available
	halfmove_clock: u32, // for the fifty move draw rule
	fullmove_number: u32,
}

// Bitboard struct to represent the board
// Each piece type and color will have its own bitboard
// This allows for efficient move generation and board representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard {
	pawns: [u64; 2], // 0 for white, 1 for black
	knights: [u64; 2],
	bishops: [u64; 2],
	rooks: [u64; 2],
	queens: [u64; 2],
	kings: [u64; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}

// Struct to organize pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
	pub color: Color,
	pub kind: PieceKind,
}

// Implementation of the piece type
impl Piece {
	pub fn fen_char(&self) -> char {
		let c = match self.kind {
			PieceKind::Pawn => 'p',
			PieceKind::Knight => 'n',
			PieceKind::Bishop => 'b',
			PieceKind::Rook => 'r',
			PieceKind::Queen => 'q',
			PieceKind::King => 'k',
		};
		match self.color {
			Color::White => c.to_ascii_uppercase(),
			Color::Black => c,
		}
	}
}

// Implementation of the bitboard
impl BitBoard {
	pub fn empty() -> Self {
		Self {
			pawns: [0; 2],
			knights: [0; 2],
			bishops: [0; 2],
			rooks: [0; 2],
			queens: [0; 2],
			kings: [0; 2],
		}
	}

	// Defines the starting position of all the pieces. Uses hexa decimal to shorten the number
	// Should be the only time this is set manually, I hate this
	pub fn startpos() -> Self {
		// mapping squares, 0 is a1
		let mut bb = Self::empty();

		// White
		bb.pawns[0] = 0x0000_0000_0000_FF00;
		bb.rooks[0] = 0x0000_0000_0000_0081;
		bb.knights[0] = 0x0000_0000_0000_0042;
		bb.bishops[0] = 0x0000_0000_0000_0024;
		bb.queens[0] = 0x0000_0000_0000_0008;
		bb.kings[0] = 0x0000_0000_0000_0010;

		// Black
		bb.pawns[1] = 0x00FF_0000_0000_0000;
		bb.rooks[1] = 0x8100_0000_0000_0000;
		bb.knights[1] = 0x4200_0000_0000_0000;
		bb.bishops[1] = 0x2400_0000_0000_0000;
		bb.queens[1] = 0x0800_0000_0000_0000;
		bb.kings[1] = 0x1000_0000_0000_0000;

		bb
	}

	// Returns what type of piece is standing on a square if any
	pub fn piece_at(&self, square: u8) -> Option<Piece> {
		if square >= 64 {
			return None;
		}
		let mask = 1u64 << square;
		for color in [Color::White, Color::Black] {
			let idx = color_index(color);
			if self.pawns[idx] & mask != 0 {
				return Some(Piece { color, kind: PieceKind::Pawn });
			}
			if self.knights[idx] & mask != 0 {
				return Some(Piece { color, kind: PieceKind::Knight });
			}
			if self.bishops[idx] & mask != 0 {
				return Some(Piece { color, kind: PieceKind::Bishop });
			}
			if self.rooks[idx] & mask != 0 {
				return Some(Piece { color, kind: PieceKind::Rook });
			}
			if self.queens[idx] & mask != 0 {
				return Some(Piece { color, kind: PieceKind::Queen });
			}
			if self.kings[idx] & mask != 0 {
				return Some(Piece { color, kind: PieceKind::King });
			}
		}
		None
	}

	// Removes piece from square regardless of type
	pub fn clear_square(&mut self, square: u8) {
		if square >= 64 {
			return;
		}
		let mask = !(1u64 << square);
		for idx in 0..2 {
			self.pawns[idx] &= mask;
			self.knights[idx] &= mask;
			self.bishops[idx] &= mask;
			self.rooks[idx] &= mask;
			self.queens[idx] &= mask;
			self.kings[idx] &= mask;
		}
	}

	// Allows you to decide what piece should be at a square
	pub fn set_piece(&mut self, square: u8, piece: Piece) {
		if square >= 64 {
			return;
		}
		let mask = 1u64 << square;
		let idx = color_index(piece.color);
		match piece.kind {
			PieceKind::Pawn => self.pawns[idx] |= mask,
			PieceKind::Knight => self.knights[idx] |= mask,
			PieceKind::Bishop => self.bishops[idx] |= mask,
			PieceKind::Rook => self.rooks[idx] |= mask,
			PieceKind::Queen => self.queens[idx] |= mask,
			PieceKind::King => self.kings[idx] |= mask,
		}
	}

	// Returns a bitboard of all pieces of a given color
	pub fn occupancy_for(&self, color: Color) -> u64 {
		let idx = color_index(color);
		self.pawns[idx]
			| self.knights[idx]
			| self.bishops[idx]
			| self.rooks[idx]
			| self.queens[idx]
			| self.kings[idx]
	}

	// Returns a bitboard of all pieces (both colors)
	pub fn all_occupancy(&self) -> u64 {
		self.occupancy_for(Color::White) | self.occupancy_for(Color::Black)
	}

	// Finds the square of the king for a given color
	pub fn king_square(&self, color: Color) -> Option<u8> {
		let idx = color_index(color);
		let king_bb = self.kings[idx];
		if king_bb == 0 {
			return None;
		}
		Some(king_bb.trailing_zeros() as u8)
	}

	// Check if a square is attacked by a given color
	pub fn is_square_attacked(&self, square: u8, by_color: Color) -> bool {
		if square >= 64 {
			return false;
		}
		let idx = color_index(by_color);
		let occupancy = self.all_occupancy();

		// Pawn attacks
		let pawn_attacks = match by_color {
			Color::White => {
				let mut attacks = 0u64;
				if square >= 8 && square % 8 != 0 {
					attacks |= 1u64 << (square - 9); // down-left
				}
				if square >= 8 && square % 8 != 7 {
					attacks |= 1u64 << (square - 7); // down-right
				}
				attacks
			}
			Color::Black => {
				let mut attacks = 0u64;
				if square < 56 && square % 8 != 0 {
					attacks |= 1u64 << (square + 7); // up-left
				}
				if square < 56 && square % 8 != 7 {
					attacks |= 1u64 << (square + 9); // up-right
				}
				attacks
			}
		};
		if self.pawns[idx] & pawn_attacks != 0 {
			return true;
		}

		// Knight attacks
		let knight_offsets: [(i8, i8); 8] = [
			(2, 1), (2, -1), (-2, 1), (-2, -1),
			(1, 2), (1, -2), (-1, 2), (-1, -2),
		];
		for (df, dr) in knight_offsets {
			let f = (square % 8) as i8 + df;
			let r = (square / 8) as i8 + dr;
			if f >= 0 && f < 8 && r >= 0 && r < 8 {
				let target = (r * 8 + f) as u8;
				if self.knights[idx] & (1u64 << target) != 0 {
					return true;
				}
			}
		}

		// King attacks (for detecting checks from enemy king, though rare)
		let king_offsets: [(i8, i8); 8] = [
			(1, 0), (-1, 0), (0, 1), (0, -1),
			(1, 1), (1, -1), (-1, 1), (-1, -1),
		];
		for (df, dr) in king_offsets {
			let f = (square % 8) as i8 + df;
			let r = (square / 8) as i8 + dr;
			if f >= 0 && f < 8 && r >= 0 && r < 8 {
				let target = (r * 8 + f) as u8;
				if self.kings[idx] & (1u64 << target) != 0 {
					return true;
				}
			}
		}

		// Sliding pieces: bishops, rooks, queens
		// Check all 8 directions
		let directions: [(i8, i8); 8] = [
			(1, 0), (-1, 0), (0, 1), (0, -1),   // rook directions
			(1, 1), (1, -1), (-1, 1), (-1, -1), // bishop directions
		];

		for (df, dr) in directions {
			let mut f = (square % 8) as i8;
			let mut r = (square / 8) as i8;
			let is_diagonal = df != 0 && dr != 0;

			loop {
				f += df;
				r += dr;
				if f < 0 || f >= 8 || r < 0 || r >= 8 {
					break;
				}
				let target = (r * 8 + f) as u8;
				let target_mask = 1u64 << target;

				// If there's a piece in the way
				if occupancy & target_mask != 0 {
					// Check if it's an attacking piece
					if is_diagonal {
						if self.bishops[idx] & target_mask != 0 || self.queens[idx] & target_mask != 0 {
							return true;
						}
					} else {
						if self.rooks[idx] & target_mask != 0 || self.queens[idx] & target_mask != 0 {
							return true;
						}
					}
					break; // Stop in this direction
				}
			}
		}

		false
	}
}

// matches input with color and returns 0 for white, 1 for black
fn color_index(color: Color) -> usize {
	match color {
		Color::White => 0,
		Color::Black => 1,
	}
}
