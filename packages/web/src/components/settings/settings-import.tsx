import { Pane } from "evergreen-ui";
import React from "react";
import { ImportWidget } from "../widgets/import-widget";

export type SettingsImportProps = {
  onImportFromExistingSolution?: () => void;
};

export const SettingsImport: React.FunctionComponent<SettingsImportProps> = ({
  onImportFromExistingSolution,
}) => {
  return (
    <Pane display="flex" flex={1}>
      <ImportWidget
        onImportFromExistingSolutionClick={onImportFromExistingSolution}
      />
    </Pane>
  );
};
