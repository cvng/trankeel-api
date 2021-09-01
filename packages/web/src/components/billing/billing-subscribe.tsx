import { Link, majorScale, Pane, Paragraph } from "evergreen-ui";
import { FormikProps } from "formik";
import * as React from "react";
import { translate } from "piteo-kit";
import { AccountActivatePlanInput, Plan } from "../../types";
import { SubscriptionValidator } from "../../validators";
import { BillingForm } from "./billing-form";
import { BillingPlan } from "./billing-plan";

const _ = translate();

export type BillingSubscribeProps = {
  plan?: Plan;
  loading?: boolean;
  processingPayment?: boolean;
  form?: FormikProps<AccountActivatePlanInput>;
  validationSchema?: typeof SubscriptionValidator;
};

export const BillingSubscribe: React.FunctionComponent<BillingSubscribeProps> =
  ({
    plan,
    loading,
    processingPayment,
    form,
    validationSchema,
  }) => {
    return (
      <Pane
        display="flex"
        flexDirection="column"
        alignItems="center"
        justifyContent="center"
        minHeight={600}
      >
        <Pane display="flex" marginTop={majorScale(3)}>
          <BillingForm
            form={form}
            validationSchema={validationSchema}
            loading={loading || processingPayment}
          />

          <BillingPlan
            plan={plan}
            available
            selectable={false}
            loading={loading && !processingPayment}
          />
        </Pane>

        <Paragraph size={300} marginTop={majorScale(2)}>
          {_("cgu_acceptation_part_one")}
          <Link size={300} href={_("cgu_acceptation_url")} target="blank">
            {_("cgu_acceptation_part_two")}
          </Link>
        </Paragraph>
      </Pane>
    );
  };
