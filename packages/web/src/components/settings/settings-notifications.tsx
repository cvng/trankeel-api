import { Button, Checkbox, Pane } from "evergreen-ui";
import { Formik, FormikProps } from "formik";
import { translate } from "piteo-kit";
import React from "react";

const _ = translate("SettingsComponent");

// TODO: Fixme
class ProfileNotificationInput {
  sendReceiptTenantAuto: boolean;
  receiveRentReceiptCopy: boolean;
}

export type SettingsNotificationsProps = {
  initialValues?: ProfileNotificationInput;
};

export const SettingsNotifications: React.FunctionComponent<
  SettingsNotificationsProps
> = ({
  initialValues,
}) => (
  <Formik
    initialValues={initialValues}
    validationSchema={{}}
    onSubmit={() => {}}
  >
    {(form: FormikProps<ProfileNotificationInput>) => (
      <Pane
        is="form"
        onSubmit={form.handleSubmit}
        flexDirection="column"
        alignItems="left"
      >
        <Pane>
          <Checkbox
            name="sendReceiptTenantAuto"
            disabled
            checked={form.values?.sendReceiptTenantAuto}
            label={_("send_receipt_tenant")}
            onChange={form.handleChange}
          />
          <Checkbox
            name="receiveRentReceiptCopy"
            disabled
            checked={form.values?.receiveRentReceiptCopy}
            label={_("receive_rent_receipt_copy")}
            onChange={form.handleChange}
          />
        </Pane>

        <Button
          disabled
          appearance="primary"
          marginTop={20}
          width={100}
          isLoading={form.isSubmitting}
          type="submit"
        >
          {_("save")}
        </Button>
      </Pane>
    )}
  </Formik>
);
