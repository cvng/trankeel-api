import { Pane } from "evergreen-ui";
import * as React from "react";
import { Tenant } from "../../types";
import { TenantSynthesisGuarantees } from "./tenant-synthesis-guarantees";
import {
  TenantSynthesisInformations,
  TenantSynthesisInformationsProps,
} from "./tenant-synthesis-informations";
import {
  TenantSynthesisSummary,
  TenantSynthesisSummaryProps,
} from "./tenant-synthesis-summary";

export type TenantSynthesisDefaultProps =
  & {
    loading?: boolean;
    tenant?: Tenant;
  }
  & TenantSynthesisSummaryProps
  & TenantSynthesisInformationsProps;

export const TenantSynthesisDefault: React.FunctionComponent<
  TenantSynthesisDefaultProps
> = ({
  loading,
  tenant,
  onSelectLease,
  onLeaseAddFromTenant,
  onPropertySelect,
}) => {
  return (
    <Pane>
      {/* Summary */}
      <TenantSynthesisSummary
        loading={loading}
        tenant={tenant}
        onSelectLease={onSelectLease}
        onLeaseAddFromTenant={onLeaseAddFromTenant}
        onPropertySelect={onPropertySelect}
      />

      {/* Informations */}
      <TenantSynthesisInformations
        loading={loading}
        tenant={tenant}
      />

      {/* Contract guarantees */}
      <TenantSynthesisGuarantees loading={loading} tenant={tenant} />
    </Pane>
  );
};
