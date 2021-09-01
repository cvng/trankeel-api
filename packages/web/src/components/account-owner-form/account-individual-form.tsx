import {
  Button,
  EndorsedIcon,
  EyeOffIcon,
  EyeOnIcon,
  Pane,
  Text,
  TextInputField,
} from "evergreen-ui";
import { FormikProps } from "formik";
import React from "react";
import { FieldHelper } from "../../helpers";
import { translate } from "piteo-kit";
import { RegisterValidationSchema } from "../../validators";
import { Row, Section } from "../common";
import { FORM_INPUT_WIDTH } from "../common/form-item";

const _ = translate();

export enum FieldNames {
  FIRST_NAME = "firstName",
  LAST_NAME = "lastName",
  EMAIL = "email",
  PASSWORD = "password",
  STREET = "address.line1",
  ADDITIONAL_STREET = "address.line2",
  POSTAL_CODE = "address.postalCode",
  CITY = "address.city",
}

export type AccountIndividualFormProps = {
  /** Form */
  form?: FormikProps<{
    firstName: string;
    lastName: string;
    email: string;
    password: string;
  }>;
  /** Validation schema */
  validationSchema?: typeof RegisterValidationSchema;
  /** True if has footer */
  hasFooter?: boolean;
  /** True if we are in the editable mode */
  isEditableModeEnabled?: boolean;
  /** True if the password value is display */
  showPassword?: boolean;
  /** True if the password value is display */
  changePasswordVisibility?: () => void;
};

export const AccountIndividualForm: React.FunctionComponent<
  AccountIndividualFormProps
> = ({
  form,
  validationSchema,
  hasFooter = true,
  isEditableModeEnabled = false,
  showPassword = false,
  changePasswordVisibility,
}) => {
  return (
    <Pane is="form" onSubmit={form?.handleSubmit} width={FORM_INPUT_WIDTH}>
      {/* Informations personnelles */}
      <Section title={_("personal_details")}>
        <Row>
          {/* Prénom */}
          <TextInputField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.FIRST_NAME,
            )}
            label={_("firstname")}
            placeholder={_("firstname_placeholder")}
          />

          {/* Nom */}
          <TextInputField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.LAST_NAME,
            )}
            label={_("lastname")}
            placeholder={_("lastname_placeholder")}
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
            label={_("email")}
            placeholder={_("email_placeholder")}
          />
        </Row>

        <TextInputField
          label={_("input_password_label")}
          type={showPassword ? "text" : "password"}
          {...FieldHelper.getFieldProps(
            form,
            validationSchema,
            FieldNames.PASSWORD,
          )}
          hint={<Button
            type="button"
            marginRight={16}
            appearance="minimal"
            iconBefore={showPassword ? EyeOnIcon : EyeOffIcon}
            onClick={changePasswordVisibility}
          >
            {_("input_show_password")}
          </Button>}
          placeholder={_("input_password_placeholder")}
        />
      </Section>

      {/* Localisation */}
      {isEditableModeEnabled && (
        <Section title={_("location")} borderBottom={hasFooter}>
          <Row>
            {/* Adresse */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.STREET,
              )}
              label={_("address")}
              placeholder={_("address_placeholder")}
            />
          </Row>
          <Row>
            {/* Complement d'adresse */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.ADDITIONAL_STREET,
              )}
              label={_("additional_address")}
              placeholder={_("additional_address_placeholder")}
            />
          </Row>
          <Row>
            {/* Ville */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.CITY,
              )}
              label={_("city")}
              placeholder={_("city_placeholder")}
            />

            {/* Code postal */}
            <TextInputField
              {...FieldHelper.getFieldProps(
                form,
                validationSchema,
                FieldNames.POSTAL_CODE,
              )}
              label={_("postal_code")}
              placeholder={_("postal_code_placeholder")}
            />
          </Row>
        </Section>
      )}

      {/* TODO Mettre le nom de la société et le lien vers les CGU */}
      <Text size={300}>{_("cgu_disclaimer")}</Text>

      <Button
        isLoading={form?.isSubmitting}
        appearance="primary"
        type="submit"
        marginTop={10}
        disabled={form?.isSubmitting || !form?.isValid}
        iconAfter={EndorsedIcon}
      >
        {_("action_register")}
      </Button>
    </Pane>
  );
};
