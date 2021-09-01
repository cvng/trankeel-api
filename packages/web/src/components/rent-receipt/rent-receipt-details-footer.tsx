import {
  ArrowLeftIcon,
  ArrowRightIcon,
  Button,
  majorScale,
  Pane,
  SavedIcon,
  Text,
} from "evergreen-ui";
import React from "react";
import { FlexRow } from "../common/flex-row";
import { Tenant } from "../../types";
import { EmptyItem } from "../common";
import { translate } from "piteo-kit";

const _ = translate("Transaction");

export type RentReceiptDetailsFooterProps = {
  selectedTenants?: Tenant[];
  selectedTenant?: Tenant;
  validateButtonLoading?: boolean;
  validateButtonEnabled: boolean;
  validateButtonClick: () => void;
  onPreviousButtonClick?: () => void;
  onNextButtonClick?: () => void;
};

export const RentReceiptDetailsFooter: React.FunctionComponent<
  RentReceiptDetailsFooterProps
> = ({
  selectedTenants,
  selectedTenant,
  validateButtonLoading,
  validateButtonEnabled,
  validateButtonClick,
  onPreviousButtonClick,
  onNextButtonClick,
}) => {
  const selectedTenantIndex = selectedTenants.indexOf(selectedTenant) + 1;
  return (
    <FlexRow
      height={80}
      justifyContent="space-between"
      alignItems="center"
      padding={majorScale(2)}
      borderTop="muted"
      borderBottom="muted"
      background="white"
    >
      <FlexRow alignItems="center">
        {selectedTenants?.length > 0
          ? (
            <Text size={400} marginRight={majorScale(2)}>
              {selectedTenantIndex}/{selectedTenants?.length}
            </Text>
          )
          : (
            <EmptyItem width={majorScale(4)} />
          )}
        <Button
          onClick={onPreviousButtonClick}
          height={24}
          iconBefore={ArrowLeftIcon}
          disabled={selectedTenantIndex - 1 === 0 ||
            selectedTenants.length === 0 ||
            selectedTenantIndex === 0}
        >
        </Button>
        <Button
          onClick={onNextButtonClick}
          height={24}
          iconAfter={ArrowRightIcon}
          disabled={selectedTenants.length === 0 ||
            selectedTenantIndex === selectedTenants.length}
        >
        </Button>
      </FlexRow>
      <Pane>
        <Button
          type="button"
          appearance="primary"
          disabled={selectedTenants.length === 0 || !validateButtonEnabled}
          onClick={validateButtonClick}
          isLoading={validateButtonLoading}
          iconBefore={SavedIcon}
          marginRight={majorScale(10)}
        >
          {_("generate_rent_receipt")}
        </Button>
      </Pane>
    </FlexRow>
  );
};
