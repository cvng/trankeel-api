use super::CreateLeaseInput;
use super::CreateLeaseState;
use crate::leases;
use crate::properties;
use crate::properties::CreatePropertyState;
use crate::tenants;
use crate::tenants::CreateTenantState;
use crate::Command;
use crate::CreatePropertyInput;
use crate::CreateTenantInput;
use crate::Result;
use trankeel_data::Account;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::Discussion;
use trankeel_data::Lease;
use trankeel_data::LeaseType;
use trankeel_data::Person;
use trankeel_data::Property;
use trankeel_data::Rent;
use trankeel_data::TenantWithIdentity;

#[derive(InputObject, Validate)]
pub struct AddExistingLeaseInput {
    pub effect_date: DateTime,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub type_: LeaseType,
    pub property: CreatePropertyInput,
    pub tenants: Vec<CreateTenantInput>,
}

pub struct AddExistingLeaseState {
    pub account: Account,
    pub account_owner: Person,
}

pub struct AddExistingLeasePayload {
    pub lease: Lease,
    pub rents: Vec<Rent>,
    pub property: Property,
    pub tenants: Vec<TenantWithIdentity>,
    pub discussions: Option<Vec<Discussion>>,
}

pub struct AddExistingLease;

impl Command for AddExistingLease {
    type Input = AddExistingLeaseInput;
    type State = AddExistingLeaseState;
    type Payload = AddExistingLeasePayload;

    fn run(state: Self::State, input: Self::Input) -> Result<Self::Payload> {
        let account = state.account;
        let account_owner = state.account_owner;

        // Add property.
        let property = properties::create_property(
            CreatePropertyState {
                account: account.clone(),
            },
            input.property,
        )?;

        // Add lease.
        let lease = leases::create_lease(
            CreateLeaseState {
                account: account.clone(),
            },
            CreateLeaseInput {
                effect_date: input.effect_date,
                rent_amount: input.rent_amount,
                rent_charges_amount: input.rent_charges_amount,
                type_: input.type_,
                property_id: property.property.id,
            },
        )?;

        // Add rents.
        let rents = lease.lease.rents();

        // Add tenant.
        let mut tenants = vec![];
        let mut discussions = vec![];

        for tenant_input in input.tenants {
            let mut tenant = tenants::create_tenant(
                CreateTenantState {
                    account: account.clone(),
                    account_owner: account_owner.clone(),
                    tenant_identity: None,
                },
                tenant_input,
            )?;

            // Attach tenant to lease.
            tenant.tenant.0.lease_id = Some(lease.lease.id);

            tenants.push(tenant.tenant);
            discussions.push(tenant.discussion);
        }

        Ok(AddExistingLeasePayload {
            lease: lease.lease,
            rents,
            property: property.property,
            tenants,
            discussions: Some(discussions.into_iter().flatten().collect()),
        })
    }
}
