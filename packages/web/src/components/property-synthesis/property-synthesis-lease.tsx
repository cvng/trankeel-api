// @ts-nocheck
import {
  Button,
  DocumentOpenIcon,
  EraserIcon,
  Pane,
  PlusIcon,
} from "evergreen-ui";
import { toLocaleDateString, translate } from "piteo-kit";
import * as React from "react";
import Skeleton from "react-loading-skeleton";
import animationData from "../../assets/lotties/51084-contract-signing.json";
import { ContractHelper, NumberHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Lease, Property } from "../../types";
import { UniquableClosure } from "../../utils";
import { AvatarItemProps } from "../avatar-item/avatar-item";
import { CardSynthesisItem } from "../cards";
import { EmptyDataset } from "../common";
import { LottieAnimation } from "../common/lottie-animation";

const MAX_WIDTH = 240;

export type PropertySynthesisLeaseProps = {
  loading: boolean;
  property: Property;
  onTenantSelect?: UniquableClosure;
  onLeaseAdd?: UniquableClosure;
  onDeleteLease?: (propertyId: string, leaseId: string) => void;
};

const _ = translate();

export const PropertySynthesisLease: React.FunctionComponent<
  PropertySynthesisLeaseProps
> = ({
  loading,
  property,
  onTenantSelect,
  onLeaseAdd,
  onDeleteLease,
}) => {
  const theme = useAppTheme();

  if (loading) {
    return <Skeleton height={20} count={4} />;
  }

  const getAvatarItems = (lease: Lease): AvatarItemProps[] => {
    const badgesTenantsItems = [];
    lease?.tenants?.map((t) => {
      return (badgesTenantsItems.push({
        name: t.fullName,
        handler: () => onTenantSelect?.(t.id),
      }));
    });
    return badgesTenantsItems;
  };

  return (
    <Pane>
      {property?.leases?.length === 0 &&
        <CardSynthesisItem
          emptyDataset={<EmptyDataset
            marginY={theme.margin.largest}
            title={_("empty_lease_title")}
            subtitle={_("empty_lease_subtitle")}
            animation={<LottieAnimation data={animationData} />}
            button={<Button
              iconBefore={PlusIcon}
              appearance="primary"
              onClick={() => onLeaseAdd?.(property?.id)}
            >
              {_("lease_new")}
            </Button>}
          />}
        />}
      {property?.leases?.map((lease) => {
        return (<CardSynthesisItem
          title={_("informations")}
          items={[
            {
              title: _("status"),
              text: ContractHelper.statusMap().get(lease?.status),
            },
            {
              title: _("type"),
              text: ContractHelper.typeMap().get(lease?.type) || "-",
            },
            {
              title: _("effect_date"),
              text: toLocaleDateString(lease?.effectDate, "-"),
              tooltip: lease?.signatureDate
                ? _("signature_date_hint", {
                  date: toLocaleDateString(lease?.signatureDate),
                })
                : null,
            },
            {
              title: _("contract_end_date"),
              text: toLocaleDateString(lease?.renewDate, "-"),
            },
            {
              title: _("rent_full_amount"),
              text: NumberHelper.formatToString(
                lease?.rentFullAmount,
                false,
              ) || "-",
              tooltip: lease?.rentFullAmount > 0
                ? _("rent_charges_hint", {
                  amount: NumberHelper.formatToString(
                    lease?.rentChargesAmount,
                    false,
                  ),
                })
                : null,
            },
            {
              title: _("deposit_amount"),
              text: NumberHelper.formatToString(lease?.depositAmount, false) ||
                "-",
            },
            {
              title: _("rent_recuperation_mode"),
              text: ContractHelper.chargesRecuperationMode().get(
                lease?.data?.chargesRecuperationMode,
              ) || "-",
            },
            {
              title: _("ongoing_proceedings"),
              text: _("no"),
            },
            {
              title: lease?.tenants?.length > 1 ? _("tenants") : _("tenant"),
              avatars: getAvatarItems(lease),
            },
          ]}
          buttons={[
            <Button
              iconBefore={DocumentOpenIcon}
              maxWidth={MAX_WIDTH}
              marginY={theme.margin.medium}
              onClick={() => {}}
              disabled
            >
              {_("show_lease_pdf")}
            </Button>,
            <Button
              intent="danger"
              iconBefore={EraserIcon}
              maxWidth={MAX_WIDTH}
              marginY={theme.margin.medium}
              onClick={() => onDeleteLease?.(property?.id, lease?.id)}
            >
              {_("delete_lease")}
            </Button>,
          ]}
          loading={loading}
        />);
      })}
    </Pane>
  );
};
