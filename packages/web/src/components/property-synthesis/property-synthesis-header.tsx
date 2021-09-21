import { AddIcon, EditIcon, TrashIcon } from "evergreen-ui";
import { translate } from "piteo-kit";
import * as React from "react";
import { Property } from "../../types";
import { EntityHeader } from "../entity-header/entity-header";

const _ = translate();

export type PropertySynthesisHeaderProps = {
  loading?: boolean;
  property?: Property;
  onPropertyEdit?: (propertyId: string) => void;
  onPropertyDelete?: (propertyId: string) => void;
  onLeaseAdd?: (propertyId: string) => void;
};

export const PropertySynthesisHeader: React.FunctionComponent<
  PropertySynthesisHeaderProps
> = ({
  loading,
  property,
  onPropertyEdit,
  onPropertyDelete,
  onLeaseAdd,
}) => {
  if (!property) return;
  return (
    <EntityHeader
      loading={false}
      cardItemProps={{
        key: property?.id,
        title: property?.name?.toLocaleUpperCase(),
        subtitle: property?.address
          ? `${property?.address
            ?.line1} ${(property?.address?.line2 ||
              "")}, ${property?.address
            ?.postalCode} ${property?.address?.city}`
          : "",
        hasAvatar: !!property,
        loading: loading,
        item: property,
        elevation: null,
        backgroundColor: null,
        style: null, // Remove the cursor style
      }}
      popinItemsProps={[
        {
          bottomDivider: true,
          subItems: [
            {
              name: `${_("lease_new")}...`,
              handler: () => onLeaseAdd?.(property?.id),
              icon: AddIcon,
            },
          ],
        },
        {
          bottomDivider: true,
          subItems: [
            {
              name: `${_("edit")}...`,
              handler: () => onPropertyEdit?.(property?.id),
              icon: EditIcon,
            },
          ],
        },
        {
          subItems: [
            {
              name: `${_("delete")}...`,
              handler: () => onPropertyDelete?.(property?.id),
              icon: TrashIcon,
            },
          ],
        },
      ]}
    />
  );
};
