import { translate } from "piteo-kit";
import { RentStatus } from "../types";
import { Color } from "./color-helper";

const _ = translate();

export class RentHelper {
  static rentStatusMap = (): Map<RentStatus, string> => {
    return new Map<RentStatus, string>([
      [RentStatus.Settled, _("rent_status_completed")],
      [RentStatus.Partial, _("rent_status_partial")],
      [RentStatus.Pending, _("rent_status_pending")],
    ]);
  };

  static rentStatusMapColor = (status: RentStatus): Color => {
    const map = new Map<RentStatus, Color>([
      [RentStatus.Settled, "green"],
      [RentStatus.Partial, "orange"],
      [RentStatus.Pending, "yellow"],
    ]);
    return map.get(status);
  };
}
