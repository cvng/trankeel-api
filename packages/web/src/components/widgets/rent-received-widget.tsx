import { Pane, PaneProps, Text } from "evergreen-ui";
import moment from "moment";
import { translate } from "piteo-kit";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { AmountLabel, AmountLabelType } from "../common/amount-label";
import { PeriodSelector } from "../common/period-selector";

const _ = translate();

const MONTH_NAME_DATE_FORMAT = "MMMM";

export type RentReceivedWidgetProps = {
  loading?: boolean;
  receivedAmount?: number;
  startedDate?: Date;
  endedDate?: Date;
} & PaneProps;

export const RentReceivedWidget: React.FunctionComponent<
  RentReceivedWidgetProps
> = ({
  loading,
  receivedAmount = 0,
  startedDate,
  endedDate,
  ...props
}) => {
  return (
    <Pane>
      {loading
        ? (
          <Pane
            display="flex"
            flexDirection="row"
            alignItems="center"
            justifyContent="space-between"
          >
            <Pane width={150}>
              <Skeleton count={2} />
            </Pane>
            <Pane width={150}>
              <Skeleton count={1} height={30} />
            </Pane>
            <Pane width={100}>
              <Skeleton count={1} height={30} />
            </Pane>
          </Pane>
        )
        : (
          <Pane
            display="flex"
            flexDirection="row"
            alignItems="center"
            justifyContent="space-between"
          >
            <Pane>
              {!loading && (
                <PeriodSelector
                  startedDate={moment(startedDate).toDate()}
                  endedDate={moment(endedDate).toDate()}
                />
              )}
            </Pane>
            <Pane display="flex" flexDirection="column" alignItems="flex-end">
              <AmountLabel
                type={AmountLabelType.BIG}
                value={receivedAmount || 0}
              />
              <Text>
                {_("rent_month", {
                  monthName: moment(startedDate).format(MONTH_NAME_DATE_FORMAT),
                })}
              </Text>
            </Pane>
          </Pane>
        )}
    </Pane>
  );
};
