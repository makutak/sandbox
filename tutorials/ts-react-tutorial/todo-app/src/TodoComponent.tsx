import * as React from 'react';

interface Props {
  todos: string[];
  onClickAddButton: (todo: string) => void;
}

interface State {
  text: string;
}

export default class extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      text: '',
    };
  }

  public render() {
    const { todos } = this.props;
    const { text } = this.state;

    return (
      <div style={{ width: '500px', margin: '0 auto' }}>
        <h1>TODO LIST</h1>
        <input type="text" value={text} onChange={this.onTextChange} />
        <button onClick={this.onClickAddButton}>Add Todo</button>
        <ul>
          {todos.map((todo, index) => (
            <li key={index}>{todo}</li>
          ))}
        </ul>
      </div>
    );
  }

  // changeEventのたびに実行される
  private onTextChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    // stateのtextにvalueを入れる
    this.setState({ text: e.currentTarget.value });
    console.log("state: ", this.state);
  };

  // buttonClick時の処理
  private onClickAddButton = () => {
    console.log("clicked!!: ", this.state);
    // clickされたときのstate.textの値をthis.stateにいれる
    // todosは初期は空配列
    const { onClickAddButton } = this.props;
    const { text } = this.state;
    onClickAddButton(text);
  };
}
