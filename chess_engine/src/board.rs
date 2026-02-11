// I am just learning rust and started coding in this file after instructions from
// the chess programming wiki. I am not yet apt with rust so bear with the comments
// that overexplain what code does.
// Maybe reading through this file when I have gotten further will help someone else
// who is also learning rust understand how to create chess.


// TODO: Implement visualization of the board using html and css (seperate file but will need to be compatible with BitBoard)

#Derive(Debug, Clone, Copy)]
// BitBoard representation of the chess board.
// Each piece for each color is represented by a 64-bit unsigned integer
// Each bit corresponds to a square on the chess board.
// Added a comment just to test if the git will actually filter this from /target/ dir
// Update, it didn't
struct BitBoard {
	// White pieces
	white_pawns: u64,
	white_knights: u64,
	white_bishops: u64,
	white_rooks: u64,
	white_queens: u64,
	white_kings: u64,

	// Black pieces
	black_pawns: u64,
	black_knights: u64,
	black_bishops: u64,
	black_rooks: u64,
	black_queens: u64,
	black_kings: u64,

	// Side to move
	side_to_move: Color,
}