import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import registerServiceWorker from './registerServiceWorker';

function info_string(data) {
	var info = '';
	if(data.turn === 1) info += 'Your turn.';
	else if(data.turn === -1) info += "AI's turn.";
	else {
		var p = 0;
		var o = 0;
		for(var i=0;i<8;i++) for(var j=0;j<8;j++) {
			if(data.board[i][j] === 1) p++;
			if(data.board[i][j] === -1) o++;
		}
		info += 'Game over. ';
		if(p > o) {
			info += 'You win!';
		} else if (p < o) {
			info += 'You lose!';
		} else {
			info += 'Draw.';
		}
		info += ` (${p} vs ${o})`;
	}
	return info;
}

function random_player() {
	let ary = document.getElementsByClassName("valid-move");
	ary[Math.floor(Math.random() * ary.length)].dispatchEvent(new MouseEvent('click', {
		bubbles: true,
		cancelable: true,
		view: window
	}));
}

var debug = window.location.search === '?debug';

if(window.Worker) {
	var worker = new Worker("worker.js");
	//	worker.postMessage({type: 'reset'});
	var clickValid = false;
	const handleClick = (i, j) => {
		return e => {
			e.preventDefault();
			console.log(`clicked: (${i}, ${j})`);
			if(clickValid) {
				worker.postMessage({type: 'move', x: i, y: j});
				clickValid = false;
			} else {
				console.log('canceled');
			}
		};
	};
	var q = [];
	var q_wait = false;
	var timer = null;
	const reset = e => {
		e.preventDefault();
		if(window.confirm('Are you sure you want to start a new game?')) {
			worker.postMessage({type: 'reset'});
		}
	};

	function update(data) {
		console.log('update');
		var info = info_string(data);
		ReactDOM.render(<App handleClick={handleClick} reset={reset} data={data} info={info} />, document.getElementById('root'), function() {
			if(data.turn === 1) {
				clickValid = true;
				if(debug) random_player();
			}
			if(data.turn === 0) {
				console.log(info);
				if(debug) {
					setTimeout(() => worker.postMessage({type: 'reset'}), 3000);
				}
			}
		});
		q_wait = true;
		timer = setTimeout(() => {
			timer = null;
			if(q.length > 0) {
				let data = q.shift();
				update(data);
			} else {
				q_wait = false;
			}
		}, 500);
	}
	worker.onmessage = e => {
		if(e.data.last === null) {
			q_wait = false;
			q = [];
			if(timer !== null) {
				clearTimeout(timer);
				timer = null;
			}
		}
		if(q_wait) q.push(e.data);
		else update(e.data);
	}
	registerServiceWorker();
}
