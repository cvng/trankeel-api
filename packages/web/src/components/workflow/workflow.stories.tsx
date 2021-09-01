import React from "react";
import { WorkflowTrigger } from "./workflow-trigger";
import { WorkflowAction } from "./workflow-action";
import { Pane } from "evergreen-ui";

export default {
  title: "Workflow/WorkflowUsecase",
  component: Pane,
};

export const standard = () =>
  <Pane>
    <WorkflowTrigger />
    <WorkflowAction name="Notifier par email" value="gail@piteo.fr" />
    <WorkflowAction name="Notifier sur Whatsapp" value="06 58 58 28 90" />
  </Pane>;
