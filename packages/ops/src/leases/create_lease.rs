use crate::error::Result;
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
use trankeel_data::Discussion;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use trankeel_data::LeaseType;
use trankeel_data::Lender;
use trankeel_data::Person;
use trankeel_data::Property;
use trankeel_data::Rent;
use trankeel_data::Tenant;
use trankeel_data::WarrantWithIdentity;
use validator::Validate;

type TenantWithIdentityExtended = (
    Tenant,
    Person,
    Option<Discussion>,
    Option<Vec<WarrantWithIdentity>>,
);

#[derive(InputObject, Validate)]
pub struct CreateLeaseInput {
    pub effect_date: DateTime,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub type_: LeaseType,
    pub property: CreatePropertyInput,
    pub tenants: Vec<CreateTenantInput>,
}

pub struct CreateLeasePayload {
    pub lease: Lease,
    pub rents: Vec<Rent>,
    pub property: Property,
    pub tenants_with_identities: Vec<TenantWithIdentityExtended>,
}

pub struct CreateLease {
    account: Account,
    account_owner: Person,
    lender: Lender,
}

impl CreateLease {
    pub fn new(account: &Account, account_owner: &Person, lender: &Lender) -> Self {
        Self {
            account: account.clone(),
            account_owner: account_owner.clone(),
            lender: lender.clone(),
        }
    }
}

impl Command for CreateLease {
    type Input = CreateLeaseInput;
    type Payload = CreateLeasePayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self {
            account,
            account_owner,
            lender,
        } = self;

        // Create property.
        let CreatePropertyPayload { property } = CreateProperty::new(&account, &lender) //
            .run(input.property)?;

        // Create lease.
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
            )
            .collect();

        Ok(Self::Payload {
            lease,
            rents,
            property,
            tenants_with_identities,
        })
    }
}
