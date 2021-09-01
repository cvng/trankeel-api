import { useRouteMatch } from "react-router-dom";

export const useParamId = (
  route: string,
  idKey?: string,
): string | undefined => {
  return useRouteMatch(route)?.params?.[idKey ?? "id"];
};
