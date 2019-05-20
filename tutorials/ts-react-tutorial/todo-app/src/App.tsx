import React from 'react';
import './App.css';
import TodoComponent from './TodoComponent';

const App: React.FC = () => {
  return (
    <div>
      <TodoComponent
        onClickAddButton={(todo: string): void => {
          console.log(todo);
        }}
      />
    </div>
  );
}

export default App;
