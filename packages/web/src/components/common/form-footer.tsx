import { Button, majorScale, Pane } from "evergreen-ui";
import { FormikProps } from "formik";
import { translate } from "piteo-kit";
import React from "react";
import { useRouter } from "../../hooks/use-router";

const _ = translate();

export type FormFooterProps = {
  /** Form */
  form: FormikProps<unknown>;
  /** Cancel text */
  cancelText?: string;
  /** Submit text */
  submitText?: string;
  /** Fired when clicking "cancel" button. Default = router.goBack() */
  onCancelClick?: (event: React.MouseEvent) => void;
};

export const FormFooter: React.FunctionComponent<FormFooterProps> = ({
  form,
  cancelText = _("cancel"),
  submitText = _("submit"),
  onCancelClick = null,
}) => {
  const router = useRouter();
  onCancelClick = onCancelClick || router.goBack;

  return (
    <Pane marginTop={majorScale(3)}>
      {/* Cancel */}
      <Button type="button" appearance="default" onClick={onCancelClick}>
        {cancelText}
      </Button>

      {/* Submit */}
      <Button
        type="submit"
        appearance="primary"
        disabled={!form.dirty || !form.isValid}
        isLoading={form.isSubmitting}
        marginLeft={majorScale(1)}
      >
        {submitText}
      </Button>
    </Pane>
  );
};
