export type ApiMessage = {
  addNewProject: [{}, {}];
};

export type ApiActions = keyof ApiMessage;
export type ApiRequest<T extends ApiActions> = ApiMessage[T][0];
export type ApiResponse<T extends ApiActions> = ApiMessage[T][1] | Error;
