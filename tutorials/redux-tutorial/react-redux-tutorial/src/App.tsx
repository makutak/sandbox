import * as React from 'react';
import { Dispatch, Action } from 'redux';
import { connect } from 'react-redux';
import { startAction, stopAction, RotateState } from './modules/rotate';

import logo from './logo.svg';
import './App.css';

interface DispatchToProps {
  startAction: () => void;
  stopAction: () => void;
}

type Props = RotateState & DispatchToProps;

class App extends React.Component<Props, {}> {
  render(): JSX.Element {
    console.log(JSON.stringify(this.props, null, "  "));
    return (
      <div className="App">
        <header className="App-header">
          <img
            src={logo}
            className={
              "App-logo" +
              (this.props.rotating ? "" : " App-logo-paused")
            }
            alt="logo"
            onClick={
              this.props.rotating ?
                this.props.stopAction : this.props.startAction
            } />
          <p>
            Edit <code>src/App.tsx</code> and save to reload.
        </p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
        </a>
        </header>
      </div>
    );
  }
}

const mapStateToDispatch = (state: RotateState) => ({
  ...state,
});

const mapDispatchToProps = (dispatch: Dispatch<Action>) => ({
  startAction: () => dispatch(startAction),
  stopAction: () => dispatch(stopAction),
});

export default connect(mapStateToDispatch, mapDispatchToProps)(App);
