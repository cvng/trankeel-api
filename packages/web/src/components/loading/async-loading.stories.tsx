import React from "react";
import { Themable } from "../common/themable";
import { AsyncLoading } from "./";
import { AsyncLoadingItemProps } from "./async-loading";

export default {
  title: "Design System/AsyncLoading",
  component: AsyncLoading,
};

const actions: AsyncLoadingItemProps[] = [
  {
    checked: false,
    title: "Génération des quittances",
  },
  {
    checked: false,
    title: "Envoi des quittances par mail",
  },
];

export const standard = () =>
  <Themable>
    <AsyncLoading
      isShown={true}
      title={"Veuillez patienter..."}
      actions={actions}
    />;
  </Themable>;
