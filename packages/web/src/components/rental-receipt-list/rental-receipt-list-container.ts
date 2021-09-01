import React from "react";
import { RentalReceiptListProps } from "./rental-receipt-list";

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    // const { tenantId } = useParams();

    // TODO: Send the query
    // const result = useQuery(TenantWithRentalReceiptsQuery, {
    //   variables: { id: tenantId }
    // });

    const componentProps: RentalReceiptListProps = {
      tenant: null, // TODO: pass the tenant
    };

    return WrappedComponent(componentProps);
  };
