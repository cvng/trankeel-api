import { AutomaticUpdatesIcon, Button, Pane, Paragraph } from "evergreen-ui";
import React from "react";
import Lottie from "react-lottie";
import { RouteComponentProps } from "react-router-dom";
import animationData from "../../assets/lotties/13805-working-people.json";
import { Routes } from "../../constants/routes-constants";
import { translate } from "piteo-kit";

const _ = translate();

export class MaintenancePage extends React.Component<RouteComponentProps> {
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
        height={window.innerHeight}
        display="flex"
        flexDirection="column"
        alignItems="center"
        justifyContent="center"
      >
        <Lottie options={defaultOptions} height={200} width={200} />
        <Paragraph width={300} size={400} marginTop={30} textAlign="center">
          {_("maintenance_message")}
        </Paragraph>

        <Button
          marginTop={20}
          iconBefore={AutomaticUpdatesIcon}
          onClick={() => this.props.history.push(Routes.LOGIN)}
        >
          {_("tryAgain")}
        </Button>
      </Pane>
    );
  };
}
