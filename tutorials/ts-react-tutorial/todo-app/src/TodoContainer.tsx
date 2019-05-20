import * as React from 'react';
import { connect } from 'react-redux';
import { Action, Dispatch } from 'redux';
import { todoActionCreator } from './action';
import { RootState } from './store';
import TodoComponent from './TodoComponent';

// redux の store を container の propsに変換する interface
interface StateToProps {
  todos: string[];
}

// redux の dispatch を propsに変換する interface
// メンバにはどのアクションを実行するのかを定義する
interface DispatchToProps {
  addTodo: (todo: string) => void;
}

// StateToProps と DispatchToProps の複合型
type Props = StateToProps & DispatchToProps;

// Props を受け取る container を定義
// Stateは持たないので、空の定義を渡す
class TodoContainer extends React.Component<Props, {}> {
  constructor(props: Props) {
    super(props);
  }

  public render(): JSX.Element {
    const { todos } = this.props;

    return (
      <TodoComponent todos={todos} onClickAddButton={this.onClickAddButton} />
    );
  }

  private onClickAddButton = (todo: string): void => {
    console.log("click");
    const { addTodo } = this.props;
    console.log(todo);
    addTodo(todo);
  };
}

const mapStateToProps = (state: RootState): StateToProps => {
  const { todoState } = state;
  return {
    todos: todoState.todos
  };
};

const mapDispatchToProps = (dispatch: Dispatch<Action>): DispatchToProps => {
  return {
    addTodo: (todo: string) => {
      dispatch(todoActionCreator.addTodoAction(todo));
    }
  };
};

export default connect<IStateToProps, IDispatchToProps>(
  mapStateToProps,
  mapDispatchToProps
)(TodoContainer);
