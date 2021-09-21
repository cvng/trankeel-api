import { EyeOpenIcon, IssueClosedIcon, TickIcon, UndoIcon } from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { RentStatus } from "../../types";
import {
  PopinMenuButton,
  PopinMenuButtonSubItemProps,
} from "../buttons/popin-menu-button";

const _ = translate("");

export enum RentManagerSingleActionsMenuType {
  MarkAsPaid,
  MarkAsUnpaid,
  Remind,
  ShowTenant,
}

export type RentManagerSingleActionsMenuProps = {
  status: RentStatus;
  onSelectItem?: (type: RentManagerSingleActionsMenuType) => void;
};

export const RentManagerSingleActionsMenu: React.FunctionComponent<
  RentManagerSingleActionsMenuProps
> = ({
  status,
  onSelectItem,
}) => {
  const unpaidRentSubItems: PopinMenuButtonSubItemProps[] = [
    // Rent receipt edition
    {
      name: _("edit_rent_receipt"),
      handler: () =>
        onSelectItem?.(RentManagerSingleActionsMenuType.MarkAsPaid),
      icon: TickIcon,
      identifier: "mark-as-paid",
    },
    {
      name: _("remind_tenant"),
      handler: () => onSelectItem?.(RentManagerSingleActionsMenuType.Remind),
      icon: IssueClosedIcon,
    },
  ];

  const paidRentSubItems: PopinMenuButtonSubItemProps[] = [
    {
      name: _("mark_as_unpaid"),
      handler: () =>
        onSelectItem?.(RentManagerSingleActionsMenuType.MarkAsUnpaid),
      icon: UndoIcon,
      identifier: "mark-as-unpaid",
    },
  ];

  const defaultSubItems: PopinMenuButtonSubItemProps[] = [
    {
      name: _("show_tenant"),
      handler: () =>
        onSelectItem?.(
          RentManagerSingleActionsMenuType.ShowTenant,
        ),
      icon: EyeOpenIcon,
    },
  ];
  const subItems = [].concat(
    status === RentStatus.Settled ? paidRentSubItems : unpaidRentSubItems,
  ).concat(defaultSubItems);

  return (<PopinMenuButton
    items={[
      {
        subItems,
      },
    ]}
  />);
};
