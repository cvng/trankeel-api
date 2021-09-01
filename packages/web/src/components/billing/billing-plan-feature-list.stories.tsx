import React from "react";
import { translate } from "piteo-kit";
import { BillingPlanFeatureList } from "./billing-plan-feature-list";

const _ = translate();

export default {
  title: "Billing/BillingPlanFeatureList",
  component: BillingPlanFeatureList,
};

export const standard = () => (
  <BillingPlanFeatureList
    featureList={[
      { title: _("benefit_1"), available: true },
      { title: _("benefit_2"), available: true },
      { title: _("benefit_3"), available: true },
      { title: _("benefit_4"), available: true },
      { title: _("benefit_5"), available: false },
      { title: _("benefit_6"), available: false },
    ]}
  />
);
