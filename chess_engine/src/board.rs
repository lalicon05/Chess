// I am just learning rust and started coding in this file after instructions from
// the chess programming wiki. I am not yet apt with rust so bear with the comments
// that overexplain what code does.
// Maybe reading through this file when I have gotten further will help someone else
// who is also learning rust understand how to create chess.


// TODO: Implement visualization of the board using html and css (seperate file but will need to be compatible with BitBoard)
// TODO: Implement move generation and make sure it is efficient

// Color handling
// public enumeration with colors black and white
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
#Derive(Debug, Clone, Copy)]
struct Board {
	bitboard: BitBoard,
	side_to_move: Color,
	white_can_castle_kingside: bool,
	white_can_castle_queenside: bool,
	black_can_castle_kingside: bool,
	black_can_castle_queenside: bool,
	en_passant_square: Option<u8>, // en-passant target, None if not available
}

// Bitboard struct to represent the board
// Each piece type and color will have its own bitboard
// This allows for efficient move generation and board representation
pub struct BitBoard {
	pawns: [u64; 2], // 0 for white, 1 for black
	knights: [u64; 2],
	bishops: [u64; 2],
	rooks: [u64; 2],
	queens: [u64; 2],
	kings: [u64; 2],
}


enum pieces {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}