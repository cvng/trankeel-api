import { translate } from "piteo-kit";
import { version } from "../../package.json";
import { env } from "../utils";

const isRunningProd = env("NODE_ENV") === "production";

const _ = translate();

export class VersionNumberHelper {
  static fullVersionNumber(): string {
    return _("version_number_format", { versionNumber: version });
  }

  static versionNumber(): string {
    return `${version} ${isRunningProd ? "" : "DEV"}`;
  }
}
