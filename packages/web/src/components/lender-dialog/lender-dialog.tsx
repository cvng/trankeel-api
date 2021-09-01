import { Dialog } from "evergreen-ui";
import React from "react";
import { translate } from "piteo-kit";
import { LenderIndividualUpdateInput } from "../../types";
import { LenderForm, LenderFormProps } from "../lender-form/lender-form";

const _ = translate();

export type LenderDialogProps<T> = {
  /** True if shown */
  isShown?: boolean;
  /** Title */
  title?: string;
  /** Fired on close complete */
  onCloseComplete?: () => void;
} & LenderFormProps<T>;

export const LenderDialog: React.FunctionComponent<
  LenderDialogProps<LenderIndividualUpdateInput>
> = ({
  form,
  validationSchema,
  isShown = true,
  isPhysicalPerson = true,
  onCloseComplete = () => null,
  changeLegalEntityType = () => null,
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
    <LenderForm
      form={form}
      validationSchema={validationSchema}
      hasFooter={false}
      isPhysicalPerson={isPhysicalPerson}
      changeLegalEntityType={changeLegalEntityType}
    />
  </Dialog>
);
