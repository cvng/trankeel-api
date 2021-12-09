use super::CreateLeaseInput;
use super::CreateLeaseState;
use crate::leases;
use crate::leases::CreateLeasePayload;
use crate::properties::CreateProperty;
use crate::tenants;
use crate::tenants::CreateTenantState;
use crate::CreatePropertyInput;
use crate::CreateTenantInput;
use crate::CreateTenantPayload;
use crate::Result;
use trankeel_core::context::Context;
use trankeel_core::database::Db;
use trankeel_core::dispatcher::AsyncCommand;
use trankeel_core::error::Error;
use trankeel_data::Account;
use trankeel_data::Amount;
use trankeel_data::AuthId;
use trankeel_data::DateTime;
use trankeel_data::Discussion;
use trankeel_data::Lease;
use trankeel_data::LeaseType;
use trankeel_data::Lender;
use trankeel_data::Person;
use trankeel_data::Property;
use trankeel_data::Rent;
use trankeel_data::Tenant;

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
    pub lender: Lender,
}

pub struct AddExistingLeasePayload {
    pub lease: Lease,
    pub rents: Vec<Rent>,
    pub property: Property,
    pub tenants: Vec<Tenant>,
    pub identities: Vec<Person>,
    pub discussions: Vec<Discussion>,
}

pub(crate) struct AddExistingLease<'a> {
    context: &'a Context,
    auth_id: &'a AuthId,
}

impl<'a> AddExistingLease<'a> {
    pub fn new(context: &'a Context, auth_id: &'a AuthId) -> Self {
        Self { context, auth_id }
    }
}

#[async_trait]
impl<'a> AsyncCommand for AddExistingLease<'a> {
    type Input = AddExistingLeaseInput;
    type Payload = AddExistingLeasePayload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let account = db.accounts().by_auth_id(self.auth_id)?;
        let account_owner = db.persons().by_auth_id(self.auth_id)?;
        let (lender, ..) = db
            .lenders()
            .by_account_id(&account.id)?
            .first()
            .cloned()
            .ok_or_else(|| Error::msg("lender_not_found"))?;

        let state = AddExistingLeaseState {
            account,
            account_owner,
            lender,
        };

        let payload = add_existing_lease(state, input)?;

        db.transaction(|| {
            db.properties().create(&payload.property)?;
            db.leases().create(&payload.lease)?;
            db.rents().create_many(&payload.rents)?;
            db.persons().create_many(&payload.identities)?;
            db.tenants().create_many(&payload.tenants)?;
            db.discussions().create_many(&payload.discussions)?;
            Ok(())
        })?;

        Ok(payload)
    }
}

pub fn add_existing_lease(
    state: AddExistingLeaseState,
    input: AddExistingLeaseInput,
) -> Result<AddExistingLeasePayload> {
    let AddExistingLeaseState {
        account,
        account_owner,
        lender,
    } = state;

    // Add property.
    let property = CreateProperty::new(&account, &lender)
        .create_property(input.property)?
        .property;

    // Add lease.
    let CreateLeasePayload { lease } = leases::create_lease(
        CreateLeaseState {
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

    // Add rents.
    let rents = lease.rents();

    // Add tenant.
    let mut tenants = vec![];
    let mut identities = vec![];
    let mut discussions = vec![];

    for tenant_input in input.tenants {
        let CreateTenantPayload {
            mut tenant,
            tenant_identity,
            warrants: _warrants,
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

        tenants.push(tenant);
        identities.push(tenant_identity);
        discussions.push(discussion);
    }

    let discussions = discussions.into_iter().flatten().collect();

    Ok(AddExistingLeasePayload {
        lease,
        rents,
        property,
        tenants,
        identities,
        discussions,
    })
}
