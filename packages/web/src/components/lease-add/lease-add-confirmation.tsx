import { Button, Card, Heading, Pane, Text } from "evergreen-ui";
import React, { useContext, useEffect, useState } from "react";
import errorAnimationData from "../../assets/lotties/59953-error.json";
import animationData from "../../assets/lotties/woman-flowers.json";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Closure } from "../../utils";
import { LottieAnimation } from "../common/lottie-animation";
import { Loading } from "../loading/loading";
import { LeaseAddContext, LeaseAddContextAction } from "./lease-add-context";
import { translate } from "piteo-kit";

const _ = translate();

export type LeaseAddConfirmationProps = {
  error: boolean;
  loading?: boolean;
  title?: string;
  timerDidEnd?: Closure;
};

export const LeaseAddConfirmation: React.FunctionComponent<
  LeaseAddConfirmationProps
> = ({
  error,
  loading = false,
  title,
  timerDidEnd,
}) => {
  const theme = useAppTheme();
  const { finishFlowWithError, dispatch } = useContext(
    LeaseAddContext,
  );

  const [counter, setCounter] = useState(5);

  useEffect(() => {
    // While the flow is still running prevent to launch the timer
    if (finishFlowWithError === undefined) {
      return;
    }
    if (counter === 0) {
      dispatch({ type: LeaseAddContextAction.Reset });
      timerDidEnd?.();
    }
    counter > 0 && setTimeout(() => setCounter(counter - 1), 1000);
  }, [counter, finishFlowWithError, dispatch, timerDidEnd]);

  const loadingState = () => {
    return <Loading
      title={_("lease_contract_loading_title")}
      message={_("lease_contract_loading_message")}
    />;
  };

  const normalState = (
    title: string,
    message: string,
    animation: unknown,
    animationSize: number,
  ) => {
    return <Pane
      display="flex"
      flexDirection="column"
      alignItems="center"
      justifyContent="center"
      minHeight={500}
    >
      <LottieAnimation
        data={animation}
        marginY={20}
        height={animationSize || 200}
      />
      <Heading marginBottom={theme.margin.medium}>{title}</Heading>
      <Text size={300}>{message}</Text>
      <Button marginY={theme.margin.medium} appearance={"primary"}>
        {_("show_lease")}
      </Button>
    </Pane>;
  };

  if (loading) {
    return loadingState();
  }

  return (
    <Card>
      {error
        ? (
          normalState(
            _("lease_contract_error"),
            _("please_retry_later"),
            errorAnimationData,
            80,
          )
        )
        : (
          normalState(
            title || _("lease_contract_success"),
            _("lease_contract_redirection_message", { counter }),
            animationData,
            null,
          )
        )}
    </Card>
  );
};
