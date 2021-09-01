import { useMutation } from "@apollo/client";
import { majorScale, Pane } from "evergreen-ui";
import { FormikProps, useFormik } from "formik";
import React, { FunctionComponent } from "react";
import { FieldHelper, FileUploadMutation } from "../../helpers";
import { translate } from "piteo-kit";
import { FileValidator } from "../../validators";
import { FormFooter, PageContent, Section } from "../common";
import { FilePickerField } from "../fields/file-picker-field";

export type FileInput = {
  file: unknown;
};

// container

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): FunctionComponent<WrappedComponentProps> =>
  () => {
    const initialValues = { file: undefined };
    const validationSchema = FileValidator;

    const [upload] = useMutation(FileUploadMutation);

    const handleSubmit = async (values: FileInput): Promise<void> => {
      const input = validationSchema.cast(values);
      // @ts-ignore: https://github.com/piteo-team/piteo/pull/166
      const file = input.file[0];
      // @ts-ignore: https://github.com/piteo-team/piteo/pull/166
      await upload({ variables: { input: { url: file.url } } });
    };

    const form = useFormik({
      initialValues,
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: FileUploadPageProps = {
      form,
      validationSchema,
    };

    return WrappedComponent(componentProps);
  };

// form

export enum FieldNames {
  FILE = "file",
}

export type UploadFormProps = {
  /** Form */
  form: FormikProps<FileInput>;
  /** Validation schema */
  validationSchema?: typeof FileValidator;
  /** True if has footer */
  hasFooter?: boolean;
};

export const UploadForm: FunctionComponent<UploadFormProps> = ({
  form,
  validationSchema = null,
  hasFooter = true,
}) => {
  return (
    <Pane is="form" onSubmit={form.handleSubmit}>
      <Section title={_("upload_file")}>
        <FilePickerField
          {...FieldHelper.getFieldProps(form, validationSchema, {
            name: FieldNames.FILE,
            type: "file",
          })}
          label={_("file")}
        />
      </Section>

      {hasFooter && <FormFooter form={form} />}
    </Pane>
  );
};

// component

const _ = translate();

export type FileUploadPageProps = {
  /** Form */
  form: FormikProps<FileInput>;
  /** Validation schema */
  validationSchema?: typeof FileValidator;
};

export let FileUploadPage: FunctionComponent<FileUploadPageProps> = ({
  form,
  validationSchema = null,
}) => {
  return (
    <PageContent title={_("upload_file")}>
      <Pane display="flex" marginTop={majorScale(3)}>
        <UploadForm form={form} validationSchema={validationSchema} />
      </Pane>
    </PageContent>
  );
};

FileUploadPage = withContainer(FileUploadPage);
