import moment from "moment";
import React from "react";
import { RentReceivedWidget } from "./rent-received-widget";

export default {
  title: "Dashboard/RentReceivedWidget",
  component: RentReceivedWidget,
};

export const loading = () => <RentReceivedWidget loading={true} />;

export const noValue = () => (
  <RentReceivedWidget
    startedDate={moment().toDate()}
    endedDate={moment().endOf("month").toDate()}
  />
);

export const withValue = () => (
  <RentReceivedWidget
    startedDate={moment().toDate()}
    endedDate={moment().endOf("month").toDate()}
    receivedAmount={450}
  />
);
