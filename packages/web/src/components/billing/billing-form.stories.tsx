import React from "react";
import { BillingForm } from "./billing-form";
import { Pane } from "evergreen-ui";
import { Elements } from "@stripe/react-stripe-js";
import { loadStripe } from "@stripe/stripe-js";

const stripePromise = loadStripe(process.env.NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY);

export default {
  title: "Billing/BillingForm",
  component: BillingForm,
};

const StripeElementsWrapper = (props) => {
  return <Elements stripe={stripePromise}>{props.children}</Elements>;
};

export const standard = () => (
  <Pane maxWidth={300}>
    <StripeElementsWrapper>
      <BillingForm loading={false} />
    </StripeElementsWrapper>
  </Pane>
);

export const loading = () => (
  <Pane maxWidth={300}>
    <StripeElementsWrapper>
      <BillingForm loading />
    </StripeElementsWrapper>
  </Pane>
);
