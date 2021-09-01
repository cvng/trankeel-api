import React from "react";
import { BillingForm } from "./billing-form";
import { Pane } from "evergreen-ui";
import { Elements } from "@stripe/react-stripe-js";
import { loadStripe } from "@stripe/stripe-js";
import { env } from "../../utils";

const stripePromise = loadStripe(env("REACT_APP_STRIPE_PUBLISHABLE_KEY"));

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
