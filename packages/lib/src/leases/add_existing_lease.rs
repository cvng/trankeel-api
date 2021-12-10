use super::CreateLeaseInput;
use super::CreateLeasePayload;
use super::CreateLeaseState;
use crate::leases;
use crate::properties::CreateProperty;
use crate::tenants;
use crate::tenants::CreateTenantState;
use crate::CreatePropertyInput;
use crate::CreateTenantInput;
use crate::CreateTenantPayload;
use crate::Result;
use trankeel_core::dispatcher::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::handlers::LeaseCreated;
use trankeel_core::handlers::PropertyCreated;
use trankeel_core::handlers::TenantCreated;
use trankeel_data::Account;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::LeaseId;
use trankeel_data::LeaseType;
use trankeel_data::Lender;
use trankeel_data::Person;
use trankeel_data::PropertyId;

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
        let CreateLeasePayload { mut lease } = leases::create_lease(
            CreateLeaseState {
                lease_id,
                account: account.clone(),
            },
            CreateLeaseInput {
                effect_date: input.effect_date,
                rent_amount: input.rent_amount,
                rent_charges_amount: input.rent_charges_amount,
                type_: input.type_,
                property_id: property.id,
            },
        )?;

        // Make the lease active by using a signature date.
        lease.signature_date = Some(lease.effect_date);

        // Generate rents.
        let rents = lease.rents();

        // Create tenants.
        let mut tenants = vec![];

        for tenant_input in input.tenants {
            let CreateTenantPayload {
                mut tenant,
                tenant_identity,
                warrants,
                discussion,
            } = tenants::create_tenant(
                CreateTenantState {
                    account: account.clone(),
                    account_owner: account_owner.clone(),
                    tenant_identity: None,
                },
                tenant_input,
            )?;

            // Attach tenant to lease.
            tenant.lease_id = Some(lease.id);

            tenants.push((tenant, tenant_identity, discussion, warrants));
        }

        Ok(vec![
            PropertyCreated { property }.into(),
            LeaseCreated { lease, rents }.into(),
        ]
        .into_iter()
        .chain(
            tenants
                .into_iter()
                .map(|(tenant, identity, discussion, warrants)| {
                    TenantCreated {
                        tenant,
                        identity,
                        discussion,
                        warrants,
                    }
                    .into()
                })
                .collect::<Vec<Event>>(),
        )
        .collect())
    }
}
