import React from "react";
import { BillingPlan } from "./billing-plan";
import { translate } from "piteo-kit";
import { Plan, PlanCode } from "../../types";

const _ = translate();

export default {
  title: "Billing/BillingPlan",
  component: BillingPlan,
};

const plan: Plan = {
  id: null,
  title: "Plan standard",
  subtitle: "Parfait pour débuter",
  price: 990,
  code: PlanCode.Solo,
  features: [
    { title: _("benefit_1"), available: true },
    { title: _("benefit_2"), available: true },
    { title: _("benefit_3"), available: true },
    { title: _("benefit_4"), available: true },
    { title: _("benefit_5"), available: false },
    { title: _("benefit_6"), available: false },
  ],
};

export const standard = () => <BillingPlan plan={plan} available selectable />;

export const premium = () => <BillingPlan plan={plan} selectable />;

export const noSelectable = () => (
  <BillingPlan plan={plan} selectable={false} />
);

export const loading = () => (
  <BillingPlan plan={plan} selectable={false} loading />
);
