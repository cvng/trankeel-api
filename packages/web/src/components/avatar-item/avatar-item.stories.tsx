import React from "react";
import { Themable } from "../common/themable";
import { AvatarItem } from "./avatar-item";

export default {
  title: "Design System/AvatarItem",
  component: AvatarItem,
};

export const standard = () =>
  <Themable>
    <AvatarItem name={"John Doe"} />
  </Themable>;

export const standardWithHandler = () =>
  <Themable>
    <AvatarItem name={"Jane Doe"} handler={() => {}} />
  </Themable>;

export const withPicture = () =>
  <Themable>
    <AvatarItem name={"John Doe"} photoURL={"https://placebeard.it/640x360"} />
  </Themable>;
