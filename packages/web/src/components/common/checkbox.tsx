import { Checkbox, majorScale } from "evergreen-ui";
import React from "react";

export const CheckBoxComponent = (props: {
  fieldName;
  value;
  label;
  formik;
  checked;
}) => {
  const { fieldName, value, label, formik, checked } = props;
  return (
    <React.Fragment>
      <Checkbox
        {...formik.getFieldProps(fieldName)}
        key={value}
        checked={checked}
        label={label}
        value={value}
        marginRight={majorScale(3)}
      />
    </React.Fragment>
  );
};

export const CheckboxesComponent = (props: {
  formik;
  fieldName: string;
  arr: { checked: boolean; value: React.ReactText; label: string }[];
}) => {
  const { formik, fieldName, arr } = props;
  return arr.map((item) => (
    <CheckBoxComponent
      key={item.value}
      formik={formik}
      checked={item.checked}
      fieldName={fieldName}
      value={item.value}
      label={item.label}
    />
  ));
};
