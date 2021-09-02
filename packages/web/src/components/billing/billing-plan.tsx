import {
  Button,
  Card,
  Heading,
  LockIcon,
  majorScale,
  minorScale,
  Pane,
  PaneProps,
  Text,
} from "evergreen-ui";
import * as React from "react";
import Skeleton from "react-loading-skeleton";
import { NumberHelper } from "../../helpers";
import { translate } from "piteo-kit";
import { Plan } from "../../types";
import { BillingPlanFeatureList } from "./billing-plan-feature-list";

const _ = translate();

export type BillingPlanProps = {
  plan?: Plan;
  available?: boolean;
  selectable?: boolean;
  loading?: boolean;
} & PaneProps;

export const BillingPlan: React.FunctionComponent<BillingPlanProps> = ({
  plan,
  available,
  selectable = true,
  loading,
  ...props
}) => (
  <Card
    display="flex"
    flexDirection="column"
    background="white"
    maxWidth={300}
    elevation={2}
    padding={majorScale(3)}
    {...props}
  >
    {loading
      ? (
        <Pane minWidth={200}>
          <Skeleton />
          <Skeleton height={12} />
          <Pane marginTop={minorScale(2)}>
            <Skeleton height={20} />
          </Pane>
          <Pane marginTop={minorScale(2)}>
            <Skeleton height={20} count={4} />
          </Pane>
        </Pane>
      )
      : (
        <Pane>
          <Heading>{plan?.title}</Heading>
          <Text>{plan?.subtitle}</Text>

          <Pane
            display="flex"
            flexDirection="column"
            justifyContent="space-around"
            marginY={majorScale(2)}
          >
            <Text size={500}>
              <strong>
                {_("price_by_month", {
                  price: NumberHelper.formatToString(plan?.price, false),
                })}
              </strong>
            </Text>
          </Pane>

          <BillingPlanFeatureList featureList={plan?.features} />

          {selectable && (
            <Pane display="flex" flex={1} justifyContent="center">
              <Button
                iconBefore={available === true ? null : LockIcon}
                disabled={!available}
                appearance="primary"
                marginY={majorScale(2)}
              >
                {available ? _("select_plan") : _("select_plan_unavailable")}
              </Button>
            </Pane>
          )}
        </Pane>
      )}
  </Card>
);
