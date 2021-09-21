import {
  Badge,
  BadgeOwnProps,
  Card,
  Heading,
  Icon,
  InfoSignIcon,
  Pane,
  Text,
  Tooltip,
} from "evergreen-ui";
import * as React from "react";
import Skeleton from "react-loading-skeleton";
import { useAppTheme } from "../../hooks/use-app-theme";
import { AvatarItem, AvatarItemProps } from "../avatar-item/avatar-item";
import { EmptyDatasetProps } from "../common";

export type CardSynthesisBadgeItem = {
  value: string;
  color?: BadgeOwnProps["color"];
  handler?: () => void;
};

export type CardSynthesisContentItemProps = {
  // Left title
  title: string;
  // Right text value
  text?: string;
  // Right badges value with handler if necesary
  badges?: CardSynthesisBadgeItem[];
  // Avatars
  avatars?: AvatarItemProps[];
  // Tooltip advise or info at the right of the text
  tooltip?: string;
};

export type CardSynthesisItemProps = {
  loading?: boolean;
  title?: string;
  items?: CardSynthesisContentItemProps[];
  buttons?: React.ReactNode[];
  emptyDataset?: React.ReactElement<EmptyDatasetProps>;
};

export const CardSynthesisItem: React.FunctionComponent<
  CardSynthesisItemProps
> = ({
  loading,
  title,
  items,
  buttons,
  emptyDataset,
}) => {
  const theme = useAppTheme();
  if (loading) {
    return <Skeleton count={3} height={40} />;
  }
  return (
    <Pane display="flex" flexDirection="column">
      <Card
        display="flex"
        flexDirection="column"
        elevation={1}
        backgroundColor="white"
        marginBottom={theme.margin.large}
      >
        {title && <Pane
          display="flex"
          padding={theme.margin.medium}
          paddingX={theme.margin.large}
          borderBottom="muted"
        >
          <Heading size={600}>{title}</Heading>
        </Pane>}
        {/* Bottom panel */}
        <Pane display="flex">
          {emptyDataset
            ? (
              <Pane
                display="flex"
                justifyContent="center"
                flex={1}
                minHeight={200}
              >
                {emptyDataset}
              </Pane>
            )
            : (<Pane
              flex={3}
              paddingBottom={theme.margin.largest}
            >
              {loading && <Pane>
                <Skeleton count={3} height={40} />
              </Pane>}
              {!loading && items.map((tab) => (
                <Pane
                  display="flex"
                  justifyContent="start"
                  marginTop={20}
                  key={tab.title}
                >
                  <Pane
                    display="flex"
                    width={300}
                    justifyContent="flex-end"
                    alignItems={tab.avatars?.length ? "center" : "flex-start"} // While using tab we need to adjust the height of the title
                  >
                    <Text size={400}>{tab.title}</Text>
                  </Pane>
                  <Pane
                    marginLeft={theme.margin.largest}
                    display="flex"
                    flexWrap="wrap"
                    maxWidth={300}
                    alignItems="center"
                  >
                    {/* Badges as single value */}
                    {tab.badges?.map((t, index) => {
                      return <Badge
                        key={index}
                        onClick={() => t?.handler?.()}
                        marginRight={theme.margin.medium}
                        color={t.color}
                        style={{
                          cursor: t?.handler ? "pointer" : "",
                        }}
                      >
                        {t.value}
                      </Badge>;
                    })}
                    {tab.avatars?.map((avatar, index) => {
                      return <AvatarItem {...avatar} key={index} />;
                    })}
                    {/* Text */}
                    {tab.text && <Text>
                      {tab.text}
                    </Text>}
                    {/* Tooltip */}
                    {tab.tooltip &&
                      <Pane
                        display="flex"
                        alignItems="center"
                        marginLeft={theme.margin.small}
                      >
                        <Tooltip
                          content={tab.tooltip}
                        >
                          <Icon
                            icon={InfoSignIcon}
                            color={theme.palette.neutral.base}
                          />
                        </Tooltip>
                      </Pane>}
                  </Pane>
                </Pane>
              ))}
            </Pane>)}
          {/* Right panel */}
          {buttons?.length > 0 && !emptyDataset && <Pane
            display="flex"
            flex={1}
            flexDirection="column"
            justifyContent="flex-start"
            marginRight={theme.margin.large}
            paddingY={theme.margin.medium}
          >
            {buttons?.map((button, index) => (
              button
            ))}
          </Pane>}
        </Pane>
      </Card>
    </Pane>
  );
};
