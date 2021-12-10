use crate::error::Result;
use trankeel_core::dispatcher::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::handlers::LeaseCreated;
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

pub(crate) struct CreateLease {
    pub lease_id: LeaseId,
    pub account: Account,
}

impl CreateLease {
    pub fn new(lease_id: LeaseId, account: Account) -> Self {
        Self { lease_id, account }
    }

    pub fn create_lease(self, input: CreateLeaseInput) -> Result<LeaseWithRents> {
        input.validate()?;

        let CreateLease { lease_id, account } = self;

        let lease = Lease {
            id: lease_id,
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

        Ok((lease, rents))
    }
}

impl Command for CreateLease {
    type Input = CreateLeaseInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        let (lease, rents) = self.create_lease(input)?;

        Ok(vec![LeaseCreated { lease, rents }.into()])
    }
}
