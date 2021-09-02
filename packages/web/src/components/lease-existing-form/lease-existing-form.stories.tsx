import React from "react";
import { Themable } from "../common/themable";
import { LeaseExistingForm } from "./lease-existing-form";

export default {
  title: "Lease/LeaseExistingForm",
  component: LeaseExistingForm,
};

export const standard = () => (
  <Themable>
    <LeaseExistingForm form={null} />
  </Themable>
);
