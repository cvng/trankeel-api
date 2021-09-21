import {
  Avatar,
  Card,
  FlagIcon,
  Heading,
  Icon,
  NotificationsUpdatedIcon,
  Pane,
  Small,
  Text,
} from "evergreen-ui";
import React, { ElementType } from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { EventType, Person } from "../../types";

export type EventModel = {
  type: EventType;
  content: string;
  createdAt: string;
  flag?: {
    title: string;
    icon?: ElementType;
  };
  user?: Person;
};

export type RecentActivityItemProps = {
  event: EventModel;
};

export const RecentActivityItem: React.FunctionComponent<
  RecentActivityItemProps
> = ({
  event,
}) => {
  const theme = useAppTheme();
  return (
    <Card display="flex" padding={theme.margin.medium}>
      {/* Avatar */}
      <Pane
        display="flex"
        flexDirection="column"
        alignItems="center"
        padding={theme.margin.medium}
        marginTop={theme.margin.small}
      >
        {event?.flag?.title?.length > 0
          ? (
            <Pane
              display="flex"
              background="#F2B241"
              width={30}
              height={30}
              borderRadius={15}
              alignItems="center"
              justifyContent="center"
            >
              <Icon icon={event?.flag?.icon || NotificationsUpdatedIcon} />
            </Pane>
          )
          : (
            <Avatar isSolid size={30} name={event?.user?.displayName} />
          )}
      </Pane>

      {/* Content */}
      <Pane padding={theme.margin.medium}>
        {event?.flag?.title && <Pane
          display="flex"
          alignItems="flex-start"
          marginY={theme.margin.small}
        >
          <Icon icon={FlagIcon} color="#58AD38" />
          <Heading paddingLeft={theme.margin.medium} size={300} color="#58AD38">
            {event?.flag?.title}
          </Heading>
        </Pane>}
        <Pane>
          {}
          <Text size={400} color="muted">
            {event.content}
          </Text>
        </Pane>
        <Pane marginTop={theme.margin.small}>
          <Text>
            <Small opacity={0.6}>{event.createdAt}</Small>
          </Text>
        </Pane>
      </Pane>
    </Card>
  );
};
