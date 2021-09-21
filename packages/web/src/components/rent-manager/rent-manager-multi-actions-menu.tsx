import { IssueClosedIcon, TickIcon } from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { PopinMenuButton } from "../buttons/popin-menu-button";

const _ = translate("");

export enum RentManagerMultiActionsMenuType {
  MarkAllAsPaid,
  RemindAll,
}

export type RentManagerMultiActionsMenuProps = {
  onSelectItem?: (type: RentManagerMultiActionsMenuType) => void;
};

export const RentManagerMultiActionsMenu: React.FunctionComponent<
  RentManagerMultiActionsMenuProps
> = ({
  onSelectItem,
}) => {
  return (<PopinMenuButton
    items={[
      {
        subItems: [
          {
            name: _("mark_all_as_paid"),
            handler: () =>
              onSelectItem?.(RentManagerMultiActionsMenuType.MarkAllAsPaid),
            icon: TickIcon,
            identifier: "mark-all-as-paid-button",
          },
          {
            name: _("remind_all_tenants"),
            handler: () =>
              onSelectItem?.(RentManagerMultiActionsMenuType.RemindAll),
            icon: IssueClosedIcon,
          },
        ],
      },
    ]}
  />);
};
