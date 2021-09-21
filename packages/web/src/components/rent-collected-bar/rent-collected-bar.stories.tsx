import React from "react";
import { Themable } from "../common/themable";
import { RentCollectedBar } from "./rent-collected-bar";

export default {
  title: "Rent/RentCollectedBar",
  component: RentCollectedBar,
};

export const standard = () =>
  <Themable>
    <RentCollectedBar
      amountPartial={0}
      amountPending={700}
      amountReceived={300}
      ratioReceived={30}
      ratioPending={0}
      ratioPartial={0}
    />
  </Themable>;

export const allRentsCollected = () =>
  <Themable>
    <RentCollectedBar
      amountPartial={0}
      amountPending={0}
      amountReceived={1000}
      ratioReceived={100}
      ratioPending={0}
      ratioPartial={0}
    />
  </Themable>;

export const withPartialRentCollected = () =>
  <Themable>
    <RentCollectedBar
      amountPartial={100}
      amountPending={100}
      amountReceived={800}
      ratioReceived={80}
      ratioPending={10}
      ratioPartial={10}
    />
  </Themable>;

export const withOnlyPartialRent = () =>
  <Themable>
    <RentCollectedBar
      amountPartial={700}
      amountPending={300}
      amountReceived={0}
      ratioReceived={0}
      ratioPending={30}
      ratioPartial={70}
    />
  </Themable>;
