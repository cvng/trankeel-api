import {
  Card,
  Heading,
  NotificationsIcon,
  Pane,
  PaneProps,
} from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { useAppTheme } from "../../hooks/use-app-theme";
import { EmptyDataset } from "../common";
import { EventModel, RecentActivityItem } from "./recent-activity-item";

const _ = translate();

export type RecentActivityListrops = {
  loading: boolean;
  events: EventModel[];
} & PaneProps;

export const RecentActivityList: React.FunctionComponent<
  RecentActivityListrops
> = ({
  loading,
  events,
  ...props
}) => {
  const theme = useAppTheme();
  const loadingState = () => {
    return <Pane padding={theme.margin.medium}>
      <Skeleton count={3} height={40} />
    </Pane>;
  };
  const emptyState = () => {
    return <Pane display="flex" justifyContent="center">
      <EmptyDataset
        title={_("no_recent_activities_title")}
        subtitle={_("no_recent_activities_subtitle")}
        icon={NotificationsIcon}
      />
    </Pane>;
  };
  return (
    <Card
      width={300}
      minHeight={300}
      maxHeight={600}
      elevation={1}
      background="white"
      marginLeft={theme.margin.large}
      {...props}
      overflowY={"scroll"}
    >
      <Pane
        alignItems="center"
        borderBottom="muted"
        paddingX={theme.margin.large}
        paddingY={theme.margin.medium}
      >
        <Heading>
          {_("recent_activities")}
        </Heading>
      </Pane>
      {/* Loading state */}
      {loading && loadingState()}
      {/* Empty state */}
      {events?.length === 0 && emptyState()}
      {/* Default state */}
      {events?.map((event, index) => {
        return <RecentActivityItem key={index} event={event} />;
      })}
    </Card>
  );
};
