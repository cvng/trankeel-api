import { Dialog } from "evergreen-ui";
import { FormikProps } from "formik";
import React from "react";
import { PropertyInput } from "../../types";
import { PropertyValidator } from "../../validators";
import { PropertyForm } from "../property-form/property-form";
import { translate } from "piteo-kit";

const _ = translate();

export type PropertyDialogProps = {
  /** Form */
  form: FormikProps<PropertyInput>;
  /** Validation schema */
  validationSchema: typeof PropertyValidator;
  /** True if shown */
  isShown?: boolean;
  /** Fired on close complete */
  onCloseComplete?: () => void;
  /** Title */
  title?: string;
  /** Lender list */
  lenderList?: Array<[string, string]>;
};

export const PropertyDialog: React.FunctionComponent<PropertyDialogProps> = ({
  form,
  validationSchema,
  isShown = true,
  onCloseComplete = () => null,
  title,
  lenderList,
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
    width={700}
  >
    <PropertyForm
      form={form}
      validationSchema={validationSchema}
      hasFooter={false}
      lenderList={lenderList}
    />
  </Dialog>
);
