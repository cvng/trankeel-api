import { Heading, majorScale, minorScale, Pane, Text } from "evergreen-ui";
import React from "react";
import { Lender, Property, Tenant } from "../../types";
import { translate } from "piteo-kit";

const _ = translate();

export type RentReceiptPreviewProps = {
  lender?: Lender;
  tenant?: Tenant;
  property?: Property;
  periodStart?: string;
  periodEnd?: string;
  rentAmount?: number;
  rentChargesAmount?: number;
  rentFullAmount?: number;
  isNotice?: boolean;
};

export const RentReceiptPreview: React.FunctionComponent<
  RentReceiptPreviewProps
> = ({
  lender,
  tenant,
  property,
  periodStart = "-",
  periodEnd = "-",
  rentAmount = "-",
  rentChargesAmount = "-",
  rentFullAmount = "-",
  isNotice = false,
}) => {
  return (
    <Pane
      elevation={2}
      paddingX={15}
      paddingY={50}
      display="flex"
      flexDirection="column"
      background="white"
      maxWidth={500}
    >
      <Pane flex={1} flexDirection="column">
        <Pane flex={1}>
          <Heading size={600} marginBottom={majorScale(4)}>
            {isNotice ? _("rent_receipt_notice") : _("rent_receipt")}
          </Heading>
        </Pane>
        <Pane flex={1} display="flex" marginBottom={majorScale(4)}>
          <Pane
            display="flex"
            flexDirection="column"
            flex={1}
            alignItems="flex-start"
          >
            {
              /* <Heading size={400}>{lender?.name}</Heading>
            <Heading size={400}>{lender?.address?.line1}</Heading>
            {lender?.address?.line2 && (
              <Heading size={400}>{lender?.address?.line2}</Heading>
            )}
            <Heading size={400}>
              {lender?.address?.postalCode}
              {lender?.address?.city}
            </Heading> */
            }
          </Pane>
          <Pane
            display="flex"
            flexDirection="column"
            flex={1}
            alignItems="flex-end"
            justifyContent="flex-start"
          >
            <Heading size={400} textAlign="right">
              {tenant?.fullName}
            </Heading>
            <Heading size={400} textAlign="right">
              {property?.address?.line1}
            </Heading>
            {property?.address?.line2 && (
              <Heading size={400}>
                {property?.address?.line2}
              </Heading>
            )}
            <Heading size={400} textAlign="right">
              {property?.address?.postalCode}
              {property?.address?.city}
            </Heading>
          </Pane>
        </Pane>
        {/* Période */}
        <Pane
          display="flex"
          flexWrap="wrap"
          background="#EEEEEE"
          borderBottom="1px solid #DDDDDD"
        >
          <Pane
            display="flex"
            flex={1}
            alignItems="center"
            paddingX={minorScale(2)}
          >
            <Heading size={400}>{_("period")}</Heading>
          </Pane>
          <Pane
            display="flex"
            flex={3}
            alignItems="center"
            justifyContent="flex-end"
            paddingX={minorScale(2)}
          >
            <Heading size={400}>
              {_("date_period", {
                periodStart: periodStart,
                periodEnd: periodEnd,
              })}
            </Heading>
          </Pane>
        </Pane>
        {/* Loyer */}
        <Pane
          height={30}
          display="flex"
          borderBottom="1px solid #DDDDDD"
          paddingX={minorScale(2)}
        >
          <Pane display="flex" flex={1} flexWrap="wrap" alignItems="center">
            <Text>{_("rent_amount")}</Text>
          </Pane>
          <Pane
            display="flex"
            flex={1}
            flexWrap="wrap"
            alignItems="center"
            justifyContent="flex-end"
          >
            <Text>{_("amount_format", { amount: rentAmount })}</Text>
          </Pane>
        </Pane>
        {/* Charges */}
        <Pane
          height={30}
          display="flex"
          borderBottom="1px solid #DDDDDD"
          paddingX={minorScale(2)}
        >
          <Pane display="flex" flex={1} flexWrap="wrap" alignItems="center">
            <Text>{_("rent_charges_amount")}</Text>
          </Pane>
          <Pane
            display="flex"
            flex={1}
            flexWrap="wrap"
            alignItems="center"
            justifyContent="flex-end"
          >
            <Text>{_("amount_format", { amount: rentChargesAmount })}</Text>
          </Pane>
        </Pane>
        {/* Total */}
        <Pane
          height={30}
          display="flex"
          background="#EEEEEE"
          borderBottom="1px solid #DDDDDD"
          paddingX={minorScale(2)}
        >
          <Pane display="flex" flex={1} flexWrap="wrap" alignItems="center">
            <Heading size={400}>
              {isNotice ? _("total_to_pay") : _("total")}
            </Heading>
          </Pane>
          <Pane
            display="flex"
            flex={1}
            flexWrap="wrap"
            alignItems="center"
            justifyContent="flex-end"
          >
            <Heading size={400}>
              {_("amount_format", { amount: rentFullAmount })}
            </Heading>
          </Pane>
        </Pane>
      </Pane>
    </Pane>
  );
};
