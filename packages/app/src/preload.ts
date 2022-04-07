/* eslint-disable @typescript-eslint/no-namespace */
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { ipcRenderer, contextBridge, IpcRenderer, dialog } from 'electron';
import { ApiActions, ApiRequest, ApiResponse } from '@ciska/message/messages';

declare global {
  var ipcRenderer: IpcRenderer;
  var api: typeof Api;
}

// Since we disabled nodeIntegration we can reintroduce
// needed node functionality here
process.once('loaded', () => {
  global.ipcRenderer = ipcRenderer;
});

const Api = {
  message: async <T extends ApiActions>(
    type: T,
    arg: ApiRequest<T>,
  ): Promise<ApiResponse<T>> => ipcRenderer.invoke('message', [type, arg]),
  openDialog: async (options: Parameters<typeof dialog.showOpenDialog>[0]) =>
    ipcRenderer.invoke('openDialog', options),
  openNewWindow: async (options: {}) =>
    ipcRenderer.invoke('openNewWindow', options),
  on: (channel: any, callback: any) => ipcRenderer.on(channel, callback),
};

contextBridge.exposeInMainWorld('api', Api);
