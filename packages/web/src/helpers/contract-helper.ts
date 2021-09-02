import { translate } from "piteo-kit";
import { LeaseStatus } from "../types";
import { Color } from "./color-helper";

const _ = translate();

export class ContractHelper {
  static statusMap = (): Map<LeaseStatus, string> => {
    return new Map<LeaseStatus, string>([
      [LeaseStatus.Active, _("status_active")],
      [LeaseStatus.Ended, _("status_ended")],
    ]);
  };

  static statusMapColor = (status: LeaseStatus): Color => {
    const map = new Map<LeaseStatus, Color>([
      [LeaseStatus.Active, "yellow"],
      [LeaseStatus.Ended, "red"],
    ]);
    return map.get(status);
  };
}
