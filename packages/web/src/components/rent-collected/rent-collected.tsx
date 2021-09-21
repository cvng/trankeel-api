import { Card, defaultTheme, Heading, Pane } from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { NumberHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { AmountLabel } from "../common/amount-label";

const _ = translate("");

export type RentCollectedProps = {
  pendingRentAmount?: number;
  totalRentAmount?: number;
};

export const RentCollected: React.FunctionComponent<
  RentCollectedProps
> = ({
  pendingRentAmount,
  totalRentAmount,
}) => {
  const theme = useAppTheme();
  return (
    <Card
      flex={1}
      display="flex"
      flexDirection="column"
      justifyContent="space-between"
      background={defaultTheme.palette.blue.lightest}
      padding={theme.margin.medium}
      maxWidth={200}
      border={"muted"}
    >
      <Pane
        display="flex"
        flexDirection="row"
        alignItems="flex-end"
        justifyContent="space-between"
      >
        <Heading size={100}>{`${_("cashin")} :`}</Heading>
        <AmountLabel
          marginLeft={theme.margin.small}
          value={totalRentAmount || 0}
        />
      </Pane>
      <Pane
        display="flex"
        flexDirection="row"
        alignItems="flex-end"
        justifyContent="space-between"
      >
        <Heading size={100}>{`${_("pending")} :`}</Heading>
        <Heading
          size={300}
          color={pendingRentAmount > 0
            ? defaultTheme.palette.red.dark
            : defaultTheme.palette.neutral.dark}
          marginLeft={theme.margin.small}
        >
          {totalRentAmount
            ? NumberHelper.formatToString(pendingRentAmount, false)
            : "-"}
        </Heading>
      </Pane>
    </Card>
  );
};
