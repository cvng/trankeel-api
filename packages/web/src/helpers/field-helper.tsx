// @ts-nocheck
import {
  Badge,
  majorScale,
  SelectField,
  SelectMenuItem,
  TextInputField,
} from "evergreen-ui";
import { FormikProps, getIn, setIn } from "formik";
import React from "react";
import { ObjectSchema, ValidationError } from "piteo-kit";
import { SegmentedControlField } from "../components/fields";

type Options = { name: string; type: string };

function isOptions(obj: unknown): obj is Options {
  return obj !== null && typeof obj === "object" &&
    ("name" in obj) && ("type" in obj);
}

export enum FieldSize {
  S = 80,
  M = 300,
  L = 500,
  XL = 600,
}

export class FieldHelper {
  static isRequiredField = (
    form: FormikProps<unknown>,
    validationSchema: ObjectSchema,
    name: string,
  ) => {
    // https://github.com/jquense/yup#mixedvalidatesyncatpath-string-value-any-options-object-any
    const isRequiredCheck = (error) =>
      error instanceof ValidationError && error.type === "required";
    const testedValues = setIn(form.values || {}, name, undefined);
    try {
      validationSchema.validateSyncAt(name, testedValues);
      return false;
    } catch (error) {
      return isRequiredCheck(error);
    }
  };

  static getFieldProps = (
    form?: FormikProps<unknown>,
    validationSchema?: ObjectSchema,
    nameOrOptions?: string | Options,
  ) => {
    if (!form) {
      return;
    }
    // https://github.com/jaredpalmer/formik/blob/master/packages/formik/src/Formik.tsx#L900
    const name = isOptions(nameOrOptions) ? nameOrOptions.name : nameOrOptions;

    const field = {
      ...form.getFieldProps(nameOrOptions),
      validationMessage: getIn(form.errors, name),
      isInvalid: !!getIn(form.errors, name),
      required: FieldHelper.isRequiredField(form, validationSchema, name),
    };

    if (isOptions(nameOrOptions) && nameOrOptions.type === "checkbox") {
      field.value = field.value.toString();
    }

    if (isOptions(nameOrOptions) && nameOrOptions.type === "control") {
      field.onChange = (value) => form.setFieldValue(name, value);
    }

    if (isOptions(nameOrOptions) && nameOrOptions.type === "file") {
      field.onChange = (value) => form.setFieldValue(name, value);
      field.onBlur = () => form.setFieldTouched(name, true);
    }

    return field;
  };

  static mapOptions = (
    map: Map<string, string> | Array<[string, string]> = [],
    firstOption: string = null,
  ) => {
    if (map instanceof Map) map = Array.from(map);

    return [["", firstOption], ...map].map(([value, label]) => (
      <option key={value} value={value}>
        {label}
      </option>
    ));
  };

  static mapValues = (map: Map<string, string>): SelectMenuItem[] => {
    return [...map].map(([value, label]) => ({ label, value }));
  };

  static textItem = ((content: string): React.ReactNode => {
    return <Badge color="blue" isSolid>{content}</Badge>;
  });

  static inputField = (
    form: FormikProps<unknown>,
    validationSchema: ObjectSchema,
    fieldName: string,
    placeholder?: string,
    hint?: string,
    width?: number,
  ): React.ReactNode => {
    return <TextInputField
      {...FieldHelper.getFieldProps(
        form,
        validationSchema,
        fieldName,
      )}
      placeholder={placeholder}
      maxWidth={width || 300}
      hint={hint}
      label={""}
    />;
  };

  static segmentedField = (
    form: FormikProps<unknown>,
    validationSchema: ObjectSchema,
    fieldName: string,
    map: Map<string, string>,
    width?: number,
  ): React.ReactNode => {
    return <SegmentedControlField
      {...FieldHelper.getFieldProps(
        form,
        validationSchema,
        { "name": fieldName, "type": "control" },
      )}
      width={width || 200}
      options={FieldHelper.mapValues(map)}
      marginY={majorScale(2)}
      label={""}
    />;
  };

  static selectField = (
    form: FormikProps<unknown>,
    validationSchema: ObjectSchema,
    fieldName: string,
    map: Map<string, string>,
    placeholder?: string,
    width?: number,
  ): React.ReactNode => {
    return <SelectField
      {...FieldHelper.getFieldProps(
        form,
        validationSchema,
        fieldName,
      )}
      width={width || 200}
      marginY={majorScale(2)}
      label={""}
    >
      {FieldHelper.mapOptions(map, placeholder)}
    </SelectField>;
  };
}
