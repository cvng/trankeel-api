use crate::error::Result;
use trankeel_core::dispatcher::Command;
use trankeel_data::Account;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use trankeel_data::LeaseType;
use trankeel_data::LeaseWithRents;
use trankeel_data::PropertyId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateLeaseInput {
    pub effect_date: DateTime,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub type_: LeaseType,
    pub property_id: PropertyId,
}

pub struct CreateLeasePayload {
    pub lease: LeaseWithRents,
}

pub(crate) struct CreateLease {
    account: Account,
}

impl CreateLease {
    pub fn new(account: &Account) -> Self {
        Self {
            account: account.clone(),
        }
    }
}

impl Command for CreateLease {
    type Input = CreateLeaseInput;
    type Payload = CreateLeasePayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { account } = self;

        let lease = Lease {
            id: LeaseId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            deposit_amount: Default::default(),
            effect_date: input.effect_date,
            signature_date: None,
            rent_amount: input.rent_amount,
            rent_charges_amount: input.rent_charges_amount,
            type_: input.type_,
            lease_id: None,
            property_id: input.property_id,
            details: None,
            expired_at: None,
            renew_date: None,
            duration: Default::default(),
        };

        let rents = lease.rents();

        let lease = (lease, rents);

        Ok(Self::Payload { lease })
    }
}
