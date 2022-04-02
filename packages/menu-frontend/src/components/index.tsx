import React from 'react';
import { useUpdater } from '@ciska/message/client';

export function App() {
  return (
    <div>
      <div>recent</div>
      <AddNewProject />
    </div>
  );
}

function AddNewProject() {
  const mes = useUpdater();
  const onClick = () => {
    mes('addNewProject', {});
  };

  return (
    <div>
      <button onClick={onClick}>new</button>
    </div>
  );
}
