import { Heading, Icon, InfoSignIcon, Pane, Tooltip } from "evergreen-ui";
import React from "react";

export const TitleItem = (props: { title: string; tooltip?: string }) => {
  const { title, tooltip } = props;
  return (
    <Pane display="flex" flexDirection="row">
      <Heading size={400}>{title}</Heading>
      {tooltip?.length > 0 && (
        <Tooltip content={tooltip}>
          <Icon icon={InfoSignIcon} color="info" marginLeft={5} />
        </Tooltip>
      )}
    </Pane>
  );
};
