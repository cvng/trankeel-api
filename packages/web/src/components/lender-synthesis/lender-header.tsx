import { EditIcon, EraserIcon } from "evergreen-ui";
import { translate } from "piteo-kit";
import * as React from "react";
import { Lender } from "../../types";
import { EntityHeader } from "../entity-header/entity-header";

const _ = translate();

export type LenderHeaderProps = {
  loading?: boolean;
  lender?: Lender;
  onLenderEdit?(lenderId: string): void;
  onLenderDelete?(lenderId: string): void;
};

export const LenderHeader: React.FunctionComponent<
  LenderHeaderProps
> = ({
  loading,
  lender,
  onLenderEdit,
  onLenderDelete,
}) => {
  return (
    <EntityHeader
      loading={false}
      cardItemProps={{
        title: lender?.displayName,
        hasAvatar: !!lender,
        loading: loading,
        item: lender,
        elevation: null,
        backgroundColor: null,
        style: null, // Remove the cursor style
      }}
      popinItemsProps={[
        {
          bottomDivider: true,
          subItems: [
            {
              name: `${_("edit")}...`,
              handler: () => onLenderEdit?.(lender?.id),
              icon: EditIcon,
            },
            {
              name: `${_("delete")}...`,
              handler: () => onLenderDelete?.(lender?.id),
              icon: EraserIcon,
            },
          ],
        },
      ]}
    />
  );
};
