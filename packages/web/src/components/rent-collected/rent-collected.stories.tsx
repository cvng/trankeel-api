import React from "react";
import { Themable } from "../common/themable";
import { RentCollected } from "./rent-collected";

export default {
  title: "Rent/RentCollected",
  component: RentCollected,
  argTypes: {
    totalRentAmount: 0,
    pendingRentAmount: 0,
  },
};

const Template = (args) =>
  <Themable>
    <RentCollected {...args} />
  </Themable>;

export const standard = Template.bind({});

standard.args = {
  totalRentAmount: 1800,
  pendingRentAmount: 80,
};
