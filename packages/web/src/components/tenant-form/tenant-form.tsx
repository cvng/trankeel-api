import { Pane, TextInputField } from "evergreen-ui";
import { FormikProps } from "formik";
import React from "react";
import { FieldHelper } from "../../helpers";
import { TenantInput } from "../../types";
import { TenantValidator } from "../../validators";
import { CardItem } from "../cards/card-item";
import { FormFooter, Row, Section } from "../common";
import { SegmentedControlField, TextareaField } from "../fields";
import { translate } from "piteo-kit";

const _ = translate();

const FIELD_MAX_WIDTH = 300;

export enum FieldNames {
  FIRST_NAME = "firstName",
  LAST_NAME = "lastName",
  EMAIL = "email",
  PHONE_NUMBER = "phoneNumber",
  NOTE = "note",
  APL = "apl",
  VISALE = "visaleId",
  BIRTHPLACE = "birthplace",
  BIRTHDATE = "birthdate",
}

export type TenantFormProps = {
  /** Form */
  form: FormikProps<TenantInput>;
  /** Validation schema */
  validationSchema: typeof TenantValidator;
  /** True if has footer */
  hasFooter?: boolean;
};

export const TenantForm: React.FunctionComponent<TenantFormProps> = ({
  form,
  validationSchema,
  hasFooter = true,
}) => (
  <Pane is="form" onSubmit={form?.handleSubmit}>
    <CardItem
      title={`${form?.values?.firstName || ""} ${form?.values?.lastName || ""}`}
      elevation={null}
      style={{}}
      border={"none"}
    />

    <Section title={_("tenant_details")} borderBottom={hasFooter}>
      <Row>
        {/* First name */}
        <TextInputField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.FIRST_NAME,
          )}
          label={_("first_name")}
          placeholder={_("first_name_placeholder")}
        />

        {/* Last name */}
        <TextInputField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.LAST_NAME,
          )}
          label={_("last_name")}
          placeholder={_("last_name_placeholder")}
        />
      </Row>

      <Row>
        {/* Email */}
        <TextInputField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.EMAIL,
          )}
          type="email"
          label={_("email")}
          placeholder={_("email_placeholder")}
        />

        {/* Phone number */}
        <TextInputField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.PHONE_NUMBER,
          )}
          label={_("phone_number")}
          placeholder={_("phone_number_placeholder")}
        />
      </Row>

      <Row>
        {/* Lieu de naissance */}
        <TextInputField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.BIRTHPLACE,
          )}
          type="email"
          label={_("place_of_birth")}
          placeholder={_("place_of_birth_placeholder")}
        />

        {/* Date de naissance */}
        <TextInputField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.BIRTHDATE,
          )}
          type="date"
          label={_("date_of_birth")}
          placeholder={_("date_of_birth_placeholder")}
        />
      </Row>

      <Section title={"Garanties"}>
        {/* APL */}
        <SegmentedControlField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            { "name": FieldNames.APL, "type": "control" },
          )}
          label={_("apl")}
          hint={_("apl_hint")}
          options={[
            { label: _("yes"), value: true },
            { label: _("no"), value: false },
          ]}
          width={FIELD_MAX_WIDTH}
        />

        {/* Visale */}
        <TextInputField
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.VISALE,
          )}
          label={_("visale")}
          placeholder={_("visale_placeholder")}
          hint={_("visale_hint")}
          maxWidth={FIELD_MAX_WIDTH}
        />
      </Section>

      {/* Note */}
      <TextareaField
        {...FieldHelper.getFieldProps(form, validationSchema, FieldNames.NOTE)}
        label={_("observations")}
        placeholder={_("observations_placeholder")}
      />
    </Section>

    {hasFooter && <FormFooter form={form} />}
  </Pane>
);
