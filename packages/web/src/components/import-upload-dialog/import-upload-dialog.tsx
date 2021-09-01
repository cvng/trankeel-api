import { Dialog } from "evergreen-ui";
import { FormikProps } from "formik";
import React, { FunctionComponent } from "react";
import { translate } from "piteo-kit";
import { ImportInput } from "../../types";
import { ImportValidator } from "../../validators";
import { ImportForm } from "./import-form";
import { Workbook } from "./parse-workbook";

const _ = translate();

export type ImportUploadDialogProps = {
  /** Form */
  form: FormikProps<ImportInput>;
  /** Validation schema */
  validationSchema: typeof ImportValidator;
  /** When true, the dialog is shown */
  isShown?: boolean;
  /** Function called when close is complete */
  onCloseComplete?: () => void;
  /** Imported workbook summary */
  workbook?: Workbook;
};

export const ImportUploadDialog: FunctionComponent<ImportUploadDialogProps> = ({
  form,
  validationSchema,
  workbook = null,
  isShown = true,
  onCloseComplete = () => null,
}) => {
  return (
    <Dialog
      isShown={isShown}
      onConfirm={form.submitForm}
      isConfirmLoading={form.isSubmitting}
      isConfirmDisabled={!form.dirty || !form.isValid}
      onCloseComplete={onCloseComplete}
      title={_("import_data")}
      confirmLabel={_("confirm")}
      cancelLabel={_("cancel")}
    >
      <ImportForm
        form={form}
        validationSchema={validationSchema}
        workbook={workbook}
        hasFooter={false}
      />
    </Dialog>
  );
};
