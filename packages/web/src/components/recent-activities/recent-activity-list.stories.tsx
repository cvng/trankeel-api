import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { eventModel } from "../../helpers/recent-activity-helper";
import { Event } from "../../types";
import { Themable } from "../common/themable";
import { RecentActivityList } from "./recent-activity-list";

export default {
  title: "Activities/RecentActivityList",
  component: RecentActivityList,
};

const eventList: Event[] = FactoryHelper.eventList();

export const loading = () =>
  <Themable>
    <RecentActivityList
      loading
      events={eventList.map((event: Event) => eventModel(event))}
    />
  </Themable>;

export const withData = () =>
  <Themable>
    <RecentActivityList
      loading={false}
      events={eventList.map((event: Event) => eventModel(event))}
    />
  </Themable>;
