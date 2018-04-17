import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import registerServiceWorker from './registerServiceWorker';

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
	const reset = e => {
		e.preventDefault();
		if(window.confirm('Are you sure you want to start a new game?')) {
			worker.postMessage({type: 'reset'});
		}
	};

	worker.onmessage = e => {
		console.log('update');
		var info = '';
		if(e.data.turn === 1) info += 'Your turn.';
		else if(e.data.turn === -1) info += "AI's turn.";
		else {
			var p = 0;
			var o = 0;
			for(var i=0;i<8;i++) for(var j=0;j<8;j++) {
				if(e.data.board[i][j] === 1) p++;
				if(e.data.board[i][j] === -1) o++;
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
		ReactDOM.render(<App handleClick={handleClick} reset={reset} data={e.data} info={info} />, document.getElementById('root'));
		if(e.data.turn === 1) clickValid = true;
	}
	registerServiceWorker();
}
