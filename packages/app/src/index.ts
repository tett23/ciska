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
} from 'electron';

app.on('ready', async () => {
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      nativeWindowOpen: true,
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
  console.log({ a: indexPath });
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

// // listen the channel `message` and resend the received message to the renderer process
// ipcMain.on('message', (event: IpcMainEvent, message: any) => {
//   console.log(message);
//   setTimeout(() => event.sender.send('message', 'hi from electron'), 500);
// });

// ipcMain.handle(
//   'message',
//   async <T extends ApiActions>(
//     _: IpcMainInvokeEvent,
//     [action, arg]: [T, ApiRequest<T>],
//   ): Promise<ApiResponse<T>> => useCases(action, arg),
// );
