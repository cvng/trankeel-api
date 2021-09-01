import { Pane, PaneProps } from "evergreen-ui";
import * as React from "react";
import Lottie from "react-lottie";

export type LottieAnimationProps = {
  data: unknown;
  loop?: boolean;
} & PaneProps;

export const LottieAnimation: React.FunctionComponent<LottieAnimationProps> = (
  props,
) => {
  const defaultOptions = {
    loop: props.loop === false ? false : true,
    autoplay: true,
    animationData: props.data,
    rendererSettings: {
      preserveAspectRatio: "xMidYMid meet",
    },
  };
  return (
    <Pane {...props}>
      <Lottie options={defaultOptions} />
    </Pane>
  );
};
