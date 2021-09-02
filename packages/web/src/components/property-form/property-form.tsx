import {
  FlameIcon,
  GeolocationIcon,
  Label,
  OfficeIcon,
  Pane,
  SelectField,
  TagInput,
  Text,
  Textarea,
  TextInputField,
  TorchIcon,
  TreeIcon,
} from "evergreen-ui";
import { FormikProps } from "formik";
import { PropertyHelper, translate } from "piteo-kit";
import React from "react";
import { FieldHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { PropertyInput } from "../../types";
import { PropertyValidator } from "../../validators";
import { FormFooter, Row, Section } from "../common";
import { SegmentedControlField } from "../fields";

const _ = translate();

const SMALL_FIELD_WIDTH = 100;
const MEDIUM_FIELD_WIDTH = 150;

export enum FieldNames {
  NAME = "name",
  LENDER_ID = "lenderId",
  SURFACE = "surface",
  STREET = "address.line1",
  ADDITIONAL_STREET = "address.line2",
  POSTAL_CODE = "address.postalCode",
  CITY = "address.city",
  BUILD_PERIOD = "buildPeriod",
  BUILDING_LEGAL_STATUS = "buildingLegalStatus",
  HOUSING_TYPE = "housingType",
  ROOM_COUNT = "roomCount",
  USAGE_TYPE = "usageType",
  HEATING_METHOD = "heatingMethod",
  WATER_HEATING_METHOD = "waterHeatingMethod",
  ENERGY_CLASS = "energyClass",
  GAS_EMISSION = "gasEmission",
  NOTE = "note",
  TAX = "tax",
  COMMON_SPACES = "commonSpaces",
  TENANT_PRIVATE_SPACES = "tenantPrivateSpaces",
  OTHER_SPACES = "otherSpaces",
  EQUIPMENTS = "equipments",
  EQUIPMENTS_NTIC = "nticEquipments",
}

export type PropertyFormProps = {
  /** Form */
  form?: FormikProps<PropertyInput>;
  /** Validation schema */
  validationSchema?: typeof PropertyValidator;
  /** True if has footer */
  hasFooter?: boolean;
  /** Lenders */
  lenderList?: Array<[string, string]>; // cf. select field mapOptions
};

export const PropertyForm: React.FunctionComponent<PropertyFormProps> = ({
  form,
  validationSchema,
  hasFooter = true,
  lenderList = [],
}) => {
  const theme = useAppTheme();
  // Remove the validationMessage property in the note props
  // as the textarea do not handle it
  const { validationMessage, ...noteFieldProps } = {
    ...FieldHelper.getFieldProps(
      form,
      validationSchema,
      FieldNames.NOTE,
    ),
  };
  return (
    <Pane
      background="white"
      padding={theme.margin.large}
      paddingTop={0}
    >
      <Section>
        <Row>
          {/* Nom */}
          <TextInputField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.NAME,
            )}
            label={_("property_name")}
            hint={_("property_name_hint")}
            placeholder={_("property_name_placeholder")}
          />
          <Pane />
        </Row>

        <Row>
          {/* Propriétaire */}
          <SelectField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.LENDER_ID,
            )}
            label={_("lender")}
            hint={_("lender_hint")}
          >
            {FieldHelper.mapOptions(lenderList, _("lender_placeholder"))}
          </SelectField>
          <Pane />
        </Row>

        <Row>
          {/* Nombre de pièces */}
          <SelectField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.ROOM_COUNT,
            )}
            label={_("property_room_count")}
          >
            {FieldHelper.mapOptions(
              PropertyHelper.roomPropertyTypes(),
              _("property_piece_count_placeholder"),
            )}
          </SelectField>
          {/* Surface */}
          <TextInputField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.SURFACE,
            )}
            type="number"
            label={_("property_surface")}
            hint={_("property_surface_hint")}
            placeholder={_("property_surface_placeholder")}
            maxWidth={MEDIUM_FIELD_WIDTH}
          />
        </Row>

        <Row>
          {/* Taxe foncière */}
          <TextInputField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.TAX,
            )}
            type="number"
            label={_("property_tax")}
            hint={_("property_tax_amount")}
            placeholder={_("property_tax_placeholder")}
            maxWidth={MEDIUM_FIELD_WIDTH}
          />
        </Row>
      </Section>

      <Section title={_("location")} icon={GeolocationIcon}>
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
          {/* Adresse additionnelle */}
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
            maxWidth={SMALL_FIELD_WIDTH}
          />
        </Row>
      </Section>

      <Section title={_("building")} icon={OfficeIcon}>
        <Row>
          {/* Régime juridique de l'immeuble */}
          <SegmentedControlField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              {
                "name": FieldNames.BUILDING_LEGAL_STATUS,
                "type": "control",
              },
            )}
            label={_("property_building_legal_status")}
            options={FieldHelper.mapValues(
              PropertyHelper.buildingLegalStatuses(),
            )}
          />

          {/* Type d'habitat */}
          <SegmentedControlField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              { "name": FieldNames.HOUSING_TYPE, "type": "control" },
            )}
            label={_("housing_type")}
            hint={_("housing_type_hint")}
            options={FieldHelper.mapValues(
              PropertyHelper.individualOrCollective(),
            )}
          />
        </Row>

        <Row>
          {/* Usage */}
          <SegmentedControlField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              {
                "name": FieldNames.USAGE_TYPE,
                "type": "control",
              },
            )}
            label={_("property_usage")}
            options={FieldHelper.mapValues(
              PropertyHelper.usageTypes(),
            )}
            hint={_("property_usage_hint")}
          />
          {/* Période de construction */}
          <SelectField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.BUILD_PERIOD,
            )}
            label={_("property_build_period")}
          >
            {FieldHelper.mapOptions(
              PropertyHelper.buildPeriods(),
              _("select_build_period"),
            )}
          </SelectField>
        </Row>
      </Section>
      <Section title={_("heating")} icon={FlameIcon}>
        <Row>
          {/* Type de chauffage */}
          <SegmentedControlField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              { "name": FieldNames.HEATING_METHOD, "type": "control" },
            )}
            label={_("heating_method")}
            options={FieldHelper.mapValues(
              PropertyHelper.individualOrCollective(),
            )}
          />

          {/* Type d’eau chaude */}
          <SegmentedControlField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              {
                "name": FieldNames.WATER_HEATING_METHOD,
                "type": "control",
              },
            )}
            label={_("water_heating_method")}
            options={FieldHelper.mapValues(
              PropertyHelper.individualOrCollective(),
            )}
          />
        </Row>
      </Section>

      <Section title={_("dpe")} icon={TorchIcon}>
        <Row>
          {/* Classe énergie */}
          <SelectField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.ENERGY_CLASS,
            )}
            label={_("energy_class")}
          >
            {FieldHelper.mapOptions(
              PropertyHelper.energyClassMap(),
              _("property_energy_class_placeholder"),
            )}
          </SelectField>

          {/* Emission de gaz à effet de serre */}
          <SelectField
            {...FieldHelper.getFieldProps(
              form,
              validationSchema,
              FieldNames.GAS_EMISSION,
            )}
            label={_("gas_emission")}
          >
            {FieldHelper.mapOptions(
              PropertyHelper.gasEmissionMap(),
              _("property_gas_emission_placeholder"),
            )}
          </SelectField>
        </Row>
      </Section>

      <Section title={_("spaces")} icon={TreeIcon}>
        <Row>
          {/* Tenant private spaces */}
          <Pane display="flex" flexDirection="column">
            <Label marginBottom={theme.margin.small}>
              {_("tenant_private_spaces")}
            </Label>
            <TagInput
              values={form?.values?.tenantPrivateSpaces?.split(",")}
              tagProps={{ color: "blue" }}
              inputProps={{
                placeholder: !form?.values?.tenantPrivateSpaces
                  ? _("tenant_private_spaces_placeholder")
                  : "",
              }}
              onChange={(values) =>
                form?.setFieldValue(
                  FieldNames.TENANT_PRIVATE_SPACES,
                  values.toString(),
                )}
            />
            <Text
              size={300}
              marginY={theme.margin.medium}
              color={theme.colors.text.muted}
            >
              {_("tenant_private_spaces_hint")}
            </Text>
          </Pane>
          {/* Common spaces */}
          <Pane display="flex" flexDirection="column">
            <Label marginBottom={theme.margin.small}>
              {_("common_spaces")}
            </Label>
            <TagInput
              values={form?.values?.commonSpaces?.split(",")}
              tagProps={{ color: "blue" }}
              inputProps={{
                placeholder: !form?.values?.commonSpaces
                  ? _("common_spaces_placeholder")
                  : "",
              }}
              onChange={(values) =>
                form?.setFieldValue(
                  FieldNames.COMMON_SPACES,
                  values.toString(),
                )}
            />
            <Text
              size={300}
              marginY={theme.margin.medium}
              color={theme.colors.text.muted}
            >
              {_("common_spaces_hint")}
            </Text>
          </Pane>
        </Row>

        {/* Other spaces */}
        <Pane
          display="flex"
          flexDirection="column"
          marginBottom={theme.margin.large}
        >
          <Label marginBottom={theme.margin.small}>
            {_("other_spaces")}
          </Label>
          <TagInput
            values={form?.values?.otherSpaces?.split(",")}
            tagProps={{ color: "blue" }}
            inputProps={{
              placeholder: !form?.values?.otherSpaces
                ? _("other_spaces_placeholder")
                : "",
            }}
            onChange={(values) =>
              form?.setFieldValue(
                FieldNames.OTHER_SPACES,
                values.toString(),
              )}
          />
        </Pane>
      </Section>

      <Section title={_("other")}>
        <Row>
          {/* Equipments */}
          <Pane display="flex" flexDirection="column">
            <Label marginBottom={theme.margin.small}>
              {_("equipments_details")}
            </Label>
            <TagInput
              values={form?.values?.equipments?.split(",")}
              tagProps={{ color: "blue" }}
              inputProps={{
                placeholder: !form?.values?.equipments
                  ? _("equipments_details_placeholder")
                  : "",
              }}
              onChange={(values) =>
                form?.setFieldValue(
                  FieldNames.EQUIPMENTS,
                  values.toString(),
                )}
            />
          </Pane>
          {/* Equipments NTIC */}
          <Pane display="flex" flexDirection="column">
            <Label marginBottom={theme.margin.small}>
              {_("equipments_ntic")}
            </Label>
            <TagInput
              values={form?.values?.nticEquipments?.split(",")}
              tagProps={{ color: "blue" }}
              inputProps={{
                placeholder: !form?.values?.nticEquipments
                  ? _("equipments_ntic_placeholder")
                  : "",
              }}
              onChange={(values) =>
                form?.setFieldValue(
                  FieldNames.EQUIPMENTS_NTIC,
                  values.toString(),
                )}
            />
            <Text
              size={300}
              marginY={theme.margin.medium}
              color={theme.colors.text.muted}
            >
              {_("equipments_ntic_hint")}
            </Text>
          </Pane>
        </Row>

        <Pane marginBottom={theme.margin.large}>
          <Label marginBottom={theme.margin.small}>
            {_("observations")}
          </Label>
          <Textarea
            {...noteFieldProps}
          />
        </Pane>
      </Section>

      {hasFooter && <FormFooter form={form} />}
    </Pane>
  );
};
