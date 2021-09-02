import {
  AddIcon,
  Card,
  Heading,
  Icon,
  IconButton,
  minorScale,
  MoreIcon,
  Pane,
  Pill,
  PlayIcon,
  Text,
} from "evergreen-ui";
import React from "react";

export type WorkflowActionProps = {
  name: string;
  hasActionAfter?: boolean;
  value?: string;
};

const cardStyle = { border: "1px solid #EBEDF3" };

const style = {
  display: "flex",
  alignItems: "center",
  padding: minorScale(3),
  flex: 1,
};

export const WorkflowAction: React.FunctionComponent<
  WorkflowActionProps
> = ({
  name,
  hasActionAfter,
  value,
}) => {
  return (
    <Pane
      minHeight={100}
      display="flex"
      flexDirection="column"
      alignItems="center"
    >
      <Pane background={"#EBEDF3"} width={2} minHeight={20}></Pane>
      <Card
        display="flex"
        flexDirection="column"
        minWidth={300}
        style={{
          ...cardStyle,
        }}
      >
        <Pane
          style={{
            ...style,
            justifyContent: "space-between",
          }}
        >
          <Pane display="flex">
            <Icon icon={PlayIcon} color="info" marginRight={minorScale(3)} />
            <Heading size={300}>ACTION</Heading>
          </Pane>
          <Pane>
            <IconButton height={24} icon={MoreIcon} />
          </Pane>
        </Pane>
        <Pane
          style={{
            ...style,
            borderTop: "1px solid #EBEDF3",
          }}
        >
          <Text>{name}</Text>
        </Pane>
        <Pane
          style={{
            ...style,
            borderTop: "1px solid #EBEDF3",
          }}
        >
          <Pill color="green">{value}</Pill>
        </Pane>
      </Card>
      <Pane
        display="flex"
        flexDirection="column"
        alignItems="center"
        width={30}
      >
        <Pane background={"#EBEDF3"} width={2} minHeight={20}></Pane>
        <Icon icon={AddIcon} color="info" size={20} />
        {hasActionAfter &&
          <Pane background={"#EBEDF3"} width={2} minHeight={20}></Pane>}
      </Pane>
    </Pane>
  );
};
