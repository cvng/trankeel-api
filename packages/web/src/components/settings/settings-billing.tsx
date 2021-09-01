import { QueryResult } from "@apollo/client";
import {
  Alert,
  Button,
  Card,
  CreditCardIcon,
  defaultTheme,
  DownloadIcon,
  Heading,
  IconButton,
  Link,
  ListItem,
  majorScale,
  Pane,
  Table,
  TickIcon,
  UnorderedList,
} from "evergreen-ui";
import moment from "moment";
import React, { FunctionComponent, useContext } from "react";
import Skeleton from "react-loading-skeleton";
import { AuthContext } from "../../context/auth-context";
import { DateHelper, SubscriptionHelper } from "../../helpers";
import { InvoiceListQuery, PlanCode, SubscriptionStatus } from "../../types";
import { Section } from "../common";
import { AmountLabel } from "../common/amount-label";
import { translate } from "piteo-kit";

const _ = translate();

export type SettingsBillingProps = {
  /** The query result */
  result: QueryResult<InvoiceListQuery>;
  /** Fired when clicking "activate" button */
  onClickActivate?: () => void;
};

export const SettingsBilling: FunctionComponent<SettingsBillingProps> = ({
  // account,
  // invoices,
  result,
  onClickActivate = () => null,
}) => {
  const context = useContext(AuthContext);
  const account = context?.user?.account;
  return (
    <Pane flex={1}>
      {/* Current plan */}
      <Section title={_("current_plan")}>
        {account?.status === SubscriptionStatus.Trialing && (
          <Alert
            appearance="card"
            intent="danger"
            marginBottom={majorScale(2)}
          >
            {_("warning_trial_end", {
              dayNb: moment(account?.trialEnd).diff(moment(), "days"),
            })}
          </Alert>
        )}
        {account?.status === SubscriptionStatus.Active && (
          <Alert appearance="card" intent="none" marginBottom={majorScale(2)}>
            {_("info_update_plan")}
          </Alert>
        )}

        <Card
          backgroundColor={defaultTheme.colors.background.tint2}
          padding={majorScale(3)}
        >
          <Heading size={500}>
            {result?.loading && <Skeleton />}
            {account?.plan && _("subscription_plan", { plan: account?.plan })}
            {account?.status === SubscriptionStatus.Trialing &&
              _("subscription_trial")}
          </Heading>

          {account?.plan &&
            Object.values(PlanCode).includes(
              account?.plan?.code as PlanCode,
            ) && (
              <UnorderedList
                icon={TickIcon}
                iconColor="success"
                marginBottom={majorScale(2)}
              >
                {SubscriptionHelper.planDetails()
                  .get(account?.plan?.code as PlanCode)
                  .map((details, index) => (
                    <ListItem key={index}>{details}</ListItem>
                  ))}
              </UnorderedList>
            )}

          <Button
            disabled={account?.status === SubscriptionStatus.Active}
            marginTop={majorScale(3)}
            iconBefore={CreditCardIcon}
            appearance="primary"
            intent="success"
            onClick={onClickActivate}
          >
            {_("activate_my_subscription")}
          </Button>
        </Card>
      </Section>

      {/* Billing history */}
      <Section title={_("billing_history")} borderBottom={false}>
        <Table>
          <Table.Head>
            <Table.TextHeaderCell>
              {_("transaction_number")}
            </Table.TextHeaderCell>
            <Table.TextHeaderCell>
              {_("transaction_type")}
            </Table.TextHeaderCell>
            <Table.TextHeaderCell>
              {_("transaction_end_date")}
            </Table.TextHeaderCell>
            <Table.TextHeaderCell>
              {_("transaction_amount")}
            </Table.TextHeaderCell>
            <Table.TextHeaderCell>{_("invoice_pdf")}</Table.TextHeaderCell>
          </Table.Head>
          <Table.Body height={240}>
            {(result?.data?.invoices || []).map((invoice) => (
              <Table.Row key={invoice.id} isSelectable>
                <Table.TextCell>
                  {invoice.number || <Skeleton />}
                </Table.TextCell>
                <Table.TextCell>
                  {invoice.planCode || <Skeleton />}
                </Table.TextCell>
                <Table.TextCell>
                  {DateHelper.formatToString(invoice.periodEnd) ||
                    <Skeleton />}
                </Table.TextCell>
                <Table.TextCell isNumber>
                  {result?.loading
                    ? (
                      <Skeleton />
                    )
                    : (
                      <AmountLabel value={invoice.amountPaid || 0} />
                    )}
                </Table.TextCell>
                <Table.TextCell>
                  <Link href={invoice.invoicePdf} target="_blank">
                    {invoice.invoicePdf
                      ? (
                        <IconButton icon={DownloadIcon} />
                      )
                      : (
                        <Skeleton />
                      )}
                  </Link>
                </Table.TextCell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table>
      </Section>
    </Pane>
  );
};
