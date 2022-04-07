import React, { useCallback } from 'react';
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
  const openWindow = useOpenWindow();

  const dialog = useOpenDialog();
  const onClick = async () => {
    const paths = await dialog();
    console.log(paths);
    await mes('addNewProject', {});
    await openWindow();
  };

  return (
    <div>
      <button onClick={onClick}>new</button>
    </div>
  );
}

function useOpenDialog() {
  return useCallback((): Promise<string[]> => {
    return global.api.openDialog({ properties: ['openDirectory'] });
  }, []);
}

function useOpenWindow() {
  return useCallback(() => {
    global.api.openNewWindow({});
  }, []);
}
