import { useQuery } from "@apollo/client";
import { FunctionComponent } from "react";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import { InvoiceListQuery } from "../../helpers";
import { SettingsBillingProps } from "./settings-billing";

export const withContainer = (Component): FunctionComponent<unknown> =>
  () => {
    const history = useHistory();
    const result = useQuery(InvoiceListQuery);

    const handleClickActivate = () => {
      history.push(Routes.SUBSCRIBE);
    };

    const componentProps: SettingsBillingProps = {
      result,
      onClickActivate: handleClickActivate,
    };

    return Component(componentProps);
  };
