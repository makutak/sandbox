import { Action } from 'redux';

export enum TodoActionType {
  ADD_TODO = 'ADD_TODO',
}

export interface AddTodoAction extends Action {
  type: TodoActionType.ADD_TODO;
  // payload: アクションの実行に必要な任意のデータ
  payload: {
    todo: string,
  };
}

export type TodoAction = AddTodoAction;

export interface ITodoActionCreator {
  addTodoAction(todo: string): AddTodoAction;
}

class TodoActionCreator implements ITodoActionCreator {
  public addTodoAction = (todo: string): AddTodoAction => {
    return {
      payload: {
        todo,
      },
      type: TodoActionType.ADD_TODO,
    };
  };
}

export const todoActionCreator: ITodoActionCreator = new TodoActionCreator();
