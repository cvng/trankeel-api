import moment from "moment";

export class DateHelper {
  static formatToString(
    rawDate: unknown,
    inFormat = "YYYY-MM-DD",
    outFormat = "DD/MM/YYYY",
  ): string {
    if (!rawDate) return;
    // https://momentjs.com/docs/#/displaying/format
    return moment(rawDate, inFormat).format(outFormat);
  }
}
