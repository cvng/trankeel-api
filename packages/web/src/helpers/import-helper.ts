import { ImportSource } from "../types";

export class ImportHelper {
  static readonly sources = (): Map<ImportSource, string> => {
    return new Map<ImportSource, string>([
      [ImportSource.Rentila, ImportSource.Rentila.toString()],
      // [ImportSource.RENDEMENT_LOCATIF, ImportSource.RENDEMENT_LOCATIF.toString()]
    ]);
  };
}
