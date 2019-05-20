import { Action, createStore, Store } from 'redux';
import reducer from './reducer';

export interface TodoState {
  todos: string[];
}

export interface RootState {
  todoState: TodoState;
}

const store: Store<TodoState, Action> = createStore(reducer);

export default store;
