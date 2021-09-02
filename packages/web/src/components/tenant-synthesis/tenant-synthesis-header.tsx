import { EditIcon, EraserIcon, PaneProps } from "evergreen-ui";
import * as React from "react";
import { Tenant } from "../../types";
import { translate } from "piteo-kit";
import { EntityHeader } from "../entity-header/entity-header";

const _ = translate();

export type TenantSynthesisHeaderProps = {
  loading?: boolean;
  tenant?: Tenant;
  onTenantEdit?(tenantId: string): void;
  onTenantDelete?(tenantId: string): void;
} & PaneProps;

export const TenantSynthesisHeader: React.FunctionComponent<
  TenantSynthesisHeaderProps
> = ({
  loading,
  tenant,
  onTenantEdit,
  onTenantDelete,
}) => {
  if (!tenant) return;
  return (
    <EntityHeader
      loading={false}
      cardItemProps={{
        title: tenant?.fullName,
        subtitle: tenant?.propertyName?.toUpperCase(),
        hasAvatar: !!tenant,
        loading: loading,
        item: tenant,
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
              handler: () => onTenantEdit?.(tenant?.id),
              icon: EditIcon,
            },
            {
              name: `${_("delete")}...`,
              handler: () => onTenantDelete?.(tenant?.id),
              icon: EraserIcon,
            },
          ],
        },
      ]}
    />
  );
};
