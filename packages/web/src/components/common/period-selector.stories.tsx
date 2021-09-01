import moment from "moment";
import React from "react";
import { PeriodSelector } from "./period-selector";

export default {
  title: "Design System/PeriodSelector",
  component: PeriodSelector,
};

export const standard = () => (
  <PeriodSelector
    startedDate={moment()
      .startOf("month")
      .toDate()}
    endedDate={moment()
      .endOf("month")
      .toDate()}
  />
);

export const isEditable = () => (
  <PeriodSelector
    isEditable
    startedDate={moment()
      .startOf("month")
      .toDate()}
    endedDate={moment()
      .endOf("month")
      .toDate()}
  />
);
