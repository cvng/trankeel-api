import { CornerDialog, majorScale, Pane, Paragraph } from "evergreen-ui";
import React, { FunctionComponent } from "react";
// import { CircularProgressbar } from "react-circular-progressbar"
// import "react-circular-progressbar/dist/styles.css"
import { translate } from "piteo-kit";
import { withContainer } from "./import-status-container";

const _ = translate();

export type ImportStatusCornerDialogProps = {
  /** Progress */
  progress: number;
  /** When true, the dialog is shown */
  isShown?: boolean;
  /** Function called when confirm button is clicked */
  onConfirm?: () => void;
  /** Function called when close is complete */
  onCloseComplete?: () => void;
};

export const ImportStatusCornerDialog: FunctionComponent<
  ImportStatusCornerDialogProps
> = ({
  progress = 0,
  isShown = true,
  onConfirm = () => null,
  onCloseComplete = () => null,
}) => {
  return (
    <CornerDialog
      isShown={isShown}
      onConfirm={onConfirm}
      onCloseComplete={onCloseComplete}
      title={_("import_status_title")}
      confirmLabel={_("see_more")}
      cancelLabel={_("close")}
    >
      <Pane
        display="flex"
        flexDirection="row"
        justifyContent="center"
        marginBottom={majorScale(2)}
      >
        <Pane width={100} height={100}>
          {/* <CircularProgressbar value={progress} text={NumberHelper.formatToPercentage(progress, 0)} /> */}
        </Pane>
      </Pane>
      <Paragraph>{_("import_status_text")}</Paragraph>
    </CornerDialog>
  );
};

export default withContainer(ImportStatusCornerDialog);
