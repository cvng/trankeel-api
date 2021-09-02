import { Pane } from "evergreen-ui";
import * as React from "react";
import Skeleton from "react-loading-skeleton";
import { LenderHelper } from "../../helpers";
import { Lender } from "../../types";
import { translate } from "piteo-kit";
import { CardSynthesisItem } from "../cards";
import { CardSynthesisContentItemProps } from "../cards/card-synthesis-item";

const _ = translate();

export type LenderDetailsProps = {
  /** Lender list */
  lender?: Lender;
  /** Loading */
  loading: boolean;
};

export const LenderDetails: React.FunctionComponent<
  LenderDetailsProps
> = ({
  lender,
  loading,
}) => {
  if (loading) {
    return <Skeleton count={5} height={40} />;
  }

  if (!lender) {
    return null;
  }

  let items: CardSynthesisContentItemProps[] = [];
  // Add the legal entity type only for moral person
  if (lender?.identity?.__typename === "Company") {
    items.push(
      {
        title: _("legal_entity_identifier"),
        text: lender?.identity?.legalEntityIdentifier || "-",
      },
    );
  }
  items = [...items, {
    title: _("type"),
    text: lender?.identity?.__typename === "Company"
      ? LenderHelper.legalEntityTypes().get(
        lender?.identity.legalEntityType,
      ) || "-"
      : _("physical_person"),
  }, {
    title: _("email"),
    text: lender?.identity?.email || "-",
  }, /* Phone number is not editable in mutation {
    title: _("phone_number"),
    text: lender?.identity?.phoneNumber || "-",
  }*/ {
    title: _("address"),
    text: lender?.identity?.address?.inline || "-",
  }];

  return (
    <Pane>
      <CardSynthesisItem
        title={_("informations")}
        items={items}
        buttons={null}
      />
    </Pane>
  );
};
