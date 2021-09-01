import { ThemeProvider } from "evergreen-ui";
import React from "react";
import { AppTheme } from "../../theme/app-theme";
import { PropertyForm } from "./property-form";

export default {
  title: "Property/PropertyForm",
  component: PropertyForm,
};

// const withContainer = (
//   WrappedComponent,
// ): React.FunctionComponent =>
//   () => {
//     const initialValues: PropertyInput = {
//       name: "MONTMOREAU",
//       address: {
//         line1: "22 Rue du palais",
//         line2: "Etg2 Bat C",
//         postalCode: "33700",
//         city: "Bordeaux",
//       },
//       surface: 23,
//       buildPeriod: PropertyBuildPeriodType.FROM_1949_1974,
//       roomCount: PropertyRoomType.T1,
//       buildingLegalStatus: PropertyBuildingLegalStatus.MONO,
//       housingType: PropertyUsageType.INDIVIDUAL,
//       usageType: PropertyHabitationUsageType.HABITATION,
//       heatingMethod: PropertyUsageType.COLLECTIVE,
//       waterHeatingMethod: PropertyUsageType.INDIVIDUAL,
//       energyClass: PropertyEnergyClass.A,
//       gasEmission: PropertyGasEmission.B,
//       note: "une note à l'attention du bailleur",
//       tax: 521,
//       commonSpaces: "Espace commun",
//       tenantPrivateSpaces: "Garage privé",
//       otherSpaces: "Autres espaces",
//       equipments: "Cuisine équipée",
//       nticEquipments: "Box ADSL",
//     };

//     const validationSchema = PropertyValidator;

//     const form = useFormik({
//       initialValues,
//       validationSchema,
//       onSubmit: () => {},
//     });

//     const componentProps: PropertyFormProps = {
//       form,
//       validationSchema,
//       hasFooter: false,
//     };

//     return WrappedComponent(componentProps);
//   };

// FIXME: Wrapped the component with HOC cause error : Cannot read property 'largest' of undefined
// export const wrappedForm = () => (
//   <ThemeProvider value={AppTheme}>
//     {withContainer(PropertyForm)({})}
//   </ThemeProvider>
// );

export const standard = () => (
  <ThemeProvider value={AppTheme}>
    <PropertyForm hasFooter={false} form={null} />
  </ThemeProvider>
);
