use async_graphql::Enum;

/// https://www.pdfmonkey.io/fr/doc/api/generer-un-document
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum FileStatus {
    Draft,
    Failure,
    Generating,
    Pending,
    Success,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum FileType {
    PaymentNotice,
    LeaseDocument,
    RentReceipt,
}
