import {
  Avatar,
  Button,
  CaretDownIcon,
  Heading,
  Menu,
  minorScale,
  Pane,
  Popover,
  Position,
  toaster,
} from "evergreen-ui";
import * as React from "react";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import { translate } from "piteo-kit";
import { signOut } from "../../services/auth-service";

const _ = translate();

export type ProfileItemProps = {
  profile: {
    displayName?: string;
    photoURL?: string;
  };
};

export const ProfileItem: React.FunctionComponent<ProfileItemProps> = ({
  profile,
}) => {
  const history = useHistory();
  return (
    <React.Fragment>
      <Pane display={"flex"} flexDirection="row" alignItems={"center"}>
        <Pane
          display="flex"
          flexDirection="row"
          alignItems="center"
          position="relative"
        >
        </Pane>
        <Popover
          position={Position.BOTTOM_LEFT}
          content={<Menu>
            <Menu.Divider />
            <Menu.Group>
              <Menu.Item
                onSelect={() =>
                  window.open(
                    _("send_feedback_email", {
                      email: _("app_email_contact"),
                    }),
                  )}
              >
                {_("support")}
              </Menu.Item>
              <Menu.Item
                onSelect={() => {
                  signOut()
                    .catch(() => {})
                    .finally(() => {
                      history.push(Routes.LOGIN);
                      toaster.success(_("message_sign_out_success"));
                    });
                }}
                intent="danger"
              >
                {_("logout")}
              </Menu.Item>
            </Menu.Group>
          </Menu>}
        >
          <Button appearance="minimal" iconAfter={CaretDownIcon}>
            <Avatar
              isSolid
              name={profile?.displayName}
              size={30}
              src={profile?.photoURL}
            />
            <Heading marginX={minorScale(2)} size={300}>
              {profile?.displayName}
            </Heading>
          </Button>
        </Popover>
      </Pane>
    </React.Fragment>
  );
};
