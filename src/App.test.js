import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';

it('renders without crashing', () => {
  const div = document.createElement('div');
	const handleClick = (i, j) => { return e => {}; };
	const reset = e => {};
  let board = [
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,1,-1,0,0,0],
    [0,0,0,-1,1,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0]
  ];
  let valid_move = [
    [true, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false]
  ];
  let state = {last: null, turn: 1, board: board, valid_move: valid_move};
  const info = 'test';
  ReactDOM.render(<App handleClick={handleClick} reset={reset} data={state} info={info} />, div);
  state.last = {x: 1, y: 1};
  state.turn = -1;
  ReactDOM.render(<App handleClick={handleClick} reset={reset} data={state} info={info} />, div);
  ReactDOM.unmountComponentAtNode(div);
});
