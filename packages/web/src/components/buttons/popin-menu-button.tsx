import { Button, Menu, MoreIcon, Pane, Popover, Position } from "evergreen-ui";
import React from "react";
import { translate } from "piteo-kit";

const _ = translate();

export type PopinMenuButtonItemProps = {
  subItems: PopinMenuButtonSubItemProps[];
  title?: string;
  bottomDivider?: boolean;
};

export type PopinMenuButtonSubItemProps = {
  name: string;
  icon?: React.ElementType;
  handler: () => void;
  identifier?: string;
};

export type PopinMenuButtonProps = {
  items: PopinMenuButtonItemProps[];
  buttonIcon?: React.ElementType;
};

export const PopinMenuButton: React.FunctionComponent<
  PopinMenuButtonProps
> = ({
  items,
  buttonIcon,
}) => {
  return (
    <Popover
      position={Position.BOTTOM_LEFT}
      content={<Menu>
        {items.map((item: PopinMenuButtonItemProps, idx) => (
          <Pane key={idx}>
            <Menu.Group title={item.title} key={idx}>
              {item.subItems?.map((subItem, index) => (
                <Menu.Item
                  icon={subItem.icon}
                  onClick={subItem.handler}
                  key={index}
                  data-test-id={subItem.identifier}
                >
                  {subItem.name}
                </Menu.Item>
              ))}
            </Menu.Group>
            {item.bottomDivider && <Menu.Divider />}
          </Pane>
        ))}
      </Menu>}
    >
      <Button
        iconBefore={buttonIcon}
        iconAfter={MoreIcon}
      >
        {_("actions")}
      </Button>
    </Popover>
  );
};
