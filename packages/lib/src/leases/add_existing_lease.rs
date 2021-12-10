use super::CreateLease;
use super::CreateLeaseInput;
use crate::properties::CreateProperty;
use crate::tenants;
use crate::tenants::CreateTenantState;
use crate::CreatePropertyInput;
use crate::CreateTenantInput;
use crate::CreateTenantPayload;
use crate::Result;
use trankeel_core::dispatcher::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::handlers::LeaseAffected;
use trankeel_core::handlers::LeaseCreated;
use trankeel_core::handlers::PropertyCreated;
use trankeel_core::handlers::TenantCreated;
use trankeel_data::Account;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use trankeel_data::LeaseType;
use trankeel_data::Lender;
use trankeel_data::Person;
use trankeel_data::PropertyId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct AddExistingLeaseInput {
    pub effect_date: DateTime,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub type_: LeaseType,
    pub property: CreatePropertyInput,
    pub tenants: Vec<CreateTenantInput>,
}

pub(crate) struct AddExistingLease {
    pub lease_id: LeaseId,
    pub account: Account,
    pub account_owner: Person,
    pub lender: Lender,
}

impl AddExistingLease {
    pub fn new(lease_id: LeaseId, account: Account, account_owner: Person, lender: Lender) -> Self {
        Self {
            lease_id,
            account,
            account_owner,
            lender,
        }
    }
}

impl Command for AddExistingLease {
    type Input = AddExistingLeaseInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let AddExistingLease {
            lease_id,
            account,
            account_owner,
            lender,
        } = self;

        // Create property.
        let property = CreateProperty::new(PropertyId::new(), account.clone(), lender)
            .create_property(input.property)?;

        // Create lease.
        let (lease, rents) = CreateLease::new(lease_id, account.clone()) //
            .create_lease(CreateLeaseInput {
                effect_date: input.effect_date,
                rent_amount: input.rent_amount,
                rent_charges_amount: input.rent_charges_amount,
                type_: input.type_,
                property_id: property.id,
            })?;

        // Make the lease active by using a signature date.
        let lease = Lease {
            signature_date: Some(lease.effect_date),
            ..lease
        };

        // Create tenants.
        let tenants_with_identities = input
            .tenants
            .into_iter()
            .map(|tenant_input| {
                tenants::create_tenant(
                    CreateTenantState {
                        account: account.clone(),
                        account_owner: account_owner.clone(),
                        tenant_identity: None,
                    },
                    tenant_input,
                )
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(
                |CreateTenantPayload {
                     tenant,
                     tenant_identity,
                     warrants,
                     discussion,
                 }| {
                    (
                        // Attach tenant to lease.
                        tenant.with_lease(&lease_id),
                        tenant_identity,
                        discussion,
                        warrants,
                    )
                },
            );

        Ok(vec![
            PropertyCreated { property }.into(),
            LeaseCreated { lease, rents }.into(),
        ]
        .into_iter()
        .chain(
            tenants_with_identities
                .clone()
                .map(|(tenant, identity, discussion, warrants)| {
                    TenantCreated {
                        tenant,
                        identity,
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
                        lease_id,
                        tenant_id: tenant.id,
                    }
                    .into()
                })
                .collect::<Vec<_>>(),
        )
        .collect())
    }
}
