import { Card, Heading, majorScale, Pane, PaneProps } from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { EmptyItem } from "../common";

const _ = translate();

export type HeritageWidgetProps = {
  propertyCount?: number;
  tenantCount?: number;
  contractCount?: number;
} & PaneProps;

export const HeritageWidget: React.FunctionComponent<HeritageWidgetProps> = ({
  propertyCount,
  tenantCount,
  contractCount,
  ...props
}) => {
  return (
    <Card
      display="flex"
      flexDirection="column"
      elevation={1}
      minHeight={100}
      background="white"
      {...props}
    >
      <Pane display="flex" flex={1} flexDirection="row">
        <Pane
          display="flex"
          flex={1}
          flexDirection="column"
          borderRight="muted"
          justifyContent="center"
          alignItems="center"
          padding={majorScale(2)}
        >
          {propertyCount > 0
            ? (
              <Heading size={800}>{propertyCount}</Heading>
            )
            : (
              <EmptyItem />
            )}
          <Heading size={100}>
            {propertyCount === 1 ? _("property") : _("properties")}
          </Heading>
        </Pane>
        <Pane
          display="flex"
          flex={1}
          flexDirection="column"
          borderRight="muted"
          justifyContent="center"
          alignItems="center"
          padding={majorScale(2)}
        >
          {tenantCount > 0
            ? (
              <Heading size={800}>{tenantCount}</Heading>
            )
            : (
              <EmptyItem />
            )}
          <Heading size={100}>
            {tenantCount === 1 ? _("tenant") : _("tenants")}
          </Heading>
        </Pane>
        <Pane
          display="flex"
          flex={1}
          flexDirection="column"
          borderRight="muted"
          justifyContent="center"
          alignItems="center"
          padding={majorScale(2)}
        >
          {contractCount > 0
            ? (
              <Heading size={800}>{contractCount}</Heading>
            )
            : (
              <EmptyItem />
            )}
          <Heading size={100}>
            {contractCount === 1 ? _("contract") : _("contracts")}
          </Heading>
        </Pane>
      </Pane>
    </Card>
  );
};
