let game = null;
let flipped = false;
let selectedSquare = null;
let lastMove = null; // [fromIdx, toIdx]

let wasmModule = null;

const menuEl = document.getElementById("menu");
const gameEl = document.getElementById("game");
const resultEl = document.getElementById("result");
const resultMessageEl = document.getElementById("resultMessage");
const mainEl = document.getElementById("main");

const pieceToImg = {
	P: "assets/wpawn.png",
	N: "assets/wknight.png",
	B: "assets/wbishop.png",
	R: "assets/wrook.png",
	Q: "assets/wqueen.png",
	K: "assets/wking.png",
	p: "assets/bpawn.png",
	n: "assets/bnight.png",
	b: "assets/bbishop.png",
	r: "assets/brook.png",
	q: "assets/bqueen.png",
	k: "assets/bking.png",
};

async function ensureGame() {
	if (game) return;
	const wasm = await ensureWasm();
	game = new wasm.WasmGame();
}

async function ensureWasm() {
	if (wasmModule) return wasmModule;
	try {
		// wasm-pack generates an ES module with a default `init()` export.
		const m = await import("./pkg/chess_engine.js");
		await m.default();
		wasmModule = m;
		return wasmModule;
	} catch (e) {
		// If `web_thing/pkg/` is missing, a static import would make the whole page blank.
		// With a dynamic import we can show a clear error.
		throw new Error(
			"WASM package not found. Build it first from the chess_engine folder:\n\n" +
			"  wasm-pack build --target web --out-dir ../web_thing/pkg\n\n" +
			"Or in VS Code: Run Task → dev / wasm:build"
		);
	}
}

function showScene(scene) {
	// 0 = menu, 1 = game, 2 = result
	menuEl.style.display = scene === 0 ? "block" : "none";
	gameEl.style.display = scene === 1 ? "block" : "none";
	resultEl.style.display = scene === 2 ? "block" : "none";
}

function idxToSquare(idx) {
	const file = idx % 8;
	const rank = Math.floor(idx / 8) + 1;
	return String.fromCharCode(97 + file) + String(rank);
}

function parseFenBoard(fen) {
	const boardPart = fen.split(" ")[0];
	const ranks = boardPart.split("/");
	if (ranks.length !== 8) throw new Error("Invalid FEN");

	const board = new Array(64).fill(null);
	for (let fenRank = 0; fenRank < 8; fenRank++) {
		const rankStr = ranks[fenRank];
		let file = 0;
		for (const ch of rankStr) {
			if (ch >= "1" && ch <= "8") {
				file += Number(ch);
				continue;
			}
			const boardRank = 7 - fenRank; // fen starts at rank 8
			const idx = boardRank * 8 + file;
			board[idx] = ch;
			file += 1;
		}
		if (file !== 8) throw new Error("Invalid FEN rank");
	}
	return board;
}

function render() {
	if (!game) return;
	const fen = game.fen();
	const board = parseFenBoard(fen);
	mainEl.replaceChildren();

	const rankIter = flipped ? [0, 1, 2, 3, 4, 5, 6, 7] : [7, 6, 5, 4, 3, 2, 1, 0];
	const fileIter = flipped ? [7, 6, 5, 4, 3, 2, 1, 0] : [0, 1, 2, 3, 4, 5, 6, 7];

	for (const rank of rankIter) {
		for (const file of fileIter) {
			const idx = rank * 8 + file;
			const square = document.createElement("div");
			square.classList.add("square");
			square.classList.add((rank + file) % 2 === 0 ? "white" : "brown");
			square.dataset.idx = String(idx);
			square.dataset.square = idxToSquare(idx);

			if (selectedSquare === idx) {
				square.classList.add("yellow");
			} else if (lastMove && (lastMove[0] === idx || lastMove[1] === idx)) {
				square.classList.add("blue");
			}

			const piece = board[idx];
			if (piece) {
				const img = document.createElement("img");
				img.alt = piece;
				img.src = pieceToImg[piece] ?? "";
				square.appendChild(img);
			}

			square.addEventListener("click", () => onSquareClick(idx, board));
			mainEl.appendChild(square);
		}
	}

	// Check for checkmate/stalemate
	const status = game.game_status();
	if (status === "checkmate") {
		const loser = fen.split(" ")[1] === "w" ? "White" : "Black";
		const winner = loser === "White" ? "Black" : "White";
		EndGame(`Checkmate! ${winner} wins!`);
	} else if (status === "stalemate") {
		EndGame("Stalemate - Draw!");
	} else if (status === "check") {
		console.log("Check!");
	}
}

function onSquareClick(idx, board) {
	if (!game) return;

	if (selectedSquare === null) {
		if (!board[idx]) return;
		selectedSquare = idx;
		render();
		return;
	}

	if (selectedSquare === idx) {
		selectedSquare = null;
		render();
		return;
	}

	const uci = idxToSquare(selectedSquare) + idxToSquare(idx);
	try {
		game.make_move_uci(uci);
		lastMove = [selectedSquare, idx];
		console.log("Move applied:", uci, "Status:", game.game_status());
	} catch (e) {
		// wasm-bindgen throws a JS Error for Result::Err
		alert(e?.message ?? String(e));
	} finally {
		selectedSquare = null;
		render();
	}
}

// Expose the HTML onclick handlers.
window.RenderScene = (scene) => showScene(scene);

window.StartGame = async () => {
	try {
		await ensureGame();
		selectedSquare = null;
		lastMove = null;
		showScene(1);
		render();
	} catch (e) {
		alert(e?.message ?? String(e));
	}
};

window.Rematch = async () => {
	try {
		await ensureGame();
		game.reset();
		selectedSquare = null;
		lastMove = null;
		showScene(1);
		render();
	} catch (e) {
		alert(e?.message ?? String(e));
	}
};

window.EndGame = (message) => {
	resultMessageEl.textContent = message;
	showScene(2);
};

window.FlipBoard = () => {
	flipped = !flipped;
	render();
};

// Initial state
showScene(0);
