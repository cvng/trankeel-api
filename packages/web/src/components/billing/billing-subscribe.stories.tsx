import { Elements } from "@stripe/react-stripe-js";
import { loadStripe } from "@stripe/stripe-js";
import React from "react";
import { PlanCode } from "../../types";
import { env } from "../../utils";
import { BillingSubscribe } from "./billing-subscribe";
import { translate } from "piteo-kit";

const _ = translate();

const stripePromise = loadStripe(env("REACT_APP_STRIPE_PUBLISHABLE_KEY"));

export default {
  title: "Billing/BillingSubscribe",
  component: BillingSubscribe,
};

const PLAN = {
  id: null,
  title: "Plan Standard",
  subtitle: "Parfait pour débuter",
  code: PlanCode.Solo,
  features: [
    { id: null, title: _("benefit_1"), available: true },
    { id: null, title: _("benefit_2"), available: true },
    { id: null, title: _("benefit_3"), available: true },
    { id: null, title: _("benefit_4"), available: true },
  ],
  price: 990,
  available: true,
};

const StripeElementsWrapper = (props) => {
  return <Elements stripe={stripePromise}>{props.children}</Elements>;
};

export const loading = () => {
  return (
    <StripeElementsWrapper>
      <BillingSubscribe plan={PLAN} loading />
    </StripeElementsWrapper>
  );
};

export const standard = () => {
  return (
    <StripeElementsWrapper>
      <BillingSubscribe plan={PLAN} />
    </StripeElementsWrapper>
  );
};

export const processingPayment = () => {
  return (
    <StripeElementsWrapper>
      <BillingSubscribe plan={PLAN} processingPayment />
    </StripeElementsWrapper>
  );
};
