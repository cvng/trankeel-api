import { read, utils } from "xlsx";

// https://simon-schraeder.de/posts/filereader-async

export const SheetNames = ["Biens", "Locataires", "Locations"];

export type Workbook = {
  properties?: number;
  tenants?: number;
  contracts?: number;
};

export function parseWorkbook(
  files: File[],
  callback: (workbook: Workbook) => void,
): void {
  if (!files.length) return;

  const reader = new FileReader();

  reader.onload = (event) => {
    const buffer = event.target.result;
    const workbook = read(buffer, { type: "array" });

    const [properties, tenants, contracts] = SheetNames.map((sheet) => {
      return (
        utils.sheet_to_json(workbook.Sheets[sheet], { header: 1 }).length - 1
      );
    });

    callback({ properties, tenants, contracts });
  };

  reader.readAsArrayBuffer(files[0]);
}
