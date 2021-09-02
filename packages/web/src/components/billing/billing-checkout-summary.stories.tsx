import React from "react";
import { translate } from "piteo-kit";
import { BillingCheckoutSummary } from "./billing-checkout-summary";
import { Plan, PlanCode } from "../../types";

const _ = translate();

export default {
  title: "Billing/BillingCheckoutSummary",
  component: BillingCheckoutSummary,
};

const plan: Plan = {
  id: null,
  title: "Plan SOLO",
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

export const standard = () => <BillingCheckoutSummary plan={plan} />;
