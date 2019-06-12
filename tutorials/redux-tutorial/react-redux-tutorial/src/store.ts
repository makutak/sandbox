import { createStore } from 'redux';
import rotateReducer, { RotateState, initialState } from './modules/rotate';

const configureStore = (state: RotateState = initialState) => {
  return createStore(rotateReducer, state);
};

export default configureStore;
