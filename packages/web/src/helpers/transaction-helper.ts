import { translate } from "piteo-kit";
import { TransactionType } from "../types";

const _ = translate("Transaction");

export class TransactionHelper {
  static transactionType = (): Map<TransactionType, string> => {
    return new Map<TransactionType, string>([
      [TransactionType.Invoice, _("invoice")],
      [TransactionType.Rent, _("rent")],
      [TransactionType.InsurancePno, _("insurance_pno")],
      [TransactionType.InsuranceHab, _("insurance_hab")],
      [TransactionType.LoanInterest, _("loan_interest")],
      [TransactionType.LoanPayment, _("loan_payment")],
      [TransactionType.Other, _("other_transaction")],
    ]);
  };

  static transactionChargesType = (): Map<TransactionType, string> => {
    return new Map<TransactionType, string>([
      [TransactionType.Invoice, _("invoice")],
      [TransactionType.InsurancePno, _("insurance_pno")],
      [TransactionType.InsuranceHab, _("insurance_hab")],
      [TransactionType.LoanInterest, _("loan_interest")],
      [TransactionType.LoanPayment, _("loan_payment")],
      [TransactionType.Other, _("other_transaction")],
    ]);
  };
}
