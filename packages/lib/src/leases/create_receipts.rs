use crate::error::Result;
use async_graphql::InputObject;
use chrono::Utc;
use trankeel_core::dispatcher::Command;
use trankeel_data::DateTime;
use trankeel_data::Payment;
use trankeel_data::PaymentId;
use trankeel_data::Receipt;
use trankeel_data::ReceiptWithRent;
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

pub struct CreateReceiptsPayload {
    pub receipts: Vec<ReceiptWithRent>,
}

pub(crate) struct CreateReceipts {
    rents: Vec<Rent>,
}

impl CreateReceipts {
    pub fn new(rents: &[Rent]) -> Self {
        Self {
            rents: rents.to_vec(),
        }
    }
}

impl Command for CreateReceipts {
    type Input = CreateReceiptsInput;
    type Payload = CreateReceiptsPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { rents } = self;

        let receipts = rents
            .into_iter()
            .map(|rent| {
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

                (receipt, rent, payment)
            })
            .collect();

        Ok(Self::Payload { receipts })
    }
}
