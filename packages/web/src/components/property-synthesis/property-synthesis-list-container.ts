import React, { useContext, useState } from "react";
import { PropertySynthesisContext } from ".";
import { useRouter } from "../../hooks/use-router";
import { useSearch } from "../../hooks/use-search";
import { Property } from "../../types";
import { PropertySynthesisContextProps } from "./property-synthesis-context";
import { PropertySynthesisListProps } from "./property-synthesis-list";

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const [filteredText, setFilteredText] = useState(null);

    const context = useContext(
      PropertySynthesisContext,
    ) as PropertySynthesisContextProps;
    let { loading, selectedProperty, properties } = context;

    // Search
    const filterEnabled = !context?.loading && filteredText?.length > 0;
    properties = useSearch(
      (item: Property) => {
        return item.name;
      },
      properties,
      filteredText,
    );

    const onPropertySelect = (propertyId: string): void => {
      router.showPropertySynthesisRoute(propertyId);
    };

    const onPropertyAdd = (): void => {
      router.showPropertyAdd();
    };

    const onSearchFieldChange = (value: string): void => {
      setFilteredText(value);
    };

    const componentProps: PropertySynthesisListProps = {
      properties,
      selectedProperty,
      loading,
      filterEnabled,
      onPropertySelect,
      onPropertyAdd,
      onSearchFieldChange,
    };

    return WrappedComponent(componentProps);
  };
