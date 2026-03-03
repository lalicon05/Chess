// I am just learning rust and started coding in this file after instructions from
// the chess programming wiki. I am not yet apt with rust so bear with the comments
// that overexplain what code does.
// Maybe reading through this file when I have gotten further will help someone else
// who is also learning rust understand how to create chess.


// TODO: Implement visualization of the board using html and css (seperate file but will need to be compatible with BitBoard)
// TODO: Implement move generation and make sure it is efficient

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
}

// matches input with color and returns 0 for white, 1 for black
fn color_index(color: Color) -> usize {
	match color {
		Color::White => 0,
		Color::Black => 1,
	}
}
