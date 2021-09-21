import { Heading, Pane } from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { NumberHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { AmountLabel, AmountLabelType } from "../common/amount-label";
import { VariationComponent } from "../variation/variation-component";

const _ = translate("");

export type RentManagerSummaryProps = {
  loading?: boolean;
  occupationRate?: number;
  paymentRate?: number;
  amountReceived?: number;
  amountPending?: number;
  nPartial?: number;
  nExpected?: number;
  nReceived?: number;
  variationReceived?: number;
};

export const RentManagerSummary: React.FunctionComponent<
  RentManagerSummaryProps
> = ({
  occupationRate,
  paymentRate,
  amountReceived = 0,
  amountPending,
  nPartial,
  nExpected,
  nReceived,
  variationReceived,
}) => {
  const theme = useAppTheme();
  const pendingAmount = Number(amountPending || 0);
  const accentColor = theme.palette.blue.base;
  return (
    <Pane borderBottom="muted" marginBottom={theme.margin.medium}>
      <Pane display="flex" flex={1} minHeight={80}>
        <Pane
          flex={1}
          display="flex"
          flexDirection="column"
          borderRight="muted"
          padding={theme.margin.large}
          alignItems="center"
          justifyContent="space-around"
        >
          <Heading size={100}>{_("occupation_rate")}</Heading>
          <Heading size={700} color={accentColor}>
            {occupationRate
              ? NumberHelper.formatToPercentage(occupationRate)
              : "-"}
          </Heading>
        </Pane>
        <Pane
          flex={1}
          display="flex"
          flexDirection="column"
          borderRight="muted"
          padding={theme.margin.large}
          alignItems="center"
          justifyContent="space-around"
        >
          <Heading size={100}>{_("incomes")}</Heading>
          <Pane display="flex" alignItems="center">
            <Heading size={800} color={accentColor}>
              {NumberHelper.formatToString(amountReceived || 0, false)}
            </Heading>
            <VariationComponent value={variationReceived} />
          </Pane>
        </Pane>
        <Pane
          flex={1}
          display="flex"
          flexDirection="column"
          padding={theme.margin.large}
          alignItems="center"
          justifyContent="space-around"
        >
          <Heading size={100}>{_("payment_rate")}</Heading>
          <Heading size={700} color={accentColor}>
            {paymentRate ? NumberHelper.formatToPercentage(paymentRate) : "-"}
          </Heading>
        </Pane>
      </Pane>
      <Pane display="flex" flex={1} minHeight={80} borderTop="muted">
        {/* Loyers perçus */}
        <Pane
          flex={1}
          display="flex"
          flexDirection="column"
          padding={theme.margin.large}
          alignItems="center"
          justifyContent="space-around"
          borderRight="muted"
        >
          <Heading size={100}>{_("rent_received")}</Heading>
          <Pane display="flex" alignItems="center" justifyContent="center">
            <Heading size={700} color={accentColor}>
              {`${nReceived}/${nExpected}` || "-"}
            </Heading>
          </Pane>
        </Pane>

        {/* Loyers partiellement payés */}
        <Pane
          flex={1}
          display="flex"
          flexDirection="column"
          borderRight="muted"
          padding={theme.margin.large}
          alignItems="center"
          justifyContent="space-around"
        >
          <Heading size={100}>{_("rent_partial")}</Heading>
          <Pane display="flex" alignItems="center">
            <Heading size={700} color={accentColor}>
              {nPartial || "-"}
            </Heading>
          </Pane>
        </Pane>

        {/* Loyers en attente */}
        <Pane
          flex={1}
          display="flex"
          flexDirection="column"
          padding={theme.margin.large}
          alignItems="center"
          justifyContent="space-around"
        >
          <Heading size={100}>{_("rent_pending")}</Heading>
          <Pane display="flex" alignItems="center" justifyContent="center">
            <AmountLabel
              type={AmountLabelType.BIG}
              value={pendingAmount}
              color={pendingAmount > 0 ? theme.palette.red.dark : accentColor}
            />
          </Pane>
        </Pane>
      </Pane>
    </Pane>
  );
};
