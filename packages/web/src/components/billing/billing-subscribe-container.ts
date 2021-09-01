import { useMutation, useQuery } from "@apollo/client";
import { CardElement, useElements, useStripe } from "@stripe/react-stripe-js";
import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { FunctionComponent, useState } from "react";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import {
  AccountActivatePlanMutation,
  AccountActivatePlanMutationOptions,
  AccountUpdatePaymentMethodMutation,
  PricingPlansQuery,
  UserQuery,
} from "../../helpers";
import { AccountActivatePlanInput } from "../../types";
import { code } from "../../utils";
import { SubscriptionValidator } from "../../validators";
import { BillingSubscribeProps } from "../billing/billing-subscribe";
import { BillingSubscribePageProps } from "./billing-subscribe-page";
import { translate } from "piteo-kit";

const _ = translate();

export const withContainer = (
  WrappedComponent,
): FunctionComponent<BillingSubscribeProps> =>
  () => {
    const history = useHistory();
    const stripe = useStripe();
    const elements = useElements();
    const [isProcessingPayment, setProcessingPayment] = useState(false);

    // Get the user data
    const { data: { user } = { user: null }, loading: isLoadingUser } =
      useQuery(
        UserQuery,
      );

    // Get the available plans
    const { data: { plans } = { plans: [] }, loading: isLoadingPlans } =
      useQuery(
        PricingPlansQuery,
      );
    // Use the first plan as there is only one plan available for the moment
    const plan = plans[0];
    const planCode = plan?.code;

    const initialValues = { planCode: planCode, name: "", id: "" };

    // Prepare the mutation requests that will be called once the card transaction is accepted
    const [accountUpdatePaymentMethod] = useMutation(
      AccountUpdatePaymentMethodMutation,
    );
    const [accountActivatePlan] = useMutation(
      AccountActivatePlanMutation,
      AccountActivatePlanMutationOptions(),
    );

    const handleSubmit = async (values: AccountActivatePlanInput) => {
      const toast = { id: "create-payment-method-action" };

      // Tokenize the credit card
      const { error, paymentMethod } = await stripe.createPaymentMethod({
        type: "card",
        card: elements.getElement(CardElement),
        billing_details: { email: user.email, name: values.name },
      });

      // Handle submission error.
      if (error || !paymentMethod) {
        toaster.danger(_(code(error)), toast);
        return;
      }

      // Handle next steps of the process.
      try {
        setProcessingPayment(true);
        // Update the user in the backend
        const accountId = user.account.id;
        const paymentVariables = {
          id: accountId,
          paymentMethodId: paymentMethod.id,
        };
        await accountUpdatePaymentMethod({
          variables: { input: paymentVariables },
        });
        toaster.success(_("update_payment_method_success"), toast);

        // Set the paid plan in the account
        const planVariables = { id: accountId, planCode: planCode };
        await accountActivatePlan({ variables: planVariables });
        toaster.success(_("activate_plan_success"), toast);

        // Once the payment is successfull allow the user to navigate
        history.push(Routes.DEFAULT);
      } catch (error) {
        toaster.danger(_(code(error), { planCode }));
      } finally {
        setProcessingPayment(false);
      }
    };

    const form = useFormik({
      initialValues,
      onSubmit: handleSubmit,
    });

    const componentProps: BillingSubscribePageProps = {
      plan: plan,
      form,
      loading: isLoadingUser || isLoadingPlans,
      processingPayment: isProcessingPayment,
      validationSchema: SubscriptionValidator,
    };

    return WrappedComponent(componentProps);
  };
