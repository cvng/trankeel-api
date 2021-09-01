import React from "react";
import { PhoneNumberField } from "./phone-number-field";

export default {
  title: "Design System/Fields/PhoneNumberField",
  component: PhoneNumberField,
};

export const standard = () => (
  <PhoneNumberField
    label={"Téléphone"}
    placeholder={"Votre numéro de téléphone"}
    options={[]}
    onChange={() => {}}
  />
);
