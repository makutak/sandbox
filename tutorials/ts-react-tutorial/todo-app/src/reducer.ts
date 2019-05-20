import { combineReducers, Reducer } from 'redux';
import { TodoActionType, TodoAction, AddTodoAction } from './action';
import { RootState, TodoState } from './store';

const initTodoState = {
  todos: [],
};

const todoReducer: Reducer<TodoState> = (
  state: TodoState = initTodoState,
  action: AddTodoAction,
): TodoState => {
  switch (action.type) {
    case TodoActionType.ADD_TODO:
      const addTodoAction: AddTodoAction = action;
      return {
        ...state,
        todos: state.todos.concat([addTodoAction.payload.todo]),
      };
    default:
      return state;
  }
};

const reducer: Reducer<RootState> = combineReducers({
  todoState: todoReducer,
});

export default reducer;
