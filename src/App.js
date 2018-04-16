import React, { Component } from 'react';
import './App.css';

class App extends Component {
	renderDisk = (c, i, j) => {
		var cls = ['disk'];
		if(c === 1) {
			cls.push('player');
		} else if(c === -1) {
			cls.push('opponent');
		} else {
			cls.push('hidden');
		}
		if(this.props.data.turn === 1 && this.props.data.valid_move[i][j]) {
			cls.push('valid-move');
		}
		return (<div className={cls.join(' ')} id={i + '-' + j}></div>);
	}
	renderSquare = (i, j) => {
		var cls = "row";
		if(this.props.data.last !== null && i === this.props.data.last.x && j === this.props.data.last.y) cls += " last-move";
	  return (
		<div className={cls} key={'row' + i + '-' + j} onClick={this.props.handleClick(i, j)}>
		  {this.renderDisk(this.props.data.board[i][j], i, j)}
		</div>
	  );
  }
  renderCol(i) {
	  var list = [];
	  var j;
		for(j=0;j<8;j++) list.push(this.renderSquare(i, j));
	  return (
		<div  key={'col' + i} className="col">
		  {list}
		</div>
	  );
  }
  render() {
	  var list = [];
	  var i;
	  for(i=0;i<8;i++) list.push(this.renderCol(i));
    return (
      <div className="App">
		<div className="board">
		{list}
		</div>
		<input type="button" value="reset" onClick={this.props.reset} />
		<div id="info">{this.props.info}</div>
      </div>
    );
  }
}

export default App;
