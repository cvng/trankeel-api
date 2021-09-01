import {
  Button,
  DownloadIcon,
  Link,
  majorScale,
  Pane,
  Table,
  ThListIcon,
} from "evergreen-ui";
import moment from "moment";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { Tenant } from "../../types";
import { DATE_FORMAT } from "../../utils";
import { EmptyDataset } from "../common";
import { translate } from "piteo-kit";

const _ = translate();

type RentReceipt = { id; periodEnd; periodStart; file };

export type RentalReceiptListProps = {
  loading?: boolean;
  tenant?: Tenant;
  rentReceiptList?: RentReceipt[]; // TODO: RentalReceipt
};

export const RentalReceiptList: React.FunctionComponent<
  RentalReceiptListProps
> = ({
  loading,
  tenant,
  rentReceiptList,
}) => {
  return (
    <Pane>
      {loading && <Skeleton count={3} height={majorScale(4)} />}
      {rentReceiptList?.length > 0 && (
        <Table>
          <Table.Head>
            <Table.TextHeaderCell>{_("last_receipts")}</Table.TextHeaderCell>
            <Table.TextHeaderCell
              flex="none"
              width={200}
            >
            </Table.TextHeaderCell>
          </Table.Head>
          <Table.Body>
            {rentReceiptList?.map((rentalReceipt) => (
              <Table.Row key={rentalReceipt.id} onSelect={() => {}}>
                <Table.TextCell>
                  {_("receipt_period", {
                    periodStart: moment(rentalReceipt.periodStart).format(
                      DATE_FORMAT,
                    ),
                    periodEnd: moment(rentalReceipt.periodEnd).format(
                      DATE_FORMAT,
                    ),
                  })}
                </Table.TextCell>
                <Table.TextCell flex="none" width={200}>
                  <Link
                    textDecoration={"none"}
                    href={rentalReceipt.file?.url}
                    target="_blank"
                  >
                    <Button marginRight={12} iconBefore={DownloadIcon}>
                      {_("open_doc")}
                    </Button>
                  </Link>
                </Table.TextCell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table>
      )}
      {rentReceiptList?.length === 0 && !loading && (
        <EmptyDataset
          display="flex"
          justifyContent="flex-start"
          icon={ThListIcon}
          title={_("no_rent_receipt_available_title")}
          subtitle={_("no_rent_receipt_available_subtitle", {
            tenantName: tenant?.firstName,
          })}
        />
      )}
    </Pane>
  );
};
