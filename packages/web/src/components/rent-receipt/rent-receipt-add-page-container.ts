import { useMutation } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import moment from "moment";
import { toLocaleDateString } from "piteo-kit";
import React, { useState } from "react";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import {
  TransactionCreateMutation,
  TransactionCreateMutationOptions,
} from "../../helpers";
import { Tenant, TransactionInput, TransactionType } from "../../types";
import { uncast } from "../../utils";
import {
  DATE_ISO_FORMAT,
  TransactionRentValidator,
  TransactionValidator,
} from "../../validators";
import { RentReceiptAddPageProps } from "./rent-receipt-add-page";
import { RentReceiptPreviewProps } from "./rent-receipt-preview";
import { translate } from "piteo-kit";

const _ = translate("Transaction");

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();
    const [transactionCreate] = useMutation(
      TransactionCreateMutation,
      TransactionCreateMutationOptions(),
    );
    // const [rentalReceiptCreate] = useMutation(
    //   RentalReceiptCreateMutation,
    //   RentalReceiptCreateMutationOptions()
    // );
    // const rentalReceiptQuery = useQuery(RentalReceiptDataQuery);
    // const lenderIdentityListResult = useQuery(LenderListQuery);
    // const isMissingAddress =
    //   lenderIdentityListResult.data?.lenderIdentities?.length > 0 &&
    //   !AddressValidator.isValidSync(
    //     lenderIdentityListResult.data?.lenderIdentities?.[0]?.address
    //   );

    const [selectedTenants, setSelectedTenants] = useState([]);
    const [currentSelectedTenant, setCurrentSelectedTenant] = useState(null);
    const [isAllTenantsSelected, setAllTenantsSelected] = useState(false);
    const [isTourEnabled, setTourEnabled] = useState(false);
    const [isGeneratingRentReceipt, setIsGeneratingRentReceipt] = useState(
      false,
    );

    const tenantList = [];

    const firstMonthDate = moment().startOf("month");
    const lastMonthDate = moment().endOf("month");

    const initialValues = uncast({
      propertyId: currentSelectedTenant?.contract?.property?.id,
      tenantId: currentSelectedTenant?.id,
      type: TransactionType.Rent,
      amount: currentSelectedTenant?.contract?.rentAmount,
      chargesAmount: currentSelectedTenant?.contract?.rentChargesAmount,
      date: moment()
        .startOf("day")
        .format(DATE_ISO_FORMAT),
      periodStart: firstMonthDate.format(DATE_ISO_FORMAT),
      periodEnd: lastMonthDate.format(DATE_ISO_FORMAT),
    });

    // Lorsque le formulaire est validé
    // on envoit la requête d'ajout de transaction
    // et ensuite on envoit la requête permettant de générer une quittance
    const handleSubmit = async (values: TransactionInput): Promise<void> => {
      const transactionInput = TransactionValidator.cast(values, {
        stripUnknown: true,
      });
      // const rentalReceiptInput = RentalReceiptValidator.cast(values, {
      //   stripUnknown: true
      // });
      try {
        setIsGeneratingRentReceipt(true);
        // Envoi de la requête d'ajout de loyer
        await transactionCreate({ variables: { input: transactionInput } });
        // // Ensuite on enchaine sur la génération de la quittance
        // await rentalReceiptCreate({ variables: { input: rentalReceiptInput } });
        // toaster.success(
        //   _("transaction_rent_add_success", {
        //     amount: values.rentFullAmount,
        //     tenantFullName: currentSelectedTenant?.fullName
        //   }),
        //   {
        //     description: _("transaction_rent_add_success_subtext"),
        //     duration: 10
        //   }
        // );
        setIsGeneratingRentReceipt(false);
        handleAfterGenerateRentalReceipt();
      } catch {
        toaster.danger(_("error_smi"));
      }
    };

    const handleSelectAllTenantsClick = () => {
      if (isAllTenantsSelected) {
        setSelectedTenants([]);
        updateSelectedTenant(null);
      } else {
        // On ne sélectionne que les locataires possédant un contrat de location
        setSelectedTenants([
          ...tenantList?.filter((tenant) => tenant.contract != null),
        ]);
        updateSelectedTenant(tenantList?.[0]);
      }
      setAllTenantsSelected(!isAllTenantsSelected);
    };

    const handleSelectTenantClick = (tenant: Tenant) => {
      if (selectedTenants.includes(tenant)) {
        handleUnselectTenant(tenant);
      } else {
        handleSelectTenant(tenant);
      }
    };

    const handleUnselectTenant = (tenant: Tenant) => {
      const selectedTenantsCopy = [...selectedTenants];
      const index = selectedTenantsCopy.indexOf(tenant, 0);
      if (index > -1) {
        selectedTenantsCopy.splice(index, 1);
      }
      if (selectedTenantsCopy.length > 0) {
        const firstSelectedTenant = selectedTenantsCopy[0];
        updateSelectedTenant(firstSelectedTenant);
        handleNav(0);
      } else {
        updateSelectedTenant(null);
      }
      setSelectedTenants(selectedTenantsCopy);
      setAllTenantsSelected(false);
    };

    const handleSelectTenant = (tenant: Tenant) => {
      const selectedTenantsCopy = [...selectedTenants];
      selectedTenantsCopy.push(tenant);
      updateSelectedTenant(selectedTenantsCopy[0]);
      setSelectedTenants(selectedTenantsCopy);
    };

    const handleNav = (index) => {
      const tenant = selectedTenants[index];
      updateSelectedTenant(tenant);
    };

    const handleAfterGenerateRentalReceipt = () => {
      if (
        selectedTenants.indexOf(currentSelectedTenant) + 1 <
          selectedTenants.length
      ) {
        handleNav(selectedTenants.indexOf(currentSelectedTenant) + 1);
      }
    };

    const handleUpdateAddressClick = () => {
      history.push(Routes.SETTINGS_LENDERS);
    };

    const updateSelectedTenant = (tenant?: Tenant) => {
      setCurrentSelectedTenant(tenant);
    };

    // Retourne vrai si on est sur une période de location partielle (mois incomplet)
    const isPartialRental = (): boolean => {
      let isPartialRental = false;
      if (form?.values?.periodStart && form?.values?.periodEnd) {
        const rentNbDays = moment(form?.values?.periodEnd).diff(
          form?.values?.periodStart,
          "days",
        ) + 1;
        const monthNbDays = lastMonthDate.diff(firstMonthDate, "days") + 1;
        if (rentNbDays < monthNbDays) {
          isPartialRental = true;
        }
      }
      return isPartialRental;
    };

    const partialRentalData = () => {
      if (form?.values?.periodStart && form?.values?.periodEnd) {
        const rentAmount = currentSelectedTenant?.contract?.rentAmount?.amount;
        const chargesAmount = currentSelectedTenant?.contract?.rentChargesAmount
          ?.amount;
        const rentNbDays = moment(form?.values?.periodEnd).diff(
          form?.values?.periodStart,
          "days",
        ) + 1;
        const monthNbDays = lastMonthDate.diff(firstMonthDate, "days") + 1;
        const calculatedRentAmount = (rentNbDays / monthNbDays) * rentAmount;
        const calculatedChargesAmount = (rentNbDays / monthNbDays) *
          chargesAmount;
        return {
          amount: Math.round(Number(calculatedRentAmount)),
          chargesAmount: Math.round(Number(calculatedChargesAmount)),
          rentFullAmount: Math.round(
            Number(calculatedRentAmount + calculatedChargesAmount),
          ),
          days: rentNbDays,
        };
      }
    };

    const handleProrataRentClick = () => {
      const data = partialRentalData();
      form.setFieldValue("amount", { amount: data.amount });
      form.setFieldValue("chargesAmount", { amount: data.chargesAmount });
    };

    const validationSchema = TransactionRentValidator;

    const rentReceiptValue = (tenant?: Tenant): RentReceiptPreviewProps => {
      if (!tenant) {
        return;
      }
      const values = form?.values;
      let fullAmount = 0;
      if (values.amount && values.chargesAmount) {
        fullAmount = Number(values.amount.amount) +
          Number(values.chargesAmount.amount);
      }
      const lender = null; // TODO: Get data from account
      const property = tenant?.lease?.property;
      return uncast({
        periodStart: toLocaleDateString(values.periodStart),
        periodEnd: toLocaleDateString(values.periodEnd),
        rentAmount: values?.amount?.amount || 0,
        rentChargesAmount: values?.chargesAmount?.amount || 0,
        rentFullAmount: fullAmount || 0,
        lender: {
          name: lender?.name || _("lender_identity"),
          address: {
            line1: lender?.address?.line1 || _("lender_address"),
            line2: lender?.address?.line2,
            postalCode: lender?.address?.postalCode,
            city: lender?.address?.city,
          },
        },
        tenant: tenant,
        property: {
          address: {
            line1: property?.address?.line1 || _("tenant_address"),
            line2: property?.address?.line2,
            postalCode: property?.address?.postalCode,
            city: property?.address?.city,
          },
        },
      });
    };

    const form = useFormik({
      validateOnMount: true,
      enableReinitialize: true,
      initialValues,
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: RentReceiptAddPageProps = {
      form,
      rentReceipt: rentReceiptValue(currentSelectedTenant),
      validationSchema,
      tenants: tenantList,
      // isLoadingTenants: rentalReceiptQuery.loading,
      isLoadingTenants: true,
      currentSelectedTenant,
      isAllTenantsSelected,
      selectedTenants,
      isMissingAddress: true,
      isTourEnabled,
      isGeneratingRentReceipt,
      isPartialRental: isPartialRental(),
      partialRentalData: partialRentalData(),
      setTourEnabled,
      onSelectAllTenantsClick: handleSelectAllTenantsClick,
      onSelectTenantClick: handleSelectTenantClick,
      onSelectPreviousTenant: () =>
        handleNav(selectedTenants.indexOf(currentSelectedTenant) - 1),
      onSelectNextTenant: () =>
        handleNav(selectedTenants.indexOf(currentSelectedTenant) + 1),
      onUpdateAddressButtonClick: handleUpdateAddressClick,
      useProrataRentButtonClick: handleProrataRentClick,
    };

    return WrappedComponent(componentProps);
  };
