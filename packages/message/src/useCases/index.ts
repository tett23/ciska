import { ApiActions, ApiRequest, ApiResponse } from '../messages';

export async function useCases<T extends ApiActions>(
  action: T,
  arg: ApiRequest<T>,
): Promise<ApiResponse<T>> {
  const act: ApiActions = action;
  console.info('call useCase', action, arg);
  switch (act) {
    case 'addNewProject':
      return {};
    // return await listNovels(arg as ApiRequest<typeof act>);
  }
}
