import React from "react";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import { SettingsImportProps } from "./settings-import";

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();

    const handleImportFromExistingSolutionClick = () => {
      history.push(Routes.SETTINGS_IMPORT);
    };

    const componentProps: SettingsImportProps = {
      onImportFromExistingSolution: handleImportFromExistingSolutionClick,
    };

    return WrappedComponent(componentProps);
  };
