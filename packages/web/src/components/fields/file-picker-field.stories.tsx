import React from "react";
import { FilePickerField } from "./file-picker-field";

export default {
  title: "Design System/Fields/FilePickerField",
  component: FilePickerField,
};

// @ts-ignore: https://github.com/piteo-team/piteo/pull/166
export const standard = () => (
  <FilePickerField placeholder={"Votre numéro de téléphone"} />
);
