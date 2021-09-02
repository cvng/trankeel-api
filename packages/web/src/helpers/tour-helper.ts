import { translate } from "piteo-kit";
import { TourType } from "../constants/tour-constants";

const _ = translate();

export interface TourItem {
  selector: string;
  content: string;
}

export class TourHelper {
  static tourContentMap = (): Map<TourType, string> => {
    return new Map<TourType, string>([
      [TourType.DASHBOARD_SHORTCUTS, _("dashboard_shortcuts")],
      [TourType.DASHBOARD_SYNTHESIS, _("dashboard_synthesis")],
      [TourType.DASHBOARD_GRAPH, _("dashboard_graph")],
      [TourType.DASHBOARD_GRAPH_STATUS, _("dashboard_graph_status")],
      [TourType.MAIN_MENU, _("main_menu")],
      [TourType.TRANSACTION_LIST, _("transaction_list")],
      [TourType.TRANSACTION_RENT_ADD_SEL_TENANT, _("tour_select_rent")],
      [
        TourType.TRANSACTION_RENT_ADD_SEL_RENT_LOCATION,
        _("tour_select_rental_location"),
      ],
      [
        TourType.TRANSACTION_RENT_ADD_SEL_RENT_AMOUNT,
        _("tour_select_rent_amount"),
      ],
      [
        TourType.TRANSACTION_RENT_ADD_RECEIVED_RENT_DATE,
        _("tour_select_received_rent_date"),
      ],
      [
        TourType.TRANSACTION_RENT_ADD_RENT_RECEIPT,
        _("tour_select_rent_receipt"),
      ],
      [
        TourType.CONTRACT_ADD_SELECT_TENANT,
        _("tour_contract_add_select_tenant"),
      ],
      [
        TourType.CONTRACT_ADD_SELECT_PROPERTY,
        _("tour_contract_add_select_property"),
      ],
      [
        TourType.CONTRACT_ADD_SET_CONTRACT_DATA,
        _("tour_contract_add_set_contract_data"),
      ],
      [TourType.CONTRACT_ADD_VALIDATE, _("tour_contract_add_validate")],
    ]);
  };

  static tourItem = (type: TourType): TourItem => {
    return {
      content: TourHelper.tourContentMap().get(type),
      // Lorsque l'on positionne le sélecteur le point est obligatoire'
      selector: "." + type,
    };
  };
}
