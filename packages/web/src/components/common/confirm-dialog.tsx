import { Dialog, IntentTypes, Pane } from "evergreen-ui";
import React from "react";
import { translate } from "piteo-kit";

const _ = translate("common");

export type ConfirmDialogProps = {
  title?: string;
  intent?: IntentTypes;
  message: string;
  isShown: boolean;
  isConfirmLoading: boolean;
  confirmLabel?: string;
  onConfirm: () => void;
  onCloseComplete: () => void;
};

export const ConfirmDialog: React.FunctionComponent<ConfirmDialogProps> = ({
  title,
  intent,
  message,
  isShown,
  isConfirmLoading,
  confirmLabel,
  onConfirm,
  onCloseComplete,
}) => (
  <Pane>
    <Dialog
      data-test-id={"confirm-dialog"}
      intent={intent || "danger"}
      isShown={isShown}
      onConfirm={onConfirm}
      isConfirmLoading={isConfirmLoading}
      onCloseComplete={onCloseComplete}
      title={title}
      confirmLabel={confirmLabel || _("delete")}
      cancelLabel={_("cancel")}
      hasCancel={!isConfirmLoading}
      hasClose={!isConfirmLoading}
    >
      {message}
    </Dialog>
  </Pane>
);
