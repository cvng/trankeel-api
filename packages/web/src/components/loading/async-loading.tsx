import {
  Badge,
  CornerDialog,
  EndorsedIcon,
  Heading,
  Icon,
  Pane,
  Spinner,
} from "evergreen-ui";
import React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { translate } from "piteo-kit";

const _ = translate();

export type AsyncLoadingItemProps = {
  id?: string;
  title: string;
  checked: boolean;
};

export type AsyncLoadingProps = {
  isShown: boolean;
  title: string;
  actions: AsyncLoadingItemProps[];
  isCancelable?: boolean;
};

export const AsyncLoading: React.FunctionComponent<AsyncLoadingProps> = ({
  isShown = false,
  title,
  isCancelable = false,
  actions = [],
}) => {
  const theme = useAppTheme();
  const completedItemsCount = actions?.filter((a) => a.checked).length || 0;

  return (
    <CornerDialog
      data-test-id="async-loading"
      title={title}
      isShown={isShown}
      hasClose={isCancelable}
      hasFooter={false}
    >
      <Pane display="flex" alignItems="center">
        <Heading size={100} marginY={theme.margin.medium}>
          {_("progress")}
        </Heading>
        <Badge marginX={theme.margin.medium}>
          {`${completedItemsCount}/${actions.length}`}
        </Badge>
      </Pane>

      {actions?.map((action, index) => {
        return <Pane
          key={index}
          display="flex"
          alignItems="center"
          justifyContent="space-between"
          marginTop={theme.margin.medium}
        >
          <Pane display="flex">
            {action.checked
              ? (<Icon icon={EndorsedIcon} color="success" />)
              : <Spinner size={16} />}
            <Heading size={300} marginLeft={theme.margin.medium}>
              {action.title}
            </Heading>
          </Pane>
        </Pane>;
      })}
    </CornerDialog>
  );
};
