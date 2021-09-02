import { Dialog } from "evergreen-ui";
import { FormikProps } from "formik";
import React from "react";
import { TenantInput } from "../../types";
import { TenantValidator } from "../../validators";
import { TenantForm } from "../tenant-form";
import { translate } from "piteo-kit";

const _ = translate();

export type TenantDialogProps = {
  /** Form */
  form: FormikProps<TenantInput>;
  /** Validation schema */
  validationSchema: typeof TenantValidator;
  /** True if shown */
  isShown?: boolean;
  /** Fired on close complete */
  onCloseComplete?: () => void;
  /** Title */
  title?: string;
};

export const TenantDialog: React.FunctionComponent<TenantDialogProps> = ({
  form,
  validationSchema,
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
    <TenantForm
      form={form}
      validationSchema={validationSchema}
      hasFooter={false}
    />
  </Dialog>
);
