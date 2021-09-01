import {
  Alert,
  Button,
  defaultTheme,
  Heading,
  majorScale,
  Pane,
  Text,
  TextInputField,
} from "evergreen-ui";
import { FormikProps } from "formik";
import React from "react";
import Lottie from "react-lottie";
import { useMediaQuery } from "react-responsive";
import animationData from "../../assets/lotties/14583-multi-tasking.json";
import { Dimens } from "../../constants";
import { FieldHelper, VersionNumberHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { isDemoEnv } from "../../utils";
import { LoginValidationSchema } from "../../validators";
import { translate } from "piteo-kit";
import { AppUnavailable } from "../common/app-unavailable";

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
  PASSWORD = "password",
  REMEMBER_ME = "rememberMe",
}

export type LoginFormProps = {
  /** Form */
  form: FormikProps<{
    email: string;
    password: string;
    rememberMe: boolean;
  }>;
  /** Validation schema */
  validationSchema: typeof LoginValidationSchema;
  /** Called on select forgot password */
  onForgotPasswordButtonClick: () => void;
  /** Called on select register */
  onRegisterButtonClick: () => void;
};

export const LoginForm: React.FunctionComponent<LoginFormProps> = ({
  form,
  validationSchema,
  onForgotPasswordButtonClick,
  onRegisterButtonClick,
}) => {
  const theme = useAppTheme();
  const isDemo = isDemoEnv();
  const isMobile = useMediaQuery({ maxWidth: Dimens.MOBILE_MAX_WIDTH });
  const isDesktopOrLaptop = useMediaQuery({
    minDeviceWidth: Dimens.LARGE_TABLET_WIDTH,
  });
  if (isMobile) {
    return <AppUnavailable />;
  }
  return (
    <Pane display="flex" flex={1} height={window.innerHeight}>
      <Pane
        is="form"
        onSubmit={form.handleSubmit}
        display="flex"
        flexDirection="column"
        flex={1}
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
            <Pane>
              <Heading size={800} marginY={majorScale(3)}>
                {_("app_name")}
              </Heading>
              {isDemo && (
                <Alert
                  appearance="card"
                  intent="none"
                  marginBottom={majorScale(2)}
                >
                  {_("demo_alert")}
                </Alert>
              )}
              <TextInputField
                label={_("input_email_label")}
                type="email"
                {...FieldHelper.getFieldProps(
                  form,
                  validationSchema,
                  FieldNames.EMAIL,
                )}
                disabled={form.isSubmitting}
                placeholder={_("input_email_placeholder")}
              />

              <TextInputField
                type="password"
                label={_("input_password_label")}
                {...FieldHelper.getFieldProps(
                  form,
                  validationSchema,
                  FieldNames.PASSWORD,
                )}
                disabled={form.isSubmitting}
                placeholder={_("input_password_placeholder")}
              />

              <Pane
                display="flex"
                flexDirection="row"
                justifyContent="space-between"
                marginTop={10}
              >
                <Button
                  appearance="primary"
                  isLoading={form.isSubmitting}
                  type="submit"
                >
                  {_("action_login")}
                </Button>
                <Button
                  appearance="minimal"
                  onClick={onForgotPasswordButtonClick}
                  disabled={form.isSubmitting}
                >
                  {_("action_forgot_password")}
                </Button>
              </Pane>
            </Pane>
          </Pane>
        </Pane>
        {!isDemo && <Pane marginBottom={30}>
          <Text>{_("no_account")}</Text>
          <Button
            appearance="primary"
            intent="success"
            disabled={form.isSubmitting}
            onClick={onRegisterButtonClick}
            marginX={theme.margin.medium}
          >
            {_("action_register")}
          </Button>
        </Pane>}
        <Pane marginBottom={20}>
          <Heading size={100}>
            {VersionNumberHelper.fullVersionNumber()}
          </Heading>
        </Pane>
      </Pane>
      {isDesktopOrLaptop && <Pane
        display="flex"
        flex={1}
        borderLeft="muted"
        alignItems="center"
        justifyContent="center"
        flexDirection="column"
        padding={theme.margin.largest}
      >
        <Heading size={600}>{_("slogan")}</Heading>
        <Lottie options={defaultOptions} height={500} width={500} />
      </Pane>}
    </Pane>
  );
};
