/* eslint-disable @typescript-eslint/no-namespace */
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { ipcRenderer, contextBridge, IpcRenderer } from 'electron';
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
  on: (channel: any, callback: any) => ipcRenderer.on(channel, callback),
};

contextBridge.exposeInMainWorld('api', Api);
