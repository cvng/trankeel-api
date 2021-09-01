import {
  Button,
  CaretDownIcon,
  DownloadIcon,
  EndorsedIcon,
  EnvelopeIcon,
  EraserIcon,
  Menu,
  Pane,
  PaperclipIcon,
  Popover,
  Position,
} from "evergreen-ui";
import React from "react";
import { Lease, RentStatus, Tenant } from "../../types";
import { Loading } from "../loading/loading";
import { translate } from "piteo-kit";

const _ = translate();

export type RentReceivedMenuPopinProps = {
  tenant: Tenant;
  rentStatus: RentStatus;
  contract: Lease;
  loading?: boolean;
  showRentalContract?: (contract: Lease) => void;
  markRentAsPaid?: (paid: boolean, contract: Lease) => void;
  sendReceipt?: (paid: boolean, contract: Lease) => void;
  downloadReceipt?: () => void;
};

export const RentReceivedMenuPopin: React.FunctionComponent<
  RentReceivedMenuPopinProps
> = ({
  tenant,
  rentStatus,
  contract,
  loading,
  showRentalContract,
  markRentAsPaid,
  sendReceipt,
  downloadReceipt,
}) => {
  const isPaid = rentStatus === RentStatus.Settled;
  return (
    <Pane display="flex" alignItems="center">
      <Popover
        position={Position.BOTTOM_LEFT}
        content={({ close }) => (
          <Menu>
            <Menu.Group title={_("rent")}>
              <Menu.Item
                icon={isPaid ? EraserIcon : EndorsedIcon}
                onSelect={() => {
                  markRentAsPaid && markRentAsPaid(isPaid, contract);
                  close();
                }}
              >
                {_(isPaid ? "mark_as_unpaid" : "mark_as_paid")}
              </Menu.Item>
            </Menu.Group>
            <Menu.Divider />
            <Menu.Group
              title={isPaid ? _("rent_receipt") : _("rent_notice")}
            >
              <Menu.Item icon={EnvelopeIcon}>
                {_(
                  isPaid ? "resend_rent_receipt" : "resend_rent_receipt_notice",
                )}
              </Menu.Item>
              <Menu.Item icon={DownloadIcon} onSelect={downloadReceipt}>
                {_(
                  isPaid
                    ? "download_rent_receipt"
                    : "download_rent_receipt_notice",
                )}
              </Menu.Item>
            </Menu.Group>
            <Menu.Divider />
            <Menu.Group title={"Contrat"}>
              <Menu.Item
                icon={PaperclipIcon}
                onSelect={() => {
                  showRentalContract(contract);
                  close();
                }}
              >
                {_("show_rental_contract")}
              </Menu.Item>
            </Menu.Group>
            <Menu.Divider />
          </Menu>
        )}
      >
        {loading
          ? (
            <Loading height={40} />
          )
          : (
            <Button appearance="primary" iconAfter={CaretDownIcon}>
              {_("shortcuts")}
            </Button>
          )}
      </Popover>
    </Pane>
  );
};
