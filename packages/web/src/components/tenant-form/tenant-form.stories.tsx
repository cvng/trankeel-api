import React from "react";
import { Themable } from "../common/themable";
import { TenantForm } from "./tenant-form";

export default {
  title: "Tenant/TenantForm",
  component: TenantForm,
};

// const tenant = FactoryHelper.tenantList()[0];

// const withContainer = (
//   WrappedComponent,
// ): React.FunctionComponent =>
//   () => {
//     const initialValues: TenantInput = {
//       firstName: tenant.firstName,
//       lastName: tenant.lastName,
//       phoneNumber: tenant.phoneNumber,
//       email: tenant.email,
//       note: tenant.note,
//       visaleId: "",
//       apl: true,
//       birthdate: null,
//       birthplace: "",
//     };

//     const validationSchema = TenantValidator;

//     const form = useFormik({
//       initialValues,
//       validationSchema,
//       onSubmit: () => {},
//     });

//     const componentProps: TenantFormProps = {
//       form,
//       validationSchema,
//       hasFooter: false,
//     };

//     return WrappedComponent(componentProps);
//   };

// FIXME: Wrapped the component with HOC cause error : Cannot read property 'medium' of undefined
// export const standard = withContainer(TenantForm);

export const ui = () => (
  <Themable>
    <TenantForm form={null} validationSchema={null} hasFooter={false} />
  </Themable>
);
