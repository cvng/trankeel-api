import { translate } from "piteo-kit";
import { PlanCode } from "../types";

const _ = translate();

export class SubscriptionHelper {
  static plan = (): Map<PlanCode, string> => {
    return new Map<PlanCode, string>([[PlanCode.Solo, _("pricing_plan_solo")]]);
  };

  static planDetails = (): Map<PlanCode, Array<string>> => {
    return new Map<PlanCode, Array<string>>([
      [PlanCode.Solo, _("pricing_plan_solo_details").split("\n")],
    ]);
  };
}
