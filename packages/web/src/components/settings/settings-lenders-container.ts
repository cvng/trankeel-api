import { useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { translate } from "piteo-kit";
import { FunctionComponent, useState } from "react";
import { useHistory } from "react-router-dom";
import { RouteById, Routes } from "../../constants";
import { LenderQuery } from "../../helpers";
import { useSearch } from "../../hooks/use-search";
import { useParamId } from "../../hooks/use-param-id";
import { Lender } from "../../types";
import { SettingsLendersProps } from "./settings-lenders";

const _ = translate();

export const withContainer = (Component): FunctionComponent<unknown> =>
  () => {
    const history = useHistory();
    const [filteredText, setFilteredText] = useState(null);

    // Get the entity id in the url
    const lenderId = useParamId(Routes.SETTINGS_LENDER_VIEW);
    let { loading, data: { lenders } = { lenders: [] } } = useQuery(
      LenderQuery,
    );

    // Select the first entity by default or get the selected in the list by its id
    const selectedLender =
      lenders?.find((lender: Lender) => lender.id === lenderId) ||
      lenders?.[0];

    // Search
    const filterEnabled = !loading && filteredText?.length > 0;
    if (filterEnabled) {
      lenders = useSearch(
        (item: Lender) => item.displayName,
        lenders,
        filteredText,
      );
    }

    const onLenderSelect = (lenderId: string): void => {
      history.push(RouteById(Routes.SETTINGS_LENDER_VIEW, [lenderId]));
    };

    const onLenderAdd = (): void => {
      // history.push(Routes.SETTINGS_LENDERS_ADD); TODO: Soon
      toaster.notify(_("feature_available_soon"));
    };

    const onLenderEdit = (lenderId: string): void => {
      history.push(RouteById(Routes.SETTINGS_LENDER_EDIT, [lenderId]));
    };

    const onLenderDelete = (): void => {
      toaster.notify(_("feature_available_soon"));
    };

    const onSearchFieldChange = (value: string): void => {
      setFilteredText(value);
    };

    const componentProps: SettingsLendersProps = {
      loading,
      selectedLender,
      lenderHeaderProps: {
        lender: selectedLender,
        loading,
        onLenderEdit,
        onLenderDelete,
      },
      lenderListProps: {
        lenders,
        selectedLender,
        loading,
        filterEnabled,
        onLenderSelect,
        onLenderAdd,
        onSearchFieldChange,
      },
    };

    return Component(componentProps);
  };
