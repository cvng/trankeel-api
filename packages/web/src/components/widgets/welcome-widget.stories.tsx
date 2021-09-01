import React from "react";
import { WelcomeWidget } from "./welcome-widget";

export default {
  title: "Dashboard/WelcomeWidget",
  component: WelcomeWidget,
  argTypes: {
    fullName: { control: "text" },
  },
};

export const withoutFullNameSet = () => <WelcomeWidget />;

export const withFullNameSet = (args) => <WelcomeWidget {...args} />;
withFullNameSet.args = {
  fullName: "Gaïl Sanctussy",
};
