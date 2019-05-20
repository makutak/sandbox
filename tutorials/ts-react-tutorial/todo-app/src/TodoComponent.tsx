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

  public render () {
    const { todos } = this.props;
    const { text } = this.state;

    return (
      <div>
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

  private onTextChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ text: e.currentTarget.value });
  };

  private onClickAddButton = () => {
    const { onClickAddButton } = this.props;
    const { text } = this.state;
    onClickAddButton(text);
  };
}
