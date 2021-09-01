import { useQuery } from "@apollo/client";
import { FunctionComponent, useMemo, useState } from "react";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import { useInterval } from "../common";
import { ImportStatusCornerDialogProps } from "./import-status-corner-dialog";

const DELAY = 300; // in ms

const RunningTaskQuery = null;

export type WrappedComponentProps = unknown;

export const withContainer = (
  Component,
): FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();

    const [isShown, setIsShown] = useState(true);
    const [progress, setProgress] = useState(1);

    const { data: { status } = { status: null } } = useQuery(RunningTaskQuery);

    const handleVisibilityChange = (): void => {
      setIsShown(status);
    };

    const handleIntervalChange = (): void => {
      setProgress(Math.min(progress + 10, 100));
    };

    const handleConfirm = (): void => {
      history.push(Routes.PROPERTY_VIEW);
    };

    useMemo(handleVisibilityChange, [status]);

    useInterval(handleIntervalChange, status && progress < 100 ? DELAY : null);

    const props: ImportStatusCornerDialogProps = {
      progress,
      isShown,
      onConfirm: handleConfirm,
    };

    return Component(props);
  };
