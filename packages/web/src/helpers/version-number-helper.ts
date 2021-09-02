import { translate } from "piteo-kit";
import packageJson from "../../package.json";

const isRunningProd = process.env.NODE_ENV === "production";

const _ = translate();

export class VersionNumberHelper {
  static fullVersionNumber(): string {
    return _("version_number_format", { versionNumber: packageJson.version });
  }

  static versionNumber(): string {
    return `${packageJson.version} ${isRunningProd ? "" : "DEV"}`;
  }
}
