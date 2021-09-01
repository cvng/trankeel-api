import {
  ArrowLeftIcon,
  Button,
  defaultTheme,
  Heading,
  majorScale,
  Pane,
  Paragraph,
} from "evergreen-ui";
import { FormikProps } from "formik";
import { translate } from "piteo-kit";
import React from "react";
import { useMediaQuery } from "react-responsive";
import { Dimens } from "../../constants";
import { AccountIndividualForm } from "../account-owner-form/account-individual-form";
import { AppUnavailable } from "../common/app-unavailable";

const _ = translate();

export type LoginRegisterProps = {
  /** Form */
  form?: FormikProps<{
    firstName: string;
    lastName: string;
    email: string;
    password: string;
  }>;
  /** True if we display the password */
  showPassword?: boolean;
  /** Called on select the back button */
  onSelectBack?: () => void;
  /** Change the password visibility */
  changePasswordVisibility?: () => void;
};

export const LoginRegister: React.FunctionComponent<LoginRegisterProps> = ({
  form,
  showPassword,
  onSelectBack,
  changePasswordVisibility,
}) => {
  const isMobile = useMediaQuery({ maxWidth: Dimens.MOBILE_MAX_WIDTH });
  const isDesktopOrLaptop = useMediaQuery({
    minDeviceWidth: Dimens.LARGE_TABLET_WIDTH,
  });
  const individualForm = <AccountIndividualForm
    form={form}
    showPassword={showPassword}
    changePasswordVisibility={changePasswordVisibility}
  />;
  if (isMobile) {
    return <AppUnavailable />;
  }
  return (
    <Pane display="flex" flex={1} height={window.innerHeight}>
      <Pane
        display="flex"
        flexDirection="column"
        flex={1}
        alignItems="center"
        justifyContent="center"
        backgroundColor={defaultTheme.palette.blue.lightest}
      >
        <Pane display="flex" alignItems="left" flexDirection="column" flex={1}>
          <Pane
            display="flex"
            flexDirection="column"
            flex={1}
            justifyContent="center"
            justifyItems="center"
          >
            <Pane margin={30}>
              <Heading size={800} marginY={majorScale(3)}>
                {_("app_name")}
              </Heading>
              <Heading size={600} marginBottom={majorScale(1)}>
                {_("register_heading")}
              </Heading>
              <Paragraph marginBottom={20}>{_("register_helptext")}</Paragraph>
              <Pane display="flex" flex={1} alignItems={"center"}>
                {!isDesktopOrLaptop && individualForm}
              </Pane>
            </Pane>
          </Pane>
        </Pane>

        <Pane marginBottom={30}>
          <Button
            appearance="minimal"
            marginX={majorScale(2)}
            onClick={onSelectBack}
            iconBefore={ArrowLeftIcon}
          >
            {_("back")}
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
        {individualForm}
      </Pane>}
    </Pane>
  );
};
