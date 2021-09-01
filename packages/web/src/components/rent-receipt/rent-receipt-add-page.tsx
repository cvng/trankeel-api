import {
  defaultTheme,
  HelpIcon,
  IconButton,
  majorScale,
  Pane,
  PeopleIcon,
} from "evergreen-ui";
import { FormikProps } from "formik";
import React from "react";
import Tour from "reactour";
import { Tenant, TransactionInput } from "../../types";
import { TransactionRentValidator } from "../../validators";
import { EmptyDataset, NAV_BAR_HEIGHT, PageContent } from "../common";
import { FlexRow } from "../common/flex-row";
import { RentReceiptDetailsFooter } from "./rent-receipt-details-footer";
import { RentReceiptDetailsHeader } from "./rent-receipt-details-header";
import { RentReceiptForm, RentReceiptPartial } from "./rent-receipt-form";
import {
  RentReceiptPreview,
  RentReceiptPreviewProps,
} from "./rent-receipt-preview";
import { RentReceiptTenantSelection } from "./rent-receipt-tenant-selection";
import { TourHelper } from "../../helpers/tour-helper";
import { TourType } from "../../constants/tour-constants";
import { translate } from "piteo-kit";

const _ = translate("Transaction");

const steps = [
  TourHelper.tourItem(TourType.TRANSACTION_RENT_ADD_SEL_TENANT),
  TourHelper.tourItem(TourType.TRANSACTION_RENT_ADD_SEL_RENT_LOCATION),
  TourHelper.tourItem(TourType.TRANSACTION_RENT_ADD_SEL_RENT_AMOUNT),
  TourHelper.tourItem(TourType.TRANSACTION_RENT_ADD_RECEIVED_RENT_DATE),
  TourHelper.tourItem(TourType.TRANSACTION_RENT_ADD_RENT_RECEIPT),
];

export type RentReceiptAddPageProps = {
  /** Form */
  form: FormikProps<TransactionInput>;
  /** Rent receipt value */
  rentReceipt: RentReceiptPreviewProps;
  /** Form validation schema */
  validationSchema: typeof TransactionRentValidator;
  /** Tenants */
  tenants: Tenant[];
  /** Is loading tenants */
  isLoadingTenants?: boolean;
  /** Is all tenants selected */
  isAllTenantsSelected?: boolean;
  /** Selected tenants */
  selectedTenants: Tenant[];
  /** Current selected tenant */
  currentSelectedTenant?: Tenant;
  /** When lender identity address is missing */
  isMissingAddress?: boolean;
  /** When tour is enabled */
  isTourEnabled?: boolean;
  /** When a rent receipt is being generating */
  isGeneratingRentReceipt?: boolean;
  /** True when the rental is being generating */
  isPartialRental?: boolean;
  /** Partial rent data */
  partialRentalData?: RentReceiptPartial;
  /** Called on enabled or disabled tour */
  setTourEnabled?: (enabled: boolean) => void;
  /** On select tenant click */
  onSelectTenantClick?: (tenant: Tenant) => void;
  /** On select all tenants */
  onSelectAllTenantsClick?: () => void;
  /** On select previous tenant */
  onSelectPreviousTenant?: () => void;
  /** On select next tenant */
  onSelectNextTenant?: () => void;
  /** On update address button click */
  onUpdateAddressButtonClick?: () => void;
  /** Called on select use prorata rent */
  useProrataRentButtonClick?: () => void;
};

export const RentReceiptAddPage: React.FunctionComponent<
  RentReceiptAddPageProps
> = ({
  form,
  rentReceipt,
  validationSchema,
  tenants = [],
  isLoadingTenants,
  isAllTenantsSelected,
  selectedTenants,
  currentSelectedTenant,
  isMissingAddress,
  isTourEnabled,
  isGeneratingRentReceipt,
  isPartialRental,
  partialRentalData = null,
  setTourEnabled,
  onSelectTenantClick,
  onSelectAllTenantsClick,
  onSelectPreviousTenant,
  onSelectNextTenant,
  onUpdateAddressButtonClick,
  useProrataRentButtonClick,
}) => (
  <PageContent
    title={_("add_rent")}
    disablePadding
    titleRightView={<Pane display="flex" flexDirection="row">
      {/* Aide */}
      <IconButton
        appearance="minimal"
        icon={HelpIcon}
        iconSize={18}
        onClick={() => setTourEnabled(true)}
        marginX={majorScale(1)}
      />
    </Pane>}
  >
    {/* Visite guidée */}
    <Tour
      steps={steps}
      accentColor={"#1259B1"}
      isOpen={isTourEnabled}
      rounded={5}
      onRequestClose={() => setTourEnabled(false)}
    />

    <FlexRow height={window.innerHeight - NAV_BAR_HEIGHT}>
      <RentReceiptTenantSelection
        isAllTenantsSelected={isAllTenantsSelected}
        selectedTenants={selectedTenants}
        tenants={tenants}
        onSelectTenantClick={onSelectTenantClick}
        loading={isLoadingTenants}
        onSelectAllTenantsClick={onSelectAllTenantsClick}
      />
      <Pane
        display="flex"
        flexDirection="column"
        flex={3}
        background={defaultTheme.palette.blue.lightest}
      >
        <Pane height={80}>
          <RentReceiptDetailsHeader tenant={currentSelectedTenant} />
        </Pane>

        {selectedTenants?.length > 0
          ? (
            <Pane display="flex" flex={1} flexDirection="row">
              <Pane
                display="flex"
                width={400}
                flexDirection="column"
                background="white"
                borderRight="muted"
              >
                <RentReceiptForm
                  form={form}
                  validationSchema={validationSchema}
                  isMissingAddress={isMissingAddress}
                  disabled={isGeneratingRentReceipt}
                  isPartialRent={isPartialRental}
                  partialRentData={partialRentalData}
                  onUpdateAddressButtonClick={onUpdateAddressButtonClick}
                  useProrataRentButtonClick={useProrataRentButtonClick}
                />
              </Pane>
              {/* Quittance */}
              <Pane
                display="flex"
                flex={1}
                justifyContent="center"
                alignItems="center"
              >
                <Pane
                  minWidth={380}
                  className={TourType.TRANSACTION_RENT_ADD_RENT_RECEIPT}
                >
                  <RentReceiptPreview {...rentReceipt} />
                </Pane>
              </Pane>
            </Pane>
          )
          : (
            <Pane
              display="flex"
              flex={1}
              alignItems="center"
              justifyContent="center"
            >
              <EmptyDataset
                title={_("empty_placeholder_no_tenants_selected")}
                subtitle={_("empty_placeholder_no_tenants_selected_subtitle")}
                icon={PeopleIcon}
              />
            </Pane>
          )}

        <Pane height={80}>
          <RentReceiptDetailsFooter
            selectedTenants={selectedTenants}
            selectedTenant={currentSelectedTenant}
            validateButtonClick={form?.submitForm}
            validateButtonEnabled={form?.isValid}
            validateButtonLoading={isGeneratingRentReceipt}
            onPreviousButtonClick={onSelectPreviousTenant}
            onNextButtonClick={onSelectNextTenant}
          />
        </Pane>
      </Pane>
    </FlexRow>
  </PageContent>
);
