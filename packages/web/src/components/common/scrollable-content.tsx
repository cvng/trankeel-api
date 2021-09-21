import { defaultTheme, Pane } from "evergreen-ui";
import React from "react";

export class ScrollableContent extends React.Component {
  render = () => {
    return (
      <Pane
        flex={1}
        style={{
          overflowY: "scroll",
          background: defaultTheme.palette.blue.lightest,
        }}
      >
        {this.props.children}
      </Pane>
    );
  };
}
