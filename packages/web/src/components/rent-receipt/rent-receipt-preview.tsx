import {
  Alert,
  Button,
  Checkbox,
  ConfirmIcon,
  Heading,
  majorScale,
  minorScale,
  Pane,
  Paragraph,
  SideSheet,
  Text,
} from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { NumberHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Lease } from "../../types";
import { Closure, UniquableClosure } from "../../utils";

const _ = translate();

export type RentReceiptPreviewProps = {
  loading?: boolean;
  isShown?: boolean;
  lease?: Lease;
  periodStart?: string;
  periodEnd?: string;
  rentAmount?: number;
  rentChargesAmount?: number;
  rentFullAmount?: number;
  isNotice?: boolean;
  lenderMissingAddress?: boolean;
  sendMail?: boolean;
  onCloseComplete?: Closure;
  onConfirmSendReceipt?: Closure;
  onLenderAddMissingAddress?: UniquableClosure;
  onSendMailChange?: (checked: boolean) => void;
};

export const RentReceiptPreview: React.FunctionComponent<
  RentReceiptPreviewProps
> = ({
  loading = false,
  isShown = true,
  lease,
  periodStart = "-",
  periodEnd = "-",
  rentAmount = "-",
  rentChargesAmount = "-",
  rentFullAmount = "-",
  isNotice = false,
  lenderMissingAddress,
  sendMail,
  onCloseComplete,
  onConfirmSendReceipt,
  onLenderAddMissingAddress,
  onSendMailChange,
}) => {
  const theme = useAppTheme();
  const loadingState = () => {
    return (<Pane>
      <Skeleton width={200} />
      <Pane
        display="flex"
        justifyContent="space-between"
        marginTop={theme.margin.medium}
      >
        <Skeleton width={200} />
        <Skeleton width={200} />
      </Pane>
      <Pane display="flex" justifyContent="space-between">
        <Skeleton width={200} />
        <Skeleton width={200} />
      </Pane>
      <Pane display="flex" justifyContent="space-between">
        <Skeleton width={200} />
        <Skeleton width={200} />
      </Pane>
    </Pane>);
  };
  return (
    <SideSheet isShown={isShown} onCloseComplete={onCloseComplete} width={500}>
      {/* Title */}
      <Pane padding={16} borderBottom="muted">
        <Heading size={600} data-test-id="rent-receipt-title">
          {isNotice ? _("payment_notice_remind") : _("your_rent_receipt")}
        </Heading>
        {/* Subtitle */}
        <Paragraph size={400} color="muted">
          {isNotice
            ? _("rent_receipt_notice_subtitle")
            : _("your_rent_receipt_subtitle")}
        </Paragraph>
      </Pane>
      <Pane
        paddingX={theme.margin.large}
        paddingTop={theme.margin.medium}
        background="tint1"
        flex={1}
      >
        {/* Missing address on lender for the property */}
        {!loading && lenderMissingAddress &&
          <Alert
            appearance="card"
            intent="warning"
            marginY={theme.margin.medium}
          >
            <Pane>
              <Text>
                {_("lender_informations_missing_address", {
                  tenantName: lease?.tenants?.map((tenant) =>
                    tenant.displayName
                  )
                    .join(", "),
                })}
              </Text>
            </Pane>
            <Button
              marginTop={theme.margin.medium}
              appearance="primary"
              onClick={() =>
                onLenderAddMissingAddress?.(lease?.property?.lender?.id)}
            >
              {_("lender_missing_address_action")}
            </Button>
          </Alert>}
        {loading ? (loadingState()) : (
          <Pane
            elevation={3}
            padding={theme.margin.large}
            display="flex"
            flexDirection="column"
            background="white"
            marginTop={theme.margin.large}
            opacity={lenderMissingAddress ? 0.6 : 1}
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
                  <Heading size={400}>
                    {lease?.property?.lender?.displayName}
                  </Heading>
                  <Heading
                    size={400}
                    data-test-id="rent-receipt-lender-address"
                  >
                    {lease?.property?.lender?.identity?.address?.inline
                      ?.toLocaleUpperCase()}
                  </Heading>
                </Pane>
                <Pane
                  display="flex"
                  flexDirection="column"
                  flex={1}
                  alignItems="flex-end"
                  justifyContent="flex-start"
                >
                  <Heading size={400} textAlign="right">
                    {lease?.tenants?.map((tenant) => tenant.fullName).join(
                      ", ",
                    )}
                  </Heading>
                  <Heading
                    size={400}
                    textAlign="right"
                    data-test-id="rent-receipt-property-address"
                  >
                    {lease?.property?.address?.inline?.toLocaleUpperCase()}
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
                  <Heading size={400} data-test-id="rent-receipt-period">
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
                <Pane
                  display="flex"
                  flex={1}
                  flexWrap="wrap"
                  alignItems="center"
                >
                  <Text>{_("rent_amount")}</Text>
                </Pane>
                <Pane
                  display="flex"
                  flex={1}
                  flexWrap="wrap"
                  alignItems="center"
                  justifyContent="flex-end"
                >
                  <Text data-test-id="rent-receipt-rent-amount">
                    {NumberHelper.formatToString(rentAmount, false)}
                  </Text>
                </Pane>
              </Pane>
              {/* Charges */}
              <Pane
                height={30}
                display="flex"
                borderBottom="1px solid #DDDDDD"
                paddingX={minorScale(2)}
              >
                <Pane
                  display="flex"
                  flex={1}
                  flexWrap="wrap"
                  alignItems="center"
                >
                  <Text>{_("rent_charges_amount")}</Text>
                </Pane>
                <Pane
                  display="flex"
                  flex={1}
                  flexWrap="wrap"
                  alignItems="center"
                  justifyContent="flex-end"
                >
                  <Text data-test-id="rent-receipt-rent-charges-amount">
                    {NumberHelper.formatToString(rentChargesAmount, false)}
                  </Text>
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
                <Pane
                  display="flex"
                  flex={1}
                  flexWrap="wrap"
                  alignItems="center"
                >
                  <Heading size={400}>
                    {isNotice ? _("total_to_pay") : _("total_paid")}
                  </Heading>
                </Pane>
                <Pane
                  display="flex"
                  flex={1}
                  flexWrap="wrap"
                  alignItems="center"
                  justifyContent="flex-end"
                >
                  <Heading
                    size={400}
                    data-test-id="rent-receipt-rent-full-amount"
                  >
                    {NumberHelper.formatToString(rentFullAmount, false)}
                  </Heading>
                </Pane>
              </Pane>
            </Pane>
          </Pane>
        )}
        <Pane padding={theme.margin.medium}>
          {/* Rent receipt confirmation alert */}
          {!loading && !lenderMissingAddress && !isNotice &&
            <Alert
              appearance="card"
              intent="none"
              marginY={theme.margin.medium}
            >
              {_("rent_receipt_confirmation_before_send", {
                amount: NumberHelper.formatToString(
                  rentFullAmount,
                  false,
                ),
                tenantName: lease?.tenants?.map((tenant) => tenant.displayName)
                  .join(", "),
              })}
            </Alert>}

          {/* Payment notice alert */}
          {!loading && isNotice &&
            <Alert
              appearance="card"
              intent="warning"
              marginY={theme.margin.medium}
            >
              {_("rent_receipt_notice_alert", {
                periodStart,
                periodEnd,
                amount: NumberHelper.formatToString(
                  lease?.rentFullAmount,
                  false,
                ),
                tenantName: lease?.tenants?.map((tenant) => tenant.displayName)
                  .join(", "),
              })}
            </Alert>}
        </Pane>
        {/* Checkbox */}
        {!loading && !isNotice && <Checkbox
          checked={sendMail}
          marginLeft={10}
          label={_("send_rent_receipt_by_mail_tenant")}
          onChange={(e) => onSendMailChange?.(e.target.checked)}
        />}
        {!loading && <Pane
          flex={1}
          display="flex"
          justifyContent="center"
          paddingY={theme.margin.large}
        >
          <Button
            marginRight={theme.margin.large}
            onClick={onCloseComplete}
          >
            {_("cancel")}
          </Button>
          <Button
            iconBefore={ConfirmIcon}
            isLoading={loading}
            appearance="primary"
            onClick={onConfirmSendReceipt}
            disabled={lenderMissingAddress}
          >
            {isNotice ? _("remind_mail") : _("confirm_and_send")}
          </Button>
        </Pane>}
      </Pane>
    </SideSheet>
  );
};
