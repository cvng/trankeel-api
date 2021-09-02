import { Button, Heading, HeartIcon, Pane } from "evergreen-ui";
import React from "react";
import { translate } from "piteo-kit";

const _ = translate();

export type WelcomeWidgetProps = {
  fullName?: string;
  onInviteFriendClick?: () => void;
};

export const WelcomeWidget: React.FunctionComponent<WelcomeWidgetProps> = ({
  fullName = _("investor"),
  onInviteFriendClick,
}) => {
  return (
    <Pane
      display="flex"
      flexDirection="row"
      justifyContent="space-between"
      height={30}
    >
      <Pane display="flex" flexDirection="column" justifyContent="center">
        <Heading size={600}>{_("welcome_message", { fullName })}</Heading>
      </Pane>
      <Pane>
        <Button iconBefore={HeartIcon} onClick={onInviteFriendClick}>
          {_("invite_friend")}
        </Button>
      </Pane>
    </Pane>
  );
};
