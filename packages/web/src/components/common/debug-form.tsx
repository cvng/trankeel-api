import { Pane, Text } from "evergreen-ui";
import { FormikProps } from "formik";
import * as React from "react";

export const DebugForm: React.FunctionComponent<
  {
    form?: FormikProps<unknown>;
    filterProperty?: string;
    excludeProps?: string[];
  }
> = ({
  form,
  filterProperty,
  excludeProps,
}) => {
  let values = filterProperty ? form?.values[filterProperty] : form?.values;

  // On specific case when there big objects in the form
  // we want to exclude some properties in the response to debug more easily
  if (excludeProps?.length > 0) {
    const copyValues: Record<string, string> = {};
    for (const key in values) {
      if (!excludeProps.includes(key)) {
        copyValues[key] = values[key];
      }
    }
    values = copyValues;
  }

  return (
    <React.Fragment>
      {<Pane marginBottom={20}>
        <Text color="red">error: {JSON.stringify(form?.errors)}</Text>
      </Pane>}
      {<Pane>
        <Text color="blue">values: {JSON.stringify(values)}</Text>
      </Pane>}
    </React.Fragment>
  );
};
