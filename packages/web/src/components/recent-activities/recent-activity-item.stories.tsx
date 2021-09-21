import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { eventModel } from "../../helpers/recent-activity-helper";
import { Themable } from "../common/themable";
import { RecentActivityItem } from "./recent-activity-item";

export default {
  title: "Activities/RecentActivityItem",
  component: RecentActivityItem,
};

const eventList = FactoryHelper.eventList();

export const noValue = () =>
  <Themable>
    <RecentActivityItem
      event={eventModel(eventList[0])}
    />
  </Themable>;
