import {
  Card,
  defaultTheme,
  Heading,
  Icon,
  majorScale,
  minorScale,
  Pane,
  PaneProps,
  Text,
} from "evergreen-ui";
import * as React from "react";

export type EmptyDatasetProps = {
  title: string;
  subtitle: string;
  icon?: React.ElementType;
  animation?: React.ReactNode; // cf. LottieAnimation
  button?: React.ReactNode;
  removeBorder?: boolean;
} & PaneProps;

export const EmptyDataset: React.FunctionComponent<EmptyDatasetProps> = ({
  title,
  subtitle,
  icon,
  animation,
  button,
  removeBorder = false,
  ...props
}) => (
  <Pane
    display="flex"
    flexDirection="column"
    justifyContent="center"
    alignItems="center"
    {...props}
  >
    <Card
      elevation={removeBorder ? 0 : 2}
      width={350}
      height={350}
      padding={minorScale(20)}
      display="flex"
      flexDirection="column"
      alignItems="center"
      justifyContent="center"
      boxShadow={(removeBorder && "none !important") || ""}
    >
      {icon && (
        <Pane
          display={"flex"}
          justifyContent={"center"}
          borderRadius={"50%"}
          background={defaultTheme.palette.green.lightest}
          alignItems="center"
        >
          <Icon
            margin={20}
            icon={icon}
            color={defaultTheme.palette.green.base}
          />
        </Pane>
      )}
      {!!animation && <Pane>{animation}</Pane>}
      <Pane
        display="flex"
        flexDirection="column"
        alignItems="center"
        justifyContent="center"
      >
        <Heading size={400} textAlign="center" marginTop={majorScale(3)}>
          {title}
        </Heading>
        <Text size={300} textAlign="center" marginTop={majorScale(1)}>
          {subtitle}
        </Text>
        <Pane marginTop={majorScale(2)}>{button}</Pane>
      </Pane>
    </Card>
  </Pane>
);
