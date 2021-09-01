export class NumberHelper extends Number {
  static formatToString(
    rawNumber: unknown,
    addSign = true,
    currency = "EUR",
  ): string {
    if (typeof rawNumber !== "number") return;
    // deno-lint-ignore no-undef https://developer.mozilla.org/fr/docs/Web/JavaScript/Reference/Objets_globaux/Intl/NumberFormat
    const formatter = Intl.NumberFormat("fr-FR", {
      style: "currency",
      currency: currency,
    });

    let formattedNumber = formatter.format(rawNumber);
    if (addSign && rawNumber > 0) formattedNumber = "+" + formattedNumber;

    return formattedNumber;
  }

  static formatToPercentage(rawNumber: unknown, toFixed = 2): string {
    if (typeof rawNumber !== "number") return;
    const value = parseFloat(rawNumber.toString()).toFixed(toFixed);
    return `${value}\u00a0%`;
  }

  static formatToEurocents(rawNumber: unknown): number {
    if (typeof rawNumber !== "number") return 0;
    return rawNumber * 100;
  }

  // Calcule le montant de la TVA (20%)
  static calculateVat(
    eurocentsNumber: number,
  ): { vatAmount: number; amount: number } {
    return {
      amount: (eurocentsNumber / 120) * 100,
      vatAmount: (eurocentsNumber / 120) * 20,
    };
  }
}
