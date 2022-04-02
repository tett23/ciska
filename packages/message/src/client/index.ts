import { useCallback } from 'react';
import useSwr from 'swr';
import { ApiActions, ApiRequest, ApiResponse } from '../messages';

export function useFetcher() {
  const mes = useElectronMessage();
  const b = useCallback(
    (...[a, b]: Parameters<ReturnType<typeof useElectronMessage>>) => {
      const result = mes(a, b);
      if (result instanceof Error) {
        console.error(result);
        return result;
      }

      return result;
    },
    [],
  );

  return useSwr('', b, {});
}

export function useUpdater() {
  const mes = useElectronMessage();

  return (...[a, b]: Parameters<ReturnType<typeof useElectronMessage>>) => {
    return mes(a, b);
  };
}

function useElectronMessage() {
  return useCallback(
    <T extends ApiActions>(
      action: T,
      arg: ApiRequest<T>,
    ): Promise<ApiResponse<T>> => {
      return global.api.message(action, arg);
    },
    [],
  );
}
