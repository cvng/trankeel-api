use async_graphql::Enum;

// # Types

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TransactionType {
    InsuranceHab,
    InsurancePno,
    Invoice,
    LoanInterest,
    LoanPayment,
    Other,
    Rent,
}

// # Impls

impl From<String> for TransactionType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "INSURANCE_HAB" => Self::InsuranceHab,
            "INSURANCE_PNO" => Self::InsurancePno,
            "INVOICE" => Self::Invoice,
            "LOAN_INTEREST" => Self::LoanInterest,
            "LOAN_PAYMENT" => Self::LoanPayment,
            "OTHER" => Self::Other,
            "RENT" => Self::Rent,
            _ => unimplemented!(),
        }
    }
}
