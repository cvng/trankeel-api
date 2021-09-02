import React from "react";
import { ShortcutsWidget } from "./shortcuts-widget";

export default {
  title: "Dashboard/ShortcutsWidget",
  component: ShortcutsWidget,
};

export const available = () => <ShortcutsWidget available={true} />;

export const notAvailable = () => <ShortcutsWidget available={false} />;
