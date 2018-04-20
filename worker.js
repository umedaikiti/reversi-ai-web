var state = {last: null, turn: 1, board: Array(8), valid_move: Array(8)};
for(let i=0;i<8;i++) state.board[i] = Array(8).fill(0);
for(let i=0;i<8;i++) state.valid_move[i] = Array(8).fill(false);
var exports;
function update_board() {
	for(var i=0;i<8;i++) for(var j=0;j<8;j++) {
		state.board[i][j] = exports.get_board(i, j);
	}
}
function update_valid_move() {
	for(var i=0;i<8;i++) for(var j=0;j<8;j++) {
		state.valid_move[i][j] = exports.is_valid_move(i, j, state.turn);
	}
}
function next(turn) {
	if(exports.has_valid_moves(turn)) {
		return turn;
	} else if(exports.has_valid_moves(-turn)) {
		return -turn;
	} else {
		return 0;
	}
}
function reset() {
	state.last = null;
	state.turn = 1;
	exports.reset();
	update_board();
	update_valid_move();
}
function set_disk(x, y) {
	if(exports.set_disk(x, y, state.turn)) {
		console.log(`${state.turn === 1 ? "You" : "AI"} placed a disk at (${x}, ${y}).`);
		state.last = {x: x, y: y};
		state.turn = next(-state.turn);
		update_board();
		update_valid_move();
		return true;
	} else {
		return false;
	}
}

const imports = {
	env: {
		rand: Math.random,
	}
};
fetch('reversi.wasm').then(response =>
  response.arrayBuffer()
).then(bytes =>
  WebAssembly.instantiate(bytes, imports)
).then(result =>
	{
		console.log('Worker: initialize');
		exports = result.instance.exports;
		onmessage = function(e) {
			if(e.data.type === 'reset') {
				console.log('reset');
				reset();
				postMessage(state);
			} else if(e.data.type === 'move') {
				console.log('move');
				if(set_disk(e.data.x, e.data.y)) {
					postMessage(state);
					while(state.turn === -1) {
						var r = exports.ai_think(-1);
						if(r >= 0) {
							var x = r >> 3;
							var y = r & 0x7;
							if(!set_disk(x, y)) {
								state.turn = 0;
								console.log('BUG');
								throw 'BUG';
							}
							postMessage(state);
						} else {
							state.turn = 0;
							console.log('BUG');
							throw 'BUG';
						}
					}
				} else {
					console.log('invalid move');
					postMessage(state); // activate onclick
				}
			}
		};
		reset();
		postMessage(state);
	}
);
