import { Dialog } from "evergreen-ui";
import { FormikProps } from "formik";
import React from "react";
import { translate } from "piteo-kit";

const _ = translate();

export type FormDialogProps<T> = {
  /** Form */
  form: FormikProps<T>;
  /** Form component */
  formComponent: React.ReactNode;
  /** True if shown */
  isShown?: boolean;
  /** Fired on close complete */
  onCloseComplete?: () => void;
  /** Title */
  title?: string;
};

export const FormDialog: React.FunctionComponent<FormDialogProps<unknown>> = ({
  form,
  formComponent,
  isShown = true,
  onCloseComplete = () => null,
  title,
}) => (
  <Dialog
    isShown={isShown}
    onConfirm={form.submitForm}
    isConfirmLoading={form.isSubmitting}
    isConfirmDisabled={!form.dirty || !form.isValid}
    onCloseComplete={onCloseComplete}
    title={title}
    confirmLabel={_("confirm")}
    cancelLabel={_("cancel")}
  >
    {formComponent}
  </Dialog>
);
