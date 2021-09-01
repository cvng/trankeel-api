import {
  Button,
  defaultTheme,
  Heading,
  Link,
  majorScale,
  Pane,
  Paragraph,
  Strong,
  Text,
  TextInputField,
} from "evergreen-ui";
import { FormikProps } from "formik";
import React from "react";
import Lottie from "react-lottie";
import { useMediaQuery } from "react-responsive";
import animationData from "../../assets/lotties/14583-multi-tasking.json";
import { Dimens } from "../../constants";
import { FieldHelper } from "../../helpers";
import { EmailValidationSchema } from "../../validators";
import { translate } from "piteo-kit";

const _ = translate();

const defaultOptions = {
  loop: true,
  autoplay: true,
  animationData: animationData,
  rendererSettings: {
    preserveAspectRatio: "xMidYMid meet",
  },
};

export enum FieldNames {
  EMAIL = "email",
}

export type LoginForgotPasswordFormProps = {
  /** Form */
  form: FormikProps<{
    email: string;
  }>;
  /** Validation schema */
  validationSchema: typeof EmailValidationSchema;
  /** Called on select login */
  onLoginButtonClick: () => void;
  /** Called after validated the form */
  hasValidateForm: boolean;
};

export const LoginForgotPasswordForm: React.FunctionComponent<
  LoginForgotPasswordFormProps
> = ({
  form,
  validationSchema,
  onLoginButtonClick,
  hasValidateForm,
}) => {
  const isDesktopOrLaptop = useMediaQuery({
    minDeviceWidth: Dimens.LARGE_TABLET_WIDTH,
  });
  return (
    <Pane display="flex" flex={1} height={window.innerHeight}>
      <Pane
        is="form"
        onSubmit={form.handleSubmit}
        display="flex"
        flex={1}
        flexDirection="column"
        alignItems="center"
        justifyContent="center"
        backgroundColor={defaultTheme.palette.blue.lightest}
      >
        <Pane
          display="flex"
          alignItems="left"
          flexDirection="column"
          width={300}
          flex={1}
        >
          <Pane
            display="flex"
            flexDirection="column"
            flex={1}
            justifyContent="center"
            justifyItems="center"
          >
            {hasValidateForm
              ? (
                <Pane>
                  <Heading size={600} marginBottom={20}>
                    {_("received_email")}
                  </Heading>
                  <Paragraph marginBottom={20}>
                    {_("received_email_hint")}
                  </Paragraph>
                  <Paragraph marginBottom={20}>
                    {_("received_email_message")}
                  </Paragraph>
                  <Strong marginBottom={20}>{_("app_email_contact")}</Strong>
                  <Paragraph marginTop={20} marginBottom={20}>
                    <Link onClick={onLoginButtonClick}>
                      {_("action_login")}
                    </Link>
                  </Paragraph>
                </Pane>
              )
              : (
                <Pane>
                  <Heading size={600} marginBottom={20}>
                    {_("heading")}
                  </Heading>
                  <Paragraph marginBottom={20}>
                    {_("forgot_password_helptext")}
                  </Paragraph>
                  <TextInputField
                    label={_("input_email_label")}
                    type="email"
                    {...FieldHelper.getFieldProps(
                      form,
                      validationSchema,
                      FieldNames.EMAIL,
                    )}
                    placeholder={_("input_email_placeholder")}
                  />

                  <Pane
                    display="flex"
                    flexDirection="row"
                    justifyContent="space-between"
                  >
                    <Button
                      type="submit"
                      isLoading={form.isSubmitting}
                      appearance="primary"
                    >
                      {_("action_submit")}
                    </Button>
                  </Pane>
                </Pane>
              )}
          </Pane>
        </Pane>
        <Pane marginBottom={30}>
          <Text>{_("already_account")}</Text>
          <Button
            appearance="primary"
            intent="success"
            marginX={majorScale(2)}
            onClick={onLoginButtonClick}
          >
            {_("action_login")}
          </Button>
        </Pane>
      </Pane>
      {isDesktopOrLaptop && <Pane
        display="flex"
        flexDirection="column"
        justifyContent="center"
        borderLeft="muted"
        alignItems="center"
        flex={1}
      >
        <Lottie options={defaultOptions} height={350} width={350} />
        <Heading size={600} marginY={majorScale(5)}>
          {_("hint_message")}
        </Heading>
      </Pane>}
    </Pane>
  );
};
