var board = Array(8);
for(let i=0;i<8;i++) board[i] = Array(8).fill(0);
var valid_move = Array(8);
for(let i=0;i<8;i++) valid_move[i] = Array(8).fill(false);
var exports;
function update_board() {
	for(var i=0;i<8;i++) for(var j=0;j<8;j++) {
		board[i][j] = exports.get_board(i, j);
	}
}
function update_valid_move(turn) {
	for(var i=0;i<8;i++) for(var j=0;j<8;j++) {
		valid_move[i][j] = exports.is_valid_move(i, j, turn);
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
fetch('reversi.wasm').then(response =>
  response.arrayBuffer()
).then(bytes =>
  WebAssembly.instantiate(bytes, {})
).then(result =>
	{
		console.log('Worker: initialize');
		exports = result.instance.exports;
		var last = null;
		onmessage = function(e) {
			var turn = 1;
			if(e.data.type === 'reset') {
				console.log('reset');
				exports.reset();
				update_board();
				update_valid_move(turn);
				last = null;
				postMessage({turn: turn, board: board, last: last, valid_move: valid_move});
			} else if(e.data.type === 'move') {
				console.log('move');
				if(exports.set_disk(e.data.x, e.data.y, turn)) {
					update_board();
					turn = next(-1);
					update_valid_move(turn);
					last = {x: e.data.x, y: e.data.y};
					console.log(`You placed a disk at (${last.x}, ${last.y}).`);
					postMessage({turn: turn, board: board, last: last, valid_move: valid_move});
					while(turn === -1) {
						var r = exports.ai_think(-1);
						if(r >= 0) {
							var x = r >> 3;
							var y = r & 0x7;
							exports.set_disk(x, y, turn);
							update_board();
							turn = next(1);
							update_valid_move(turn);
							last = {x: x, y: y};
							console.log(`AI placed a disk at (${last.x}, ${last.y}).`);
							postMessage({turn: turn, board: board, last: last, valid_move: valid_move});
						} else {
							turn = 0;
							console.log('BUG');
						}
					}
				} else {
					console.log('invalid move');
					postMessage({turn: turn, board: board, last: last, valid_move: valid_move});
				}
			}
		};
		exports.reset();
		update_board();
		update_valid_move(1);
		postMessage({turn: 1, board: board, last: null, valid_move: valid_move});
	}
);
