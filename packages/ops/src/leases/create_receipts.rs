use crate::command::Command;
use crate::error::DomainError;
use crate::error::Result;
use crate::event::Event;
use crate::event::ReceiptCreated;
use async_graphql::InputObject;
use chrono::Utc;
use trankeel_data::DateTime;
use trankeel_data::LenderWithIdentity;
use trankeel_data::Payment;
use trankeel_data::PaymentId;
use trankeel_data::Receipt;
use trankeel_data::Rent;
use trankeel_data::RentId;
use trankeel_data::RentStatus;
use trankeel_data::TransactionType;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateReceiptsInput {
    pub rent_ids: Vec<RentId>,
    pub date: Option<DateTime>,
    pub send_mail: Option<bool>,
}

pub struct CreateReceipts {
    rents: Vec<Rent>,
    lenders: Vec<LenderWithIdentity>,
}

impl CreateReceipts {
    pub fn new(rents: &[Rent], lenders: &[LenderWithIdentity]) -> Self {
        Self {
            rents: rents.to_vec(),
            lenders: lenders.to_vec(),
        }
    }
}

impl Command for CreateReceipts {
    type Input = CreateReceiptsInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self { rents, lenders } = self;

        // Early return if any lender has a missing address.
        lenders
            .iter()
            .map(|lender| match lender.1.address() {
                Some(_) => Ok(lender),
                None => Err(DomainError::MissingLenderAddress(lender.0.id).into()),
            })
            .collect::<Result<Vec<_>>>()?;

        let receipts = rents.into_iter().map(|rent| {
            // Create payment.
            let payment = Payment {
                id: PaymentId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                rent_id: rent.id,
                amount: rent.full_amount,
                date: Utc::now().into(),
                type_: TransactionType::Rent,
                label: None,
            };

            // Mark rent as settled.
            let rent = Rent {
                status: RentStatus::Paid,
                ..rent
            };

            // Create new receipt.
            let receipt = Receipt::receipt_document(&rent);

            // Link receipt with rent.
            let rent = Rent {
                receipt_id: Some(receipt.id),
                ..rent
            };

            (receipt, rent, payment)
        });

        Ok(receipts
            .into_iter()
            .map(|(receipt, rent, payment)| {
                ReceiptCreated {
                    receipt,
                    rent,
                    payment,
                }
                .into()
            })
            .collect())
    }
}
