import { useQuery } from "@apollo/client";
import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { TenantListQuery } from "../../helpers";
import { useSearch } from "../../hooks/use-search";
import { File, Tenant } from "../../types";
import { TenantDocumentListProps } from "./tenant-document-list";

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const { id: tenantId } = useParams();
    const [filteredText, setFilteredText] = useState(null);
    const [tenant, setTenant] = useState(null);

    const { loading, data: { tenants } = { tenants: [] } } = useQuery(
      TenantListQuery,
    );

    useEffect(() => {
      setTenant(tenants?.find((item: Tenant) => item.id === tenantId));
    }, [tenants, tenantId]);

    let files = tenant?.files;

    // Search
    const filterEnabled = !loading && filteredText?.length > 0;
    if (filterEnabled) {
      files = useSearch(
        (file: File) => {
          return file.filename;
        },
        files,
        filteredText,
      );
    }

    const onSearchFieldChange = (value: string): void => {
      setFilteredText(value);
    };

    const componentProps: TenantDocumentListProps = {
      loading,
      filterEnabled,
      files,
      onSearchFieldChange,
    };

    return WrappedComponent(componentProps);
  };
