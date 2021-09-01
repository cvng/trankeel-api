import { Heading, Pane } from "evergreen-ui";
import React from "react";
import Lottie from "react-lottie";
import { RouteComponentProps } from "react-router-dom";
import animationData from "../../assets/lotties/13805-working-people.json";
import { translate } from "piteo-kit";

const _ = translate();

export class ComingSoon extends React.Component<RouteComponentProps> {
  render = () => {
    const defaultOptions = {
      loop: true,
      autoplay: true,
      animationData: animationData,
      rendererSettings: {
        preserveAspectRatio: "xMidYMid meet",
      },
    };
    return (
      <Pane
        display="flex"
        flex={1}
        flexDirection="column"
        alignItems="center"
        justifyContent="center"
      >
        <Lottie options={defaultOptions} height={400} width={400} />
        <Heading size={400} marginTop={30} textAlign="center">
          {_("feature_available_soon")}
        </Heading>
      </Pane>
    );
  };
}
