use crate::File;
use crate::FileId;
use crate::Payment;
use crate::Rent;

pub type Receipt = File; // alias for a File

pub type ReceiptId = FileId; // alias for a FileId

pub type ReceiptWithRent = (Receipt, Rent, Payment);
