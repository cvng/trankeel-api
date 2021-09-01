import { Card, minorScale } from "evergreen-ui";
import React from "react";

type CardContentProps = React.PropsWithChildren<unknown>;

export const CardContent = (props: CardContentProps) => {
  return (
    <React.Fragment>
      <Card
        elevation={1}
        padding={minorScale(10)}
        flexDirection="column"
        background="white"
        {...props}
      >
        {props.children}
      </Card>
    </React.Fragment>
  );
};
