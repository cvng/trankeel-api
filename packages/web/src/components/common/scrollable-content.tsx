import React from "react";
import { MAIN_MENU_WIDTH } from "../main-menu/main-menu";
import { defaultTheme } from "evergreen-ui";

export class ScrollableContent extends React.Component {
  render = () => {
    return (
      <div
        style={{
          overflowY: "scroll",
          width: window.innerWidth - MAIN_MENU_WIDTH,
          background: defaultTheme.palette.blue.lightest,
        }}
      >
        {this.props.children}
      </div>
    );
  };
}
