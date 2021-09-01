import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import { useReferrer } from "../../hooks/user-referrer";
import { signInWithEmailAndPassword } from "../../services/auth-service";
import { LoginValidationSchema } from "../../validators";
import { LoginFormProps } from "./login-form";

type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();
    const referrer = useReferrer();

    const handleForgotPassword = (): void => {
      history.push(Routes.FORGOT_PASSWORD);
    };

    const handleRegister = (): void => {
      history.push(Routes.REGISTER);
    };

    const handleSubmit = (): void => {
      const { email, password, rememberMe } = form.values;
      // The redirection to the dashboard page or to the referrer
      // is not handle here. It's fire by a context modification
      // and the login form redirect on the desired screen
      signInWithEmailAndPassword(email, password, rememberMe)
        .then(() => {
          // Redirect to the referrer page
          history.push(referrer || Routes.DASHBOARD);
        })
        .catch((error) => {
          toaster.danger(error.message);
        })
        .finally(() => {
          form.setSubmitting(false);
        });
    };

    const validationSchema = LoginValidationSchema;

    const form = useFormik({
      initialValues: {
        email: "",
        password: "",
        rememberMe: false,
      },
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: LoginFormProps = {
      form,
      validationSchema,
      onForgotPasswordButtonClick: handleForgotPassword,
      onRegisterButtonClick: handleRegister,
    };

    return WrappedComponent(componentProps);
  };
