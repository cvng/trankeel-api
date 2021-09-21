// @ts-nocheck
import {
  Card,
  Heading,
  Link,
  majorScale,
  Pane,
  Paragraph,
  Text,
} from "evergreen-ui";
import * as React from "react";
import { NumberHelper } from "../../helpers";
import { Plan } from "../../types";
import { AmountLabel, AmountLabelType } from "../common/amount-label";
import { BillingPlanFeatureList } from "./billing-plan-feature-list";
import { translate } from "piteo-kit";

const _ = translate();

export type BillingCheckoutSummaryProps = {
  plan?: Plan;
};

export const BillingCheckoutSummary: React.FunctionComponent<
  BillingCheckoutSummaryProps
> = ({
  plan,
}) => (
  <Pane
    elevation={1}
    flex={1}
    padding={majorScale(2)}
    maxWidth={300}
    display="flex"
    flexDirection="column"
  >
    <Card
      background="tint2"
      minHeight={40}
      display="flex"
      alignItems="center"
      justifyContent="center"
    >
      <Heading size={400}>{_("checkout_summary")}</Heading>
    </Card>

    <Pane
      display="flex"
      flexDirection="row"
      justifyContent="space-between"
      marginTop={majorScale(3)}
    >
      <Heading size={400}>{plan.title}</Heading>
    </Pane>
    <BillingPlanFeatureList featureList={plan.features} />

    <Pane
      display="flex"
      flexDirection="row"
      justifyContent="space-between"
      marginTop={majorScale(3)}
    >
      <Text>{_("billing")}</Text>
      <Heading size={400}>{_("billing_mode_monthly")}</Heading>
    </Pane>
    <Pane
      display="flex"
      flexDirection="column"
      borderTop={"1px solid #495b70"}
      paddingTop={10}
    >
      <Pane display="flex" flexDirection="row" justifyContent="space-between">
        <Text>Prix :</Text>
        <AmountLabel
          type={AmountLabelType.MEDIUM}
          value={NumberHelper.calculateVat(plan.price).amount}
        />
      </Pane>
      <Pane
        display="flex"
        flexDirection="row"
        justifyContent="space-between"
        marginTop={majorScale(1)}
      >
        <Text>TVA :</Text>
        <AmountLabel
          type={AmountLabelType.MEDIUM}
          value={NumberHelper.calculateVat(plan.price).vatAmount}
        />
      </Pane>
      <Pane
        display="flex"
        flexDirection="row"
        justifyContent="space-between"
        borderTop={"1px solid #495b70"}
        marginTop={majorScale(2)}
      >
        <Heading size={500}>Total :</Heading>
        <AmountLabel type={AmountLabelType.MEDIUM} value={plan.price} />
      </Pane>
    </Pane>

    <Pane marginTop={majorScale(2)}>
      <Paragraph size={300}>
        En validant cette commande, vous acceptez nos{" "}
        <Link size={300}>conditions générales de vente.</Link>
      </Paragraph>
    </Pane>
  </Pane>
);
