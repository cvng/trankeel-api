import {
  ArrowDownIcon,
  ArrowUpIcon,
  Card,
  defaultTheme,
  Heading,
  Icon,
  majorScale,
  minorScale,
  Pane,
  Text,
} from "evergreen-ui";
import React from "react";
import { EmptyItem } from "../../components/common";
import { TourType } from "../../constants/tour-constants";
import { NumberHelper } from "../../helpers";
import { AmountLabel, AmountLabelType } from "../common/amount-label";
import { translate } from "piteo-kit";

const _ = translate();

export type SynthesisWidgetProps = {
  cashflow?: number; // Cashflow mensuel
  cashflowVariation?: number; // Evolution du cashflow
  cashout?: number; // Charges décaissées
  cashoutVariation?: number; // Evolution des charges décaissées
  cashin?: number; // Loyer mensuel
  cashinVariation?: number; // Evolution des loyers
  receivedRentNb?: number; // Nb de loyers encaissé
  totalRentNb?: number; // Nb de loyers attendu,
  receivedRentPercentage?: number; // Pourcentage des loyers perçus
};

export const SynthesisWidget: React.FunctionComponent<SynthesisWidgetProps> = ({
  cashflow,
  cashflowVariation,
  cashout,
  cashoutVariation,
  cashin,
  cashinVariation,
  receivedRentNb,
  totalRentNb,
  receivedRentPercentage,
}) => {
  return (
    <Card
      className={TourType.DASHBOARD_SYNTHESIS}
      elevation={1}
      display="flex"
      background="white"
      height={120}
      padding={majorScale(2)}
    >
      {/* Encaissé ce mois-ci */}
      <Pane
        display="flex"
        flex={1}
        alignItems="center"
        flexDirection="column"
        borderRight="muted"
      >
        <Pane
          display="flex"
          flexDirection="column"
          flex={1}
          marginLeft={5}
          alignItems="center"
          justifyContent="center"
        >
          <Heading
            size={400}
            marginBottom={minorScale(2)}
            alignItems="right"
            color={defaultTheme.palette.neutral.base}
          >
            {_("rent_this_month")}
          </Heading>
          {NumberHelper.isInteger(cashin)
            ? (
              <Pane
                display="flex"
                flexDirection="row"
                justifyContent="center"
                alignItems="center"
              >
                <AmountLabel
                  type={AmountLabelType.BIG}
                  value={cashin}
                  addSign
                />
                {!!cashinVariation && (
                  <Pane marginX={majorScale(1)}>
                    <Icon
                      icon={cashinVariation > 0 ? ArrowUpIcon : ArrowDownIcon}
                      size={10}
                      color={cashinVariation > 0
                        ? defaultTheme.palette.green.dark
                        : defaultTheme.palette.red.dark}
                      marginX={2}
                    />
                    <Text
                      size={300}
                      color={cashinVariation > 0
                        ? defaultTheme.palette.green.dark
                        : defaultTheme.palette.red.dark}
                    >
                      {NumberHelper.formatToPercentage(cashinVariation)}
                    </Text>
                  </Pane>
                )}
              </Pane>
            )
            : (
              <EmptyItem />
            )}
        </Pane>
      </Pane>

      {/* Charges ce mois-ci */}
      <Pane
        display="flex"
        flex={1}
        alignItems="center"
        flexDirection="column"
        borderRight="muted"
      >
        <Pane
          display="flex"
          flexDirection="column"
          flex={1}
          marginLeft={5}
          alignItems="center"
          justifyContent="center"
        >
          <Heading
            size={400}
            marginBottom={minorScale(2)}
            alignItems="right"
            color={defaultTheme.palette.neutral.dark}
          >
            {_("charges_this_month")}
          </Heading>
          {NumberHelper.isInteger(cashout)
            ? (
              <Pane
                display="flex"
                flexDirection="row"
                justifyContent="center"
                alignItems="center"
              >
                <AmountLabel
                  type={AmountLabelType.BIG}
                  value={cashout}
                  addSign
                />
                {!!cashoutVariation && (
                  <Pane marginX={majorScale(1)}>
                    <Icon
                      icon={cashoutVariation > 0 ? ArrowUpIcon : ArrowDownIcon}
                      size={10}
                      color={cashoutVariation > 0
                        ? defaultTheme.palette.green.dark
                        : defaultTheme.palette.red.dark}
                      marginX={2}
                    />
                    <Text
                      size={300}
                      color={cashoutVariation > 0
                        ? defaultTheme.palette.green.dark
                        : defaultTheme.palette.red.dark}
                    >
                      {NumberHelper.formatToPercentage(cashoutVariation)}
                    </Text>
                  </Pane>
                )}
              </Pane>
            )
            : (
              <EmptyItem />
            )}
        </Pane>
      </Pane>

      {/* Cashflow mensuel */}
      <Pane
        display="flex"
        flex={1}
        alignItems="center"
        flexDirection="column"
        borderRight="muted"
      >
        <Pane
          display="flex"
          flexDirection="column"
          flex={1}
          marginLeft={5}
          alignItems="center"
          justifyContent="center"
        >
          <Heading
            size={400}
            marginBottom={minorScale(2)}
            alignItems="right"
            color={defaultTheme.palette.neutral.dark}
          >
            {_("monthly_cashflow")}
          </Heading>
          {NumberHelper.isInteger(cashflow)
            ? (
              <Pane
                display="flex"
                flexDirection="row"
                justifyContent="center"
                alignItems="center"
              >
                <AmountLabel
                  type={AmountLabelType.BIG}
                  value={cashflow}
                  addSign
                />
                {!!cashflowVariation && (
                  <Pane marginX={majorScale(1)}>
                    <Icon
                      icon={cashflowVariation > 0 ? ArrowUpIcon : ArrowDownIcon}
                      size={10}
                      color={cashflowVariation > 0
                        ? defaultTheme.palette.green.dark
                        : defaultTheme.palette.red.dark}
                      marginX={2}
                    />
                    <Text
                      size={300}
                      color={cashflowVariation > 0
                        ? defaultTheme.palette.green.dark
                        : defaultTheme.palette.red.dark}
                    >
                      {NumberHelper.formatToPercentage(cashflowVariation)}
                    </Text>
                  </Pane>
                )}
              </Pane>
            )
            : (
              <EmptyItem />
            )}
        </Pane>
      </Pane>

      {/* Loyer à venir */}
      <Pane display="flex" flex={1} alignItems="center" flexDirection="column">
        <Pane
          display="flex"
          flexDirection="column"
          flex={1}
          marginLeft={5}
          alignItems="center"
          justifyContent="center"
        >
          <Heading
            size={400}
            marginBottom={minorScale(2)}
            alignItems="right"
            color={defaultTheme.palette.neutral.dark}
          >
            {_("late_rent")}
          </Heading>
          {receivedRentNb && totalRentNb
            ? (
              <Pane
                display="flex"
                flexDirection="row"
                justifyContent="center"
                alignItems="center"
              >
                <Heading size={600}>
                  {receivedRentNb}/{totalRentNb}
                </Heading>
                {!!receivedRentPercentage && (
                  <Pane marginX={majorScale(1)}>
                    <Text size={300} color={defaultTheme.palette.green.dark}>
                      {NumberHelper.formatToPercentage(receivedRentPercentage)}
                    </Text>
                  </Pane>
                )}
              </Pane>
            )
            : (
              <EmptyItem />
            )}
        </Pane>
      </Pane>
    </Card>
  );
};
