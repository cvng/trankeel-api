import {
  Alert,
  GeolocationIcon,
  OfficeIcon,
  Pane,
  PersonIcon,
  SelectField,
  TextInputField,
} from "evergreen-ui";
import { FormikProps } from "formik";
import { LenderHelper, translate } from "piteo-kit";
import React from "react";
import physicalPersonAnimation from "../../assets/lotties/28654-happy-freelancer.json";
import moralPersonAnimation from "../../assets/lotties/40815-building.json";
import { FieldHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { LegalEntityType, LenderIndividualUpdateInput } from "../../types";
import { Closure } from "../../utils";
import { LenderIndividualValidator } from "../../validators";
import { CardSelector } from "../cards/card-selector";
import { Row, Section } from "../common";

const _ = translate();

export enum FieldNames {
  FIRST_NAME = "individual.firstName",
  LAST_NAME = "individual.lastName",
  PHONE_NUMBER = "individual.phoneNumber",
  STREET = "individual.address.line1",
  ADDITIONAL_STREET = "individual.address.line2",
  POSTAL_CODE = "individual.address.postalCode",
  CITY = "individual.address.city",
  LEGAL_ENTITY = "company.legalEntity",
  LEGAL_ENTITY_ID = "company.legalEntityIdentifier",
  LEGAL_ENTITY_TYPE = "company.legalEntityType",
  LEGAL_ENTITY_TYPE_OTHER = "company.legalEntityTypeOther",
}

export type LenderFormProps<T> = {
  /** Form */
  form?: FormikProps<T>;
  /** Validation schema */
  validationSchema?: typeof LenderIndividualValidator;
  /** True if has footer */
  hasFooter?: boolean;
  /** True if physical person */
  isPhysicalPerson?: boolean;
  /** Change the legal entity type */
  changeLegalEntityType?: Closure;
};

export const LenderForm: React.FunctionComponent<
  LenderFormProps<LenderIndividualUpdateInput>
> = ({
  form,
  validationSchema,
  hasFooter = true,
  isPhysicalPerson = true,
  changeLegalEntityType,
}) => {
  const theme = useAppTheme();
  return (
    <Pane is="form" onSubmit={form?.handleSubmit}>
      <Pane
        display="flex"
        justifyContent="center"
        marginBottom={theme.margin.large}
      >
        <CardSelector
          items={[{
            id: "0",
            title: _("physical_person"),
            selected: isPhysicalPerson,
            animation: physicalPersonAnimation,
          }, {
            id: "1",
            title: _("moral_person"),
            selected: !isPhysicalPerson,
            animation: moralPersonAnimation,
          }]}
          onClickItem={changeLegalEntityType}
        />
      </Pane>
      <Alert
        intent="warning"
        title={_("lender_informations_hint")}
        marginBottom={theme.margin.largest}
      />
      {/* Informations personnelles */}
      <Section
        title={_("informations")}
        icon={isPhysicalPerson ? PersonIcon : OfficeIcon}
      >
        {isPhysicalPerson
          ? // Personne physique
            (<Row>
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
            </Row>)
          : (
            // Personne morale
            <Pane>
              <Row>
                {/* Nom de la société */}
                <TextInputField
                  {...FieldHelper.getFieldProps(
                    form,
                    validationSchema,
                    "FieldNames.COMPANY_LEGAL_NAME",
                  )}
                  label={_("legal_entity_name")}
                  placeholder={_("legal_entity_name_placeholder")}
                />
              </Row>
              <Row>
                {/* Forme juridique */}
                <SelectField
                  {...FieldHelper.getFieldProps(
                    form,
                    validationSchema,
                    "FieldNames.ROOM_COUNT",
                  )}
                  label={_("legal_entity_type")}
                >
                  {FieldHelper.mapOptions(
                    LenderHelper.legalEntityTypes(),
                    _("legal_entity_type_placeholder"),
                  )}
                </SelectField>

                {/* Numéro de SIRET */}
                <TextInputField
                  {...FieldHelper.getFieldProps(
                    form,
                    validationSchema,
                    "FieldNames.COMPANY_LEGAL_ENTITY_ID",
                  )}
                  label={_("legal_entity_identifier")}
                  placeholder={_("legal_entity_identifier_placeholder")}
                />
              </Row>

              {/* Autre forme juridique */}
              {form?.values?.["legalEntityType"] === LegalEntityType.Other &&
                <TextInputField
                  {...FieldHelper.getFieldProps(
                    form,
                    validationSchema,
                    "FieldNames.COMPANY_LEGAL_ENTITY_ID",
                  )}
                  label={_("legal_entity_type_other_label")}
                  placeholder={_("legal_entity_type_other_placeholder")}
                />}
            </Pane>
          )}

        {/* <Row> */}
        {/* Email */}
        {
          /* <TextInputField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.EMAIL,
            )}
            label={_("email")}
            placeholder={_("email_placeholder")}
          />
          <Pane />
        </Row> */
        }
      </Section>

      {/* Localisation */}

      <Section
        title={_("location")}
        borderBottom={hasFooter}
        icon={GeolocationIcon}
      >
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
    </Pane>
  );
};
