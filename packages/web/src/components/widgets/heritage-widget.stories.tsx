import React from "react";
import { HeritageWidget } from "./heritage-widget";

export default {
  title: "Dashboard/HeritageWidget",
  component: HeritageWidget,
};

export const noData = () => <HeritageWidget />;

export const fill = () => (
  <HeritageWidget propertyCount={5} tenantCount={18} contractCount={9} />
);

export const fillSingle = () => (
  <HeritageWidget propertyCount={1} tenantCount={1} contractCount={1} />
);
