import {
  BankAccountIcon,
  Button,
  Card,
  DoubleChevronDownIcon,
  Heading,
  Icon,
  Pane,
  TextInputField,
  TimelineEventsIcon,
} from "evergreen-ui";
import { FormikProps } from "formik";
import { translate } from "piteo-kit";
import React from "react";
import { FieldHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { LeaseFurnishedInput } from "../../types";
import { LeaseFurnishedValidator } from "../../validators";
import { Row, Section } from "../common";
import { NumberIcon } from "../common/number-icon";

const _ = translate();

export enum FieldNames {
  TYPE = "type",
  LENDER = "lenderId",
  EFFECT_DATE = "effectDate",
  SIGNATURE_DATE = "signatureDate",
  RENT_AMOUNT = "rentAmount",
  RENT_CHARGES_AMOUNT = "rentChargesAmount",
  DEPOSIT_AMOUNT = "depositAmount",
  FILE = "file",
}

const FIELD_MAX_WIDTH = 200;

export type LeaseExistingFormProps<T> = {
  /** Form */
  form?: FormikProps<T>;
  /** Validation schema */
  validationSchema?: typeof LeaseFurnishedValidator;
};

export const LeaseExistingForm: React.FunctionComponent<
  LeaseExistingFormProps<LeaseFurnishedInput>
> = ({
  form,
  validationSchema,
}) => {
  const theme = useAppTheme();
  return (
    <Pane is="form" onSubmit={form?.handleSubmit}>
      <Card
        background="white"
        padding={theme.margin.large}
        elevation={2}
      >
        <Pane
          display="flex"
          alignItems="center"
          borderBottom="muted"
          paddingBottom={theme.margin.medium}
        >
          <NumberIcon>2</NumberIcon>
          <Heading marginLeft={theme.margin.medium}>
            {_("contract_conditions_hint")}
          </Heading>
        </Pane>

        {/* Dates */}
        <Section
          title={_("dates")}
          icon={TimelineEventsIcon}
          marginTop={theme.margin.large}
        >
          <Row>
            {/* Date d'effet */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.EFFECT_DATE,
              )}
              label={_("effect_date")}
              placeholder={_("effect_date_placeholder")}
              hint={_("effect_date_hint")}
              type="date"
              maxWidth={FIELD_MAX_WIDTH}
            />
            {/* Date de signature */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.SIGNATURE_DATE,
              )}
              label={_("signature_date")}
              placeholder={_("signature_date_placeholder")}
              type="date"
              maxWidth={FIELD_MAX_WIDTH}
            />
          </Row>
        </Section>

        {/* Données financières */}
        <Section
          title={_("financial_data")}
          icon={BankAccountIcon}
        >
          <Row>
            {/* Montant du loyer */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.RENT_AMOUNT,
              )}
              label={_("rent_amount")}
              placeholder={_("rent_amount_placeholder")}
              hint={_("rent_amount_hint")}
              maxWidth={FIELD_MAX_WIDTH}
              type="number"
            />
            {/* Montant des charges */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.RENT_CHARGES_AMOUNT,
              )}
              label={_("rent_charges_amount")}
              placeholder={_("rent_charges_amount_placeholder")}
              maxWidth={FIELD_MAX_WIDTH}
              type="number"
            />
          </Row>
          <Row>
            {/* Montant dépôt de garantie */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.DEPOSIT_AMOUNT,
              )}
              label={_("deposit_amount")}
              placeholder={_("deposit_amount_placeholder")}
              maxWidth={FIELD_MAX_WIDTH}
              type="number"
            />
            <Pane maxWidth={FIELD_MAX_WIDTH} />
          </Row>
        </Section>
      </Card>

      <Pane
        display="flex"
        justifyContent="center"
        marginTop={theme.margin.large}
      >
        <Icon icon={DoubleChevronDownIcon} color={theme.palette.blue.base} />
      </Pane>

      <Pane
        display="flex"
        justifyContent="center"
        marginTop={theme.margin.large}
      >
        <Button
          isLoading={form?.isSubmitting}
          appearance="primary"
          type="submit"
          marginTop={theme.margin.medium}
          disabled={!form.dirty || !form?.isValid}
        >
          {_("continue")}
        </Button>
      </Pane>
    </Pane>
  );
};
