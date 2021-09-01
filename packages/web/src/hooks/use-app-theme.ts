import { useTheme } from "evergreen-ui";
import { AppTheme } from "../theme/app-theme";

export const useAppTheme = (): typeof AppTheme => {
  return useTheme() as typeof AppTheme;
};
