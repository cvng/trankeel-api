import { HomeIcon } from "evergreen-ui";
import * as React from "react";
import { Lender } from "../../types";
import { translate } from "piteo-kit";
import { EntityList } from "../entity-list/entity-list";

const _ = translate();

export type LenderListProps = {
  /** Lender list */
  lenders?: Lender[];
  /** The selected lender */
  selectedLender?: Lender;
  /** Loading status */
  loading: boolean;
  /** True if the search field is not empty */
  filterEnabled: boolean;
  /** Fired when an item is selected  */
  onLenderSelect?: (lenderId: string) => void;
  /** Fired when clicking "add" button */
  onLenderAdd?: () => void;
  /** Fired when search field change */
  onSearchFieldChange?: (value: string) => void;
};

export const LenderList: React.FunctionComponent<
  LenderListProps
> = ({
  lenders,
  selectedLender,
  loading,
  filterEnabled,
  onLenderSelect,
  onLenderAdd,
  onSearchFieldChange,
}) => {
  return (
    <EntityList
      loading={loading}
      filterEnabled={filterEnabled}
      entities={lenders}
      onSearchFieldChange={onSearchFieldChange}
      onSelectEntity={onLenderSelect}
      onAddNewEntity={onLenderAdd}
      emptyDatasetIcon={HomeIcon}
      entity={_("lender")}
      cardData={(lender: Lender) => {
        return {
          key: lender.id,
          title: lender.displayName,
          subtitle: lender.identity.__typename === "Company"
            ? _("moral_person")
            : _("physical_person"),
          selected: lender.id === selectedLender?.id,
        };
      }}
    />
  );
};
