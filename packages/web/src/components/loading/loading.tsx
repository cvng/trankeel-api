import { Heading, Pane, Paragraph } from "evergreen-ui";
import React from "react";
import animationData from "../../assets/lotties/loading.json";
import { LottieAnimation } from "../common/lottie-animation";

export type LoadingProps = {
  title?: string;
  message?: string;
  height?: number;
};

export const Loading: React.FunctionComponent<LoadingProps> = ({
  title,
  message,
  height,
}) => {
  return (
    <Pane display="flex" flexDirection="column" alignItems="center">
      <LottieAnimation
        data={animationData}
        maxHeight={200}
        marginY={20}
        height={height}
      />
      {title && <Heading>{title}</Heading>}
      {message && <Paragraph>{message}</Paragraph>}
    </Pane>
  );
};
