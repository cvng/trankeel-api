import { CardElement } from "@stripe/react-stripe-js";
import {
  Button,
  CreditCardIcon,
  FormField,
  majorScale,
  Pane,
  TextInputField,
} from "evergreen-ui";
import { FormikProps } from "formik";
import * as React from "react";
import { FieldHelper } from "../../helpers";
import { AccountActivatePlanInput } from "../../types";
import { SubscriptionValidator } from "../../validators";
import { translate } from "piteo-kit";

const _ = translate();

export enum FieldNames {
  NAME = "name",
}

const CARD_ELEMENT_OPTIONS = {
  style: {
    base: {
      color: "#010101",
      fontFamily: '"SF UI Text", Helvetica, sans-serif',
      fontSmoothing: "antialiased",
      fontSize: "13px",
      "::placeholder": {
        color: "#aab7c4",
      },
    },
    invalid: {
      color: "#fa755a",
      iconColor: "#fa755a",
    },
  },
  hidePostalCode: true,
};

interface BillingFormProps {
  /** True while loading */
  loading: boolean;
  /** Form */
  form?: FormikProps<AccountActivatePlanInput>;
  /** Validation schema */
  validationSchema?: typeof SubscriptionValidator;
}

export const BillingForm: React.FunctionComponent<BillingFormProps> = ({
  loading,
  form,
  validationSchema,
}) => (
  <Pane
    is="form"
    onSubmit={form?.handleSubmit}
    display="flex"
    flexDirection="column"
    justifyContent="center"
    padding={majorScale(2)}
    minWidth={300}
  >
    {/* Nom du porteur de la carte */}
    <TextInputField
      {...FieldHelper.getFieldProps(form, validationSchema, FieldNames.NAME)}
      label={_("card_holder_name")}
      placeholder={_("card_holder_name_placeholder")}
      disabled={loading}
    />

    <FormField label="Données de la carte">
      <Pane
        border="default"
        borderRadius={4}
        background={loading ? "#F5F6F7" : "white"}
        borderColor="#DCE0E3"
        height={majorScale(4)}
        paddingTop={8}
        paddingX={10}
        marginBottom={majorScale(3)}
      >
        <CardElement
          options={{ ...CARD_ELEMENT_OPTIONS, ...{ disabled: loading } }}
        />
      </Pane>
    </FormField>

    <Button
      iconBefore={CreditCardIcon}
      appearance="primary"
      isLoading={loading}
      disabled={!form?.dirty || !form?.isValid}
    >
      {_("activate_my_subscription")}
    </Button>
  </Pane>
);
