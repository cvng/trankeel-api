import { Avatar, Card, Heading, Pane } from "evergreen-ui";
import React, { useState } from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Closure } from "../../utils";

const DEFAULT_SIZE = 25;

export type AvatarItemProps = {
  name: string;
  photoURL?: string;
  size?: number;
  handler?: Closure;
};

export const AvatarItem: React.FunctionComponent<AvatarItemProps> = ({
  name,
  photoURL,
  size,
  handler = null,
}) => {
  const [hover, setHover] = useState(false);
  const isSelectable = handler !== null;
  const toggleHover = () => {
    if (!isSelectable) {
      return;
    }
    setHover(!hover);
  };
  const style = (selected: boolean) => {
    return selected && isSelectable
      ? { border: "1px solid " + theme.borderSelectedColor, cursor: "pointer" }
      : { border: "1px solid " + theme.palette.neutral.light, cursor: null };
  };
  const theme = useAppTheme();
  return (
    <Pane display="inline-block">
      <Card
        display="flex"
        background={"white"}
        borderRadius={DEFAULT_SIZE}
        alignItems="center"
        padding={theme.margin.medium}
        style={{
          ...style(hover),
        }}
        onMouseEnter={toggleHover}
        onMouseLeave={toggleHover}
        onClick={() => handler?.()}
      >
        <Avatar
          isSolid
          name={name}
          size={size || DEFAULT_SIZE}
          src={photoURL}
          marginRight={theme.margin.medium}
        />
        <Heading size={200}>{name}</Heading>
      </Card>
    </Pane>
  );
};
