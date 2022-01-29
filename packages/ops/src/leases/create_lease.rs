use crate::error::Result;
use crate::event::Event;
use crate::event::LeaseAffected;
use crate::event::LeaseCreated;
use crate::event::PropertyCreated;
use crate::event::TenantCreated;
use crate::properties::CreateProperty;
use crate::properties::CreatePropertyInput;
use crate::properties::CreatePropertyPayload;
use crate::tenants::CreateTenant;
use crate::tenants::CreateTenantInput;
use crate::tenants::CreateTenantPayload;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::Account;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use trankeel_data::LeaseType;
use trankeel_data::Lender;
use trankeel_data::Person;
use trankeel_data::Tenant;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateLeaseInput {
    pub effect_date: DateTime,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub type_: LeaseType,
    pub property: CreatePropertyInput,
    pub tenants: Vec<CreateTenantInput>,
}

pub struct CreateLease {
    lease_id: LeaseId,
    account: Account,
    account_owner: Person,
    lender: Lender,
}

impl CreateLease {
    pub fn new(
        lease_id: LeaseId,
        account: &Account,
        account_owner: &Person,
        lender: &Lender,
    ) -> Self {
        Self {
            lease_id,
            account: account.clone(),
            account_owner: account_owner.clone(),
            lender: lender.clone(),
        }
    }
}

impl Command for CreateLease {
    type Input = CreateLeaseInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self {
            lease_id,
            account,
            account_owner,
            lender,
        } = self;

        // Create property.
        let CreatePropertyPayload { property } = CreateProperty::new(&account, &lender) //
            .run(input.property)?;

        // Create lease.
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
            property_id: property.id,
            details: None,
            expired_at: None,
            renew_date: None,
            duration: Default::default(),
        };

        // Generate rents.
        let rents = lease.rents();

        // Make the lease active by using a signature date.
        let lease = Lease {
            signature_date: Some(lease.effect_date),
            ..lease
        };

        // Create tenants.
        let tenants_with_identities = input
            .tenants
            .into_iter()
            .map(|tenant_input| CreateTenant::new(&account, &account_owner, None).run(tenant_input))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(
                |CreateTenantPayload {
                     tenant,
                     identity,
                     warrants,
                     discussion,
                 }| {
                    (
                        // Attach tenant to lease.
                        Tenant {
                            lease_id: Some(lease.id),
                            ..tenant
                        },
                        identity,
                        discussion,
                        warrants,
                    )
                },
            );

        Ok(vec![
            PropertyCreated { property }.into(),
            LeaseCreated {
                lease: lease.clone(),
                rents,
            }
            .into(),
        ]
        .into_iter()
        .chain(
            tenants_with_identities
                .clone()
                .into_iter()
                .map(|(tenant, identity, discussion, warrants)| {
                    TenantCreated {
                        tenant,
                        identity: Some(identity),
                        discussion,
                        warrants,
                    }
                    .into()
                })
                .collect::<Vec<_>>(),
        )
        .chain(
            tenants_with_identities
                .map(|(tenant, ..)| {
                    LeaseAffected {
                        lease_id: lease.id,
                        tenant_id: tenant.id,
                    }
                    .into()
                })
                .collect::<Vec<_>>(),
        )
        .collect())
    }
}
