import {
  Alert,
  ArrowRightIcon,
  Button,
  defaultTheme,
  FormField,
  Icon,
  majorScale,
  minorScale,
  Pane,
  TextInputField,
} from "evergreen-ui";
import { FormikProps } from "formik";
import { translate } from "piteo-kit";
import React from "react";
import { TourType } from "../../constants/tour-constants";
import { FieldHelper } from "../../helpers";
import { TransactionInput } from "../../types";
import { TransactionRentValidator } from "../../validators";
import { Row, Section } from "../common";

const _ = translate("Transaction");

export enum FieldNames {
  TENANT_ID = "tenantId",
  AMOUNT = "amount.amount",
  CHARGES_AMOUNT = "chargesAmount.amount",
  DATE = "date",
  PERIOD_START = "periodStart",
  PERIOD_END = "periodEnd",
}

export type RentReceiptPartial = {
  amount: number;
  chargesAmount: number;
  rentFullAmount: number;
  days: number;
};

export type RentReceiptFormProps = {
  /** Form */
  form?: FormikProps<TransactionInput>;
  /** Validation schema */
  validationSchema?: typeof TransactionRentValidator;
  /** True if the lender identity address is not set */
  isMissingAddress?: boolean;
  /** True when the form need to be disabled */
  disabled?: boolean;
  /** True when the rent is partial */
  isPartialRent?: boolean;
  /** Fill when the rent is partial */
  partialRentData?: RentReceiptPartial;
  /** Called on update address button click */
  onUpdateAddressButtonClick?: () => void;
  /** Called on select use prorata rent */
  useProrataRentButtonClick?: () => void;
};

export const RentReceiptForm: React.FunctionComponent<
  RentReceiptFormProps
> = ({
  form,
  validationSchema,
  isMissingAddress,
  disabled,
  isPartialRent,
  partialRentData = null,
  onUpdateAddressButtonClick,
  useProrataRentButtonClick,
}) => {
  return (
    <Pane
      display="flex"
      flex={1}
      flexDirection="column"
      padding={majorScale(2)}
      is="form"
      onSubmit={form?.handleSubmit}
    >
      {/* Période de location */}
      <FormField label={_("rent_period")}>
        <Section>
          <Pane
            display="flex"
            flex={1}
            alignItems="space-between"
            justifyContent="center"
            className={TourType.TRANSACTION_RENT_ADD_SEL_RENT_LOCATION}
          >
            {/* Date de début */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.PERIOD_START,
              )}
              type="date"
              label={_("from")}
              disabled={disabled}
            />
            <Pane display="flex" flexDirection="column" justifyContent="center">
              <Icon
                icon={ArrowRightIcon}
                color={defaultTheme.palette.blue.base}
                marginX={minorScale(2)}
              />
            </Pane>
            {/* Date de fin */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.PERIOD_END,
              )}
              type="date"
              label={_("to")}
              disabled={disabled}
            />
          </Pane>

          <Pane
            display="flex"
            flexDirection="column"
            justifyContent="flex-start"
            alignItems="flex-end"
            marginLeft={majorScale(1)}
            marginBottom={majorScale(1)}
          >
            {isPartialRent && (
              <Alert
                appearance="card"
                intent="none"
                title={_("prorata_rent_message", partialRentData)}
                marginBottom={32}
              >
                <Button
                  appearance="minimal"
                  type="button"
                  onClick={useProrataRentButtonClick}
                  disabled={disabled}
                >
                  {_("use_prorata_rate")}
                </Button>
              </Alert>
            )}
          </Pane>
        </Section>
      </FormField>

      <Row className={TourType.TRANSACTION_RENT_ADD_SEL_RENT_AMOUNT}>
        {/* Montant du loyer */}
        <TextInputField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.AMOUNT,
          )}
          type="number"
          min={0}
          label={_("rent_amount")}
          hint={_("rent_hc")}
          placeholder={_("rent_amount_placeholder")}
          step={0.01}
          disabled={disabled}
        />

        {/* Montant des charges */}
        <TextInputField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.CHARGES_AMOUNT,
          )}
          type="number"
          min={0}
          label={_("charges_amount")}
          hint={_("charges_received")}
          step={0.01}
          placeholder={_("charges_placeholder")}
          disabled={disabled}
        />
      </Row>

      {/* Date */}
      <TextInputField
        className={TourType.TRANSACTION_RENT_ADD_RECEIVED_RENT_DATE}
        {...FieldHelper.getFieldProps(form, validationSchema, FieldNames.DATE)}
        type="date"
        label={_("rent_receive_date")}
        disabled={disabled}
      />

      {/* TODO: Implémenter l'ajout de ce booléen */}
      {/* <Checkbox label="Envoyer automatiquement la quittance lors de la génération" /> */}

      {isMissingAddress && (
        <Alert
          marginY={majorScale(2)}
          intent="warning"
          title={_("update_address_message")}
          width={350}
        >
          <Button
            appearance="minimal"
            type="button"
            onClick={onUpdateAddressButtonClick}
            disabled={disabled}
          >
            {_("update_address")}
          </Button>
        </Alert>
      )}
    </Pane>
  );
};
