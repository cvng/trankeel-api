import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { useState } from "react";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import { resetPassword } from "../../services/auth-service";
import { EmailValidationSchema } from "../../validators";
import { LoginForgotPasswordFormProps } from "./login-forgot-password-form";

type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();
    const [hasValidateForm, setValidateForm] = useState(false);

    const handleLogin = (): void => {
      history.push(Routes.LOGIN);
    };

    const handleSubmit = (): void => {
      const { email } = form.values;
      // Le traitement est identique dans les deux cas
      resetPassword(email)
        .then(() => {})
        .catch((error) => {
          toaster.danger(error.message);
        })
        .finally(() => {
          form.setSubmitting(false);
          setValidateForm(true);
        });
    };

    const validationSchema = EmailValidationSchema;

    const form = useFormik({
      initialValues: {
        email: "",
      },
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: LoginForgotPasswordFormProps = {
      form,
      validationSchema,
      onLoginButtonClick: handleLogin,
      hasValidateForm,
    };

    return WrappedComponent(componentProps);
  };
