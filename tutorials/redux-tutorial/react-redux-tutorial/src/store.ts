import { createStore } from 'redux';
import rotateReducer, { RotateState } from './modules/rotate';

const initialState: RotateState = {
  rotating: true,
}

const configureStore = (state: RotateState = initialState) => {
  return createStore(rotateReducer, state);
};

export default configureStore;
