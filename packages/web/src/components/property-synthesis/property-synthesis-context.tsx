import { useQuery } from "@apollo/client";
import React, { createContext, useEffect, useState } from "react";
import { Routes } from "../../constants";
import { PropertyListQuery } from "../../helpers";
import { useParamId } from "../../hooks/use-param-id";
import { Property } from "../../types";

export const PropertySynthesisContext = createContext(null);

export interface PropertySynthesisContextProps {
  properties: Property[];
  selectedProperty?: Property;
  loading: boolean;
}

export const PropertySynthesisProvider = ({ children }) => {
  // Get the ids in the url
  const propertyId = useParamId(Routes.PROPERTY_VIEW);

  const [selectedProperty, setSelectedProperty] = useState(null);

  // Fetch the properties
  const { loading, data: { properties } = { properties: [] } } = useQuery(
    PropertyListQuery,
  );

  // Select the first property by default or get the selected in the list by its id
  useEffect(() => {
    setSelectedProperty(
      properties?.find((property: Property) => property.id === propertyId) ||
        properties?.[0],
    );
  }, [propertyId, properties]);

  return (
    <PropertySynthesisContext.Provider
      value={{ properties, loading, selectedProperty }}
    >
      {children}
    </PropertySynthesisContext.Provider>
  );
};
