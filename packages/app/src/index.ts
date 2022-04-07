import { join } from 'path';
import { format } from 'url';
import {
  BrowserWindow,
  app,
  ipcMain,
  IpcMainEvent,
  IpcMainInvokeEvent,
  protocol,
  shell,
  dialog,
} from 'electron';
import { ApiActions, ApiRequest, ApiResponse } from '@ciska/message/messages';
import { useCases } from '@ciska/message/useCases';

app.on('ready', async () => {
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      // nativeWindowOpen: true,
      nodeIntegrationInWorker: true,
      preload: join(__dirname, 'preload.js'),
    },
  });
  mainWindow.webContents.openDevTools();
  protocol.interceptFileProtocol('file', (req, callback) => {
    callback(req.url.replace(/^file:\/\//, ''));
  });

  const indexPath = join(
    __dirname,
    '..',
    '..',
    '..',
    'menu-frontend',
    'lib',
    'cjs',
    'index.html',
  );
  mainWindow.loadURL(
    format({
      pathname: indexPath,
      protocol: 'file:',
      slashes: true,
    }),
  );

  mainWindow.webContents.on('new-window', (e, url) => {
    e.preventDefault();
    shell.openExternal(url);
  });

  ipcMain.emit('startPoll');
});

app.on('window-all-closed', app.quit);

ipcMain.handle(
  'message',
  async <T extends ApiActions>(
    _: IpcMainInvokeEvent,
    [action, arg]: [T, ApiRequest<T>],
  ): Promise<ApiResponse<T>> => {
    return useCases(action, arg);
  },
);

ipcMain.handle(
  'openDialog',
  async (
    _: IpcMainInvokeEvent,
    options: Parameters<typeof dialog.showOpenDialog>[0],
  ) => {
    const ret = await dialog.showOpenDialog(null, options);
    console.log(ret.filePaths);

    return ret.filePaths;
  },
);
