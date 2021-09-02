import {
  Avatar,
  Badge,
  Heading,
  majorScale,
  minorScale,
  Pane,
  Text,
} from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { Tenant } from "../../types";
import { EmptyItem } from "../common";
import { AmountLabel } from "../common/amount-label";

const _ = translate("Transaction");

export type RentReceiptDetailsHeaderProps = {
  tenant: Tenant;
};

export const RentReceiptDetailsHeader: React.FunctionComponent<
  RentReceiptDetailsHeaderProps
> = ({
  tenant,
}) => {
  const rentDetails = (tenant) => {
    const hasActiveRent = !!tenant?.contract?.rentFullAmount;
    if (!tenant) {
      return <EmptyItem />;
    }

    if (hasActiveRent) {
      return (
        <Pane>
          <Heading size={100}>{_("rent_amount")}</Heading>
          <AmountLabel value={tenant?.contract?.rentFullAmount} />
        </Pane>
      );
    } else {
      return (
        <Badge marginTop={minorScale(1)} color="orange">
          {_("no_rent")}
        </Badge>
      );
    }
  };

  return (
    <Pane
      display="flex"
      height={80}
      borderBottom="muted"
      justifyContent="space-between"
      flexDirection="row"
      background={"white"}
      padding={majorScale(2)}
    >
      <Pane display="flex" alignItems="center">
        <Avatar
          name={tenant?.fullName}
          size={30}
          padding={10}
          marginRight={10}
        />
        <Text>{tenant?.fullName}</Text>
      </Pane>
      <Pane
        display="flex"
        flexDirection="row"
        alignItems="center"
        justifyContent="space-between"
        minWidth={300}
      >
        <Pane display="flex" flexDirection="column" alignItems="flex-start">
          <Heading size={100}>{_("property_name")}</Heading>
          {tenant?.propertyName
            ? (
              <Badge marginTop={minorScale(1)} color="blue">
                {tenant?.propertyName}
              </Badge>
            )
            : (
              <EmptyItem />
            )}
        </Pane>
        <Pane display="flex" flexDirection="column" alignItems="center">
          {rentDetails(tenant)}
        </Pane>
      </Pane>
    </Pane>
  );
};
