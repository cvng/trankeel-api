import React from "react";
import {
  Button,
  EditIcon,
  IconButton,
  Menu,
  MoreIcon,
  Popover,
  Position,
  TrashIcon,
} from "evergreen-ui";
import { translate } from "piteo-kit";

const _ = translate();

interface Props {
  editTitle?: string;
  editEnabled?: boolean;
  deleteTitle?: string;
  isDeleting?: boolean;
  onClickEdit?(): void;
  onClickDelete?(): void;
}

export const EditMenu = (props: Props) => {
  const {
    onClickEdit,
    onClickDelete,
    editTitle,
    editEnabled = true,
    deleteTitle,
    isDeleting = false,
  } = props;
  return (
    <React.Fragment>
      <Popover
        position={Position.BOTTOM_LEFT}
        content={({ close }) => (
          <Menu>
            {editEnabled && (
              <Menu.Group>
                <Menu.Item
                  onSelect={() => {
                    close();
                    onClickEdit();
                  }}
                  icon={EditIcon}
                >
                  {editTitle ? editTitle : _("edit")}
                </Menu.Item>
              </Menu.Group>
            )}
            <Menu.Divider />
            <Menu.Group>
              <Menu.Item
                onSelect={() => {
                  close();
                  onClickDelete();
                }}
                intent="danger"
                icon={TrashIcon}
              >
                {deleteTitle ? deleteTitle : _("delete")}
              </Menu.Item>
            </Menu.Group>
          </Menu>
        )}
      >
        {isDeleting
          ? (
            <Button disabled>{_("please_wait")}</Button>
          )
          : (
            <IconButton height={24} icon={MoreIcon} />
          )}
      </Popover>
    </React.Fragment>
  );
};
