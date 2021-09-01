import React from "react";
import { Themable } from "../common/themable";
import { NavBar } from "./nav-bar";

export default {
  title: "Design System/NavBar",
  component: NavBar,
};

export const standard = () =>
  <Themable>
    <NavBar
      items={[
        {
          title: "Synthèse",
          route: "synthesis",
          available: true,
          selected: true,
        },
        { title: "Baux", route: "leases", available: true },
        {
          title: "Documents",
          route: "documents",
          available: false,
        },
      ]}
    />
  </Themable>;

export const withLastIndexSelected = () =>
  <Themable>
    <NavBar
      items={[{ title: "Synthèse", route: "synthesis", available: true }, {
        title: "Baux",
        route: "leases",
        available: true,
      }, {
        title: "Documents",
        route: "documents",
        available: false,
        selected: true,
      }]}
    />
  </Themable>;
