import {
  FormField,
  Heading,
  ListItem,
  majorScale,
  Pane,
  Text,
  TickIcon,
  UnorderedList,
} from "evergreen-ui";
import { FormikProps } from "formik";
import React, { FunctionComponent } from "react";
import ReactPlayer from "react-player";
import { FieldHelper } from "../../helpers";
import { ImportInput } from "../../types";
import { ImportValidator } from "../../validators";
import { FormFooter, Section } from "../common";
import { FilePickerField } from "../fields";
import { Workbook } from "./parse-workbook";
import { translate } from "piteo-kit";

const _ = translate();

const HOW_TO_IMPORT = "https://www.youtube.com/watch?v=t3fw9jkfs-E";

export enum FieldNames {
  FILES = "files",
}

export type ImportFormProps = {
  /** Form */
  form: FormikProps<ImportInput>;
  /** Validation schema */
  validationSchema: typeof ImportValidator;
  /** When true, the footer is shown */
  hasFooter?: boolean;
  /** Imported workbook summary */
  workbook?: Workbook;
};

export const ImportForm: FunctionComponent<ImportFormProps> = ({
  form,
  validationSchema,
  hasFooter = false,
  workbook = null,
}) => {
  return (
    <Pane is="form" onSubmit={form.handleSubmit}>
      <Pane marginBottom={majorScale(2)}>
        <Heading size={300}>{_("source_hint")}</Heading>
        <Text>{_("source_hint_2")}</Text>
      </Pane>
      <ReactPlayer url={HOW_TO_IMPORT} playing width="100%" />
      <Section marginTop={majorScale(1)}>
        <FilePickerField
          {...FieldHelper.getFieldProps(form, validationSchema, {
            name: FieldNames.FILES,
            type: "file",
          })}
          accept=".xlsx"
          label={_("select_file")}
        />
      </Section>
      <Section title={_("summary")} borderBottom={hasFooter}>
        {workbook && (
          <FormField label={_("workbook")} hint={_("privacy_note")}>
            <UnorderedList icon={TickIcon} iconColor="success">
              <ListItem>{_("properties_found", workbook)}</ListItem>
              <ListItem>{_("tenants_found", workbook)}</ListItem>
              <ListItem>{_("rentals_found", workbook)}</ListItem>
            </UnorderedList>
          </FormField>
        )}
      </Section>

      {hasFooter && <FormFooter form={form} />}
    </Pane>
  );
};
