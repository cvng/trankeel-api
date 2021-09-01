import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { useState } from "react";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import { createUserWithAccount } from "../../services/auth-service";
import { UserWithAccountInput } from "../../types";
import { RegisterValidationSchema } from "piteo-kit";
import { LoginRegisterProps } from "./login-register";

type AccountCreateInput = Omit<UserWithAccountInput, "authId">;

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();
    const validationSchema = RegisterValidationSchema;
    const initialValues = validationSchema.cast({});

    const [showPassword, setShowPassword] = useState(false);

    const handleChangePasswordVisibility = (): void => {
      setShowPassword(!showPassword);
    };

    const handleClickBack = (): void => {
      history.replace(Routes.LOGIN);
    };

    const handleSubmit = async (values: AccountCreateInput): Promise<void> => {
      const accountInput = RegisterValidationSchema.cast(values, {
        stripUnknown: true,
      });
      // Call the service
      await createUserWithAccount(
        accountInput.firstName,
        accountInput.lastName,
        accountInput.email,
        accountInput.password,
      ).then((_) => {
        // Go to the authenticated route
        history.push(Routes.DEFAULT);
      }).catch((error) => {
        toaster.danger(error.message);
      });
    };

    const form = useFormik({
      initialValues,
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: LoginRegisterProps = {
      form,
      showPassword,
      onSelectBack: handleClickBack,
      changePasswordVisibility: handleChangePasswordVisibility,
    };

    return WrappedComponent(componentProps);
  };
