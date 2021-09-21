import { Heading, Pane } from "evergreen-ui";
import { rentStatusMapColor, translate } from "piteo-kit";
import * as React from "react";
import { NumberHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { RentStatus } from "../../types";

const _ = translate("");

export type RentCollectedBarProps = {
  amountPending: number; // Loyers en attente (somme des loyers qui n'ont pas encore été encaissé pour la période en cours)
  amountPartial: number; // Loyers partiellement payé (somme des montant partiels reçus pour tous les locataires)
  amountReceived: number; // Loyers perçus (somme des loyers perçus pour la période en cours)
  ratioReceived: number;
  ratioPending: number;
  ratioPartial: number;
};

export const RentCollectedBar: React.FunctionComponent<RentCollectedBarProps> =
  ({
    amountPending = 0,
    amountPartial = 0,
    amountReceived = 0,
    ratioReceived = 0,
    ratioPending = 0,
    ratioPartial = 0,
  }) => {
    const theme = useAppTheme();

    // Colors
    const collectedColor = rentStatusMapColor(RentStatus.Settled);
    const partialColor = rentStatusMapColor(RentStatus.Partial);
    const pendingColor = rentStatusMapColor(RentStatus.Pending);

    const items: {
      title: string;
      amount: number;
      percentage: number;
      color: string;
    }[] = [
      {
        title: _("rent_received"),
        amount: amountReceived,
        percentage: Math.round(ratioReceived),
        color: collectedColor,
      },
      {
        title: _("rent_partial"),
        amount: amountPartial,
        percentage: Math.round(ratioPartial),
        color: partialColor,
      },
      {
        title: _("rent_pending"),
        amount: amountPending,
        percentage: Math.round(ratioPending),
        color: pendingColor,
      },
    ];
    return (
      <Pane flex={1}>
        <Heading>{_("rent_collection")}</Heading>
        {/* Bar */}
        <Pane
          display="flex"
          background={theme.palette.blue.lightest}
          height={22}
          marginTop={theme.margin.medium}
        >
          {items
            .map((item, index) => {
              return <Pane
                key={index}
                background={item.color}
                width={`${item.percentage}%`}
                paddingX={item.percentage > 0 ? theme.margin.medium : 0}
                display="flex"
                justifyContent="flex-end"
                alignItems="center"
              />;
            })}
        </Pane>
        {/* Legend */}
        <Pane
          display="flex"
          justifyContent="space-between"
          flexDirection="column"
        >
          {items
            .map((item, index) => {
              return <Pane
                key={index}
                display="flex"
                marginTop={theme.margin.medium}
              >
                <Pane display="flex" justifyContent="space-between" flex={1}>
                  <Pane display="flex" alignItems="center">
                    <Pane
                      background={item.color}
                      height={10}
                      width={10}
                      borderRadius={5}
                      marginRight={theme.margin.medium}
                    />
                    <Heading size={300}>
                      {item.title}
                    </Heading>
                  </Pane>
                  <Heading size={300}>
                    {NumberHelper.formatToString(item.amount, false)}
                  </Heading>
                </Pane>
              </Pane>;
            })}
        </Pane>
      </Pane>
    );
  };
