import { ListItem, TickIcon, UnorderedList } from "evergreen-ui";
import * as React from "react";
import { Feature } from "../../types";

export type BillingPlanFeatureListProps = {
  featureList?: Feature[];
};

export const BillingPlanFeatureList: React.FunctionComponent<
  BillingPlanFeatureListProps
> = ({
  featureList,
}) => (
  <UnorderedList icon={TickIcon} iconColor="success">
    {featureList?.map((obj) => {
      return (
        <ListItem
          key={obj.title}
          // TODO: We are not handling the unavailable features yet
          // TODO: iconColor={!!obj?.available || true ? "success" : "danger"}
          iconColor={"success"}
          // TODO: icon={!!obj?.available || true ? TickIcon : CrossIcon}
          icon={TickIcon}
        >
          {obj.title}
        </ListItem>
      );
    })}
  </UnorderedList>
);
