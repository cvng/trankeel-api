import {
  Button,
  Card,
  LockIcon,
  majorScale,
  Pane,
  PeopleIcon,
  PlusIcon,
  SavedIcon,
} from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { TourType } from "../../constants/tour-constants";

const _ = translate();

export type ShortcutsWidgetProps = {
  available: boolean;
  onAddRentClick?: () => void;
  onTenantAddClick?: () => void;
  onRentalAddClick?: () => void;
};

export const ShortcutsWidget: React.FunctionComponent<ShortcutsWidgetProps> = ({
  available,
  onAddRentClick,
  onTenantAddClick,
  onRentalAddClick,
}) => {
  return (
    <Card
      display="flex"
      flex={1}
      flexDirection="row"
      marginY={majorScale(1)}
      justifyContent="center"
    >
      <Pane
        className={TourType.DASHBOARD_SHORTCUTS}
        alignItems="center"
        justifyContent="center"
        padding={majorScale(2)}
      >
        <Button
          iconBefore={available ? SavedIcon : LockIcon}
          appearance="primary"
          onClick={onAddRentClick}
          disabled={!available}
        >
          {_("establish_receipt")}
        </Button>
        <Button
          iconBefore={PeopleIcon}
          marginLeft={majorScale(2)}
          onClick={onTenantAddClick}
        >
          {_("create_tenant")}
        </Button>
        <Button
          iconBefore={PlusIcon}
          marginLeft={majorScale(2)}
          onClick={onRentalAddClick}
        >
          {_("create_contract")}
        </Button>
      </Pane>
    </Card>
  );
};
