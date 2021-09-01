export {};
// import { FactoryHelper } from "../../helpers/factory-helper";
// import { Document } from "./document";

// export default {
//   title: "Design System/DocumentLeaseFurnished",
//   component: Document,
// };

// const property = FactoryHelper.propertyList()[1];

// const tenant = FactoryHelper.tenantList()[0];

// const lender = FactoryHelper.lenderList()[0];

// const withContainer = (
//   WrappedComponent,
//   delegate,
// ): React.FunctionComponent =>
//   () => {
//     const initialValues = {
//       property,
//       tenant,
//       propertyHousingType: PropertyUsageType.COLLECTIVE,
//       propertyUsage: PropertyHabitationUsageType.HABITATION,
//       buildingLegalStatus: PropertyBuildingLegalStatus.COPRO,
//       propertyEquipments: "",
//       propertyBuildPeriod: PropertyBuildPeriodType.FROM_1975_1989,
//       propertyOtherSpaces: "",
//       propertyUsageType: PropertyUsageType.INDIVIDUAL,
//       propertyHeatingMethod: PropertyUsageType.INDIVIDUAL,
//       propertyWaterHeatingMethod: PropertyUsageType.INDIVIDUAL,
//       propertyCommonSpaces: "",
//       propertyNticEquipments: "",
//       contractEffectDate: "03/05/2021",
//       contractRentAmount: 315,
//       contractRentMaxEvolutionRelocation: false,
//       contractRentMajorationDecree: false,
//       contractTenantLastRentAmount: 295,
//       contractRentMajDecreeReferenceAmount: 0,
//       contractRentMajDecreeIncreasedAmount: 0,
//       contractRentComplement: 0,
//       contractRentIrl: LeaseRentReferenceIrl.APRIL_FIRST_SEMESTER_2021,
//       contractRentChargesRecuperationMode: RentChargesRecuperationMode.PACKAGE,
//       contractRentChargesAmount: 0,
//       contractColocationInsuranceLender: false,
//       contractColocationInsuranceTotalAmount: 0,
//       contractColocationInsuranceMonthlyAmount: 0,
//       contractRentPeriodicity: LeaseRentPeriodicity.MONTHLY,
//       contractRentPaymentMethod: RentPaymentMethod.BEFORE,
//       contractRentPaymentDate: null,
//       contractRentPaymentPlace: null,
//       contractRentFirstAmount: null,
//       contractRentUnderestimatedMonthlyVariation: null,
//       contractRentUnderestimatedMethod: null,
//       contractWorksDecenceSinceLastRental: null,
//       contractWorksRentIncreaseLender: null,
//       contractWorksRentDecreaseTenant: null,
//       contractDepositAmount: null,
//       contractSolidarityClause: null,
//       contractResolutaryClause: null,
//       contractTenantFeeCapNewRental: null,
//       contractTenantFeeCapReportByMeter: null,
//       contractTenantFeeCapReportPrestations: null,
//       contractTenantFeeCapPrestations: null,
//       contractLenderFeeCapPrestations: null,
//       contractLenderFeeCap: null,
//       contractLenderFeeCapOther: null,
//       contractOtherConditions: null,
//     };

//     const validationSchema = ContractFurnishedDataValidator;

//     const [selectedIndex, setSelectedIndex] = useState(0);

//     const form = useFormik({
//       initialValues,
//       validationSchema,
//       onSubmit: () => {},
//     });

//     const fields = createLeaseFurnishedContract(
//       form,
//       validationSchema,
//       property,
//       tenant,
//       lender,
//     );

//     const getItem = (key: string) => fields[key];

//     const componentProps: DocumentProps = {
//       selectedIndex,
//       pages: [
//         LeaseFurnishedTemplatePart1({ form, getItem }),
//         LeaseFurnishedTemplatePart2({ form, getItem }),
//         LeaseFurnishedTemplatePart3({ form, getItem }),
//       ],
//       onClickPreviousPage: () => {
//         setSelectedIndex(selectedIndex - 1);
//         window.scrollTo(0, 0);
//       },
//       onClickNextPage: () => {
//         setSelectedIndex(selectedIndex + 1);
//         window.scrollTo(0, 0);
//       },
//       maxWidth: 800,
//       minHeight: 700,
//     };

//     return WrappedComponent(componentProps);
//   };

// FIXME: Wrapped the component with HOC cause error : Cannot read property 'largest' of undefined
// export const leaseFurnished = () => {
//   return (withContainer(Document, null)({}));
// };
