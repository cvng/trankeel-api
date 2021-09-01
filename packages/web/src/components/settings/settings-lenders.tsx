import { Pane } from "evergreen-ui";
import React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Lender } from "../../types";
import { LenderDetails, LenderHeader, LenderList } from "../lender-synthesis";
import { LenderHeaderProps } from "../lender-synthesis/lender-header";
import { LenderListProps } from "../lender-synthesis/lender-list";

export type SettingsLendersProps = {
  loading: boolean;
  selectedLender?: Lender;
  lenderHeaderProps: LenderHeaderProps;
  lenderListProps: LenderListProps;
};

export const SettingsLenders: React.FunctionComponent<
  SettingsLendersProps
> = (
  {
    loading,
    selectedLender,
    lenderHeaderProps,
    lenderListProps,
  },
) => {
  const theme = useAppTheme();
  return (
    <Pane
      display="flex"
      flex={1}
      background={theme.colors.background.tint1}
      padding={theme.margin.largest}
    >
      <LenderList
        {...lenderListProps}
      />

      <Pane flex={1} marginLeft={theme.margin.largest}>
        {selectedLender &&
          <LenderHeader {...lenderHeaderProps} />}

        <LenderDetails loading={loading} lender={selectedLender} />
      </Pane>
    </Pane>
  );
};
