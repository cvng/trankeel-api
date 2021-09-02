import {
  Card,
  CaretDownIcon,
  Heading,
  IconButton,
  minorScale,
  Pane,
} from "evergreen-ui";
import moment from "moment";
import * as React from "react";
import { translate } from "piteo-kit";

const _ = translate();

const STARTED_DATE_FORMAT = "DD";
const ENDED_DATE_FORMAT = "DD MMMM YYYY";

export type PeriodSelectorProps = {
  startedDate: Date;
  endedDate: Date;
  isEditable?: boolean;
  onSelect?: () => void;
};

export const PeriodSelector: React.FunctionComponent<PeriodSelectorProps> = ({
  startedDate,
  endedDate,
  isEditable = false,
  onSelect,
  ...props
}) => {
  return (
    <Card
      display="flex"
      flexDirection="row"
      border="muted"
      alignItems="center"
      background="white"
      elevation={1}
      height={35}
      {...props}
    >
      <Pane flex={1} justifyContent="center">
        <Heading size={200} marginX={minorScale(2)}>
          {_("date_period", {
            periodStart: moment(startedDate).format(STARTED_DATE_FORMAT),
            periodEnd: moment(endedDate).format(ENDED_DATE_FORMAT),
          })}
        </Heading>
      </Pane>
      {isEditable && (
        <IconButton
          appearance="minimal"
          icon={CaretDownIcon}
          onClick={() => onSelect && onSelect()}
        />
      )}
    </Card>
  );
};
