import { FileType } from "../types";

// https://github.com/segmentio/evergreen/blob/master/index.d.ts#L523
export type Color =
  | "yellow"
  | "red"
  | "blue"
  | "green"
  | "orange"
  | "purple"
  | "teal"
  | "automatic"
  | "neutral";

export class ColorHelper {
  static fileMapColor = (status: FileType): Color => {
    const map = new Map<FileType, Color>([
      [FileType.LeaseDocument, "green"],
      [FileType.PaymentNotice, "yellow"],
      [FileType.RentReceipt, "neutral"],
    ]);
    return map.get(status);
  };
}
