import React from "react";
import { AccountIndividualForm } from "./account-individual-form";

export default {
  title: "Login/AccountIndividualForm",
  component: AccountIndividualForm,
};

export const individual = () => <AccountIndividualForm hasFooter={false} />;
