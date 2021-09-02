import React from "react";
import { WorkflowAction } from "./workflow-action";

export default {
  title: "Workflow/WorkflowAction",
  component: WorkflowAction,
};

export const standard = () =>
  <WorkflowAction
    name="Notifier par email"
    value="gail@piteo.fr"
  />;
