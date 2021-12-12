use crate::auth::CreateUserWithAccount;
use crate::auth::CreateUserWithAccountInput;
use crate::auth::CreateUserWithAccountPayload;
use crate::auth::SignupUserFromInvite;
use crate::auth::SignupUserFromInviteInput;
use crate::auth::SignupUserFromInvitePayload;
use crate::candidacies::AcceptCandidacy;
use crate::candidacies::AcceptCandidacyInput;
use crate::candidacies::AcceptCandidacyPayload;
use crate::candidacies::CreateCandidacy;
use crate::candidacies::CreateCandidacyInput;
use crate::candidacies::CreateCandidacyPayload;
use crate::error::Result;
use crate::leases::send_receipts;
use crate::leases::AddExistingLease;
use crate::leases::AddExistingLeaseInput;
use crate::leases::AddExistingLeasePayload;
use crate::leases::CreateFurnishedLease;
use crate::leases::CreateFurnishedLeaseInput;
use crate::leases::CreateFurnishedLeasePayload;
use crate::leases::CreateNoticesInput;
use crate::leases::CreateReceiptsInput;
use crate::leases::DeleteLease;
use crate::leases::DeleteLeaseInput;
use crate::leases::SendReceiptsInput;
use crate::leases::UpdateFurnishedLease;
use crate::leases::UpdateFurnishedLeaseInput;
use crate::leases::UpdateFurnishedLeasePayload;
use crate::lenders::UpdateIndividualLender;
use crate::lenders::UpdateIndividualLenderInput;
use crate::lenders::UpdateIndividualLenderPayload;
use crate::messaging::DeleteDiscussion;
use crate::messaging::DeleteDiscussionInput;
use crate::messaging::PushMessage;
use crate::messaging::PushMessageInput;
use crate::messaging::PushMessagePayload;
use crate::properties::CreateAdvertisement;
use crate::properties::CreateAdvertisementInput;
use crate::properties::CreateAdvertisementPayload;
use crate::properties::CreateProperty;
use crate::properties::CreatePropertyInput;
use crate::properties::CreatePropertyPayload;
use crate::properties::DeleteProperty;
use crate::properties::DeletePropertyInput;
use crate::properties::UpdateAdvertisement;
use crate::properties::UpdateAdvertisementInput;
use crate::properties::UpdateAdvertisementPayload;
use crate::properties::UpdateProperty;
use crate::properties::UpdatePropertyInput;
use crate::properties::UpdatePropertyPayload;
use crate::tenants::CreateTenant;
use crate::tenants::CreateTenantInput;
use crate::tenants::CreateTenantPayload;
use crate::tenants::DeleteTenant;
use crate::tenants::DeleteTenantInput;
use crate::tenants::UpdateTenant;
use crate::tenants::UpdateTenantInput;
use crate::tenants::UpdateTenantPayload;
use crate::workflows::CompleteStep;
use crate::workflows::CompleteStepInput;
use crate::workflows::CompleteStepPayload;
use log::info;
use trankeel_core::billing::BillingProvider;
use trankeel_core::context;
use trankeel_core::database::AccountStore;
use trankeel_core::database::AdvertisementStore;
use trankeel_core::database::BalanceStore;
use trankeel_core::database::CandidacyStore;
use trankeel_core::database::CompanyStore;
use trankeel_core::database::Db;
use trankeel_core::database::DiscussionStore;
use trankeel_core::database::EventStore;
use trankeel_core::database::FileStore;
use trankeel_core::database::LeaseStore;
use trankeel_core::database::LenderStore;
use trankeel_core::database::MessageStore;
use trankeel_core::database::PaymentStore;
use trankeel_core::database::PersonStore;
use trankeel_core::database::PlanStore;
use trankeel_core::database::PropertyStore;
use trankeel_core::database::RentStore;
use trankeel_core::database::ReportStore;
use trankeel_core::database::TenantStore;
use trankeel_core::database::WarrantStore;
use trankeel_core::database::WorkflowStore;
use trankeel_core::dispatcher;
use trankeel_core::dispatcher::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::error::Error;
use trankeel_core::handlers::AdvertisementCreated;
use trankeel_core::handlers::AdvertisementUpdated;
use trankeel_core::handlers::LeaseAffected;
use trankeel_core::handlers::LeaseCreated;
use trankeel_core::handlers::PropertyCreated;
use trankeel_core::handlers::PropertyUpdated;
use trankeel_core::handlers::TenantCreated;
use trankeel_core::handlers::TenantUpdated;
use trankeel_core::mailer::IntoMail;
use trankeel_core::mailer::Mail;
use trankeel_core::mailer::Mailer;
use trankeel_core::providers;
use trankeel_core::providers::Messagerie;
use trankeel_core::providers::Pdfmonkey;
use trankeel_core::providers::Pg;
use trankeel_core::providers::Sendinblue;
use trankeel_core::providers::Stripe;
use trankeel_core::templates::CandidacyCreatedMail;
use trankeel_data::Account;
use trankeel_data::Advertisement;
use trankeel_data::AuthId;
use trankeel_data::Candidacy;
use trankeel_data::DiscussionId;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use trankeel_data::LegalIdentity;
use trankeel_data::Lender;
use trankeel_data::Message;
use trankeel_data::Notice;
use trankeel_data::Person;
use trankeel_data::Property;
use trankeel_data::PropertyId;
use trankeel_data::Receipt;
use trankeel_data::Step;
use trankeel_data::Tenant;
use trankeel_data::TenantId;

pub struct Client(context::Context);

impl<'a> Client {
    pub fn new(
        pg: Pg,
        pdfmonkey: Pdfmonkey,
        sendinblue: Sendinblue,
        messagerie: Messagerie,
        stripe: Stripe,
    ) -> Self {
        Self(context::Context::new(
            pg, pdfmonkey, sendinblue, messagerie, stripe,
        ))
    }

    // Stores

    pub fn accounts(&self) -> Box<dyn AccountStore + '_> {
        self.0.db().accounts()
    }

    pub fn balances(&self) -> Box<dyn BalanceStore + '_> {
        self.0.db().balances()
    }

    pub fn persons(&self) -> Box<dyn PersonStore + '_> {
        self.0.db().persons()
    }

    pub fn companies(&self) -> Box<dyn CompanyStore + '_> {
        self.0.db().companies()
    }

    pub fn lenders(&self) -> Box<dyn LenderStore + '_> {
        self.0.db().lenders()
    }

    pub fn tenants(&self) -> Box<dyn TenantStore + '_> {
        self.0.db().tenants()
    }

    pub fn warrants(&self) -> Box<dyn WarrantStore + '_> {
        self.0.db().warrants()
    }

    pub fn advertisements(&self) -> Box<dyn AdvertisementStore + '_> {
        self.0.db().advertisements()
    }

    pub fn candidacies(&self) -> Box<dyn CandidacyStore + '_> {
        self.0.db().candidacies()
    }

    pub fn properties(&self) -> Box<dyn PropertyStore + '_> {
        self.0.db().properties()
    }

    pub fn leases(&self) -> Box<dyn LeaseStore + '_> {
        self.0.db().leases()
    }

    pub fn rents(&self) -> Box<dyn RentStore + '_> {
        self.0.db().rents()
    }

    pub fn files(&self) -> Box<dyn FileStore + '_> {
        self.0.db().files()
    }

    pub fn payments(&self) -> Box<dyn PaymentStore + '_> {
        self.0.db().payments()
    }

    pub fn plans(&self) -> Box<dyn PlanStore + '_> {
        self.0.db().plans()
    }

    pub fn events(&self) -> Box<dyn EventStore + '_> {
        self.0.db().events()
    }

    pub fn reports(&self) -> Box<dyn ReportStore + '_> {
        self.0.db().reports()
    }

    pub fn discussions(&self) -> Box<dyn DiscussionStore + '_> {
        self.0.db().discussions()
    }

    pub fn messages(&self) -> Box<dyn MessageStore + '_> {
        self.0.db().messages()
    }

    pub fn workflows(&self) -> Box<dyn WorkflowStore + '_> {
        self.0.db().workflows()
    }

    // Operations

    pub async fn create_user_with_account(
        &self,
        input: CreateUserWithAccountInput,
    ) -> Result<CreateUserWithAccountPayload> {
        let skip_create_customer = matches!(input.skip_create_customer, Some(true));

        let CreateUserWithAccountPayload {
            user,
            lender,
            account,
        } = CreateUserWithAccount::new().run(input)?;

        self.0.db().transaction(|| {
            self.0.db().accounts().create(&account)?;
            self.0.db().persons().create(&user)?;
            self.0.db().lenders().create(&lender)?;
            Ok(())
        })?;

        if !skip_create_customer {
            // Create subscription.
            let subscription = self
                .0
                .billing_provider()
                .create_subscription_with_customer(user.email.clone())
                .await?;
            info!(
                "Created subscription {} for account {}",
                subscription.id, account.id
            );

            // Update the local customer data.
            self.0.db().accounts().update(&Account {
                id: account.id,
                stripe_customer_id: Some(subscription.customer_id.clone()),
                stripe_subscription_id: Some(subscription.id.clone()),
                status: subscription.status,
                trial_end: subscription.trial_end,
                ..account
            })?;
        }

        Ok(CreateUserWithAccountPayload {
            user,
            lender,
            account,
        })
    }

    pub async fn signup_user_from_invite(
        &self,
        input: SignupUserFromInviteInput,
    ) -> Result<Person> {
        let invite = self.0.db().invites().by_token(&input.invite_token)?;
        let invitee = self.0.db().persons().by_id(&invite.invitee_id)?;

        let SignupUserFromInvitePayload {
            invite,
            invitee,
            account,
        } = SignupUserFromInvite::new(&invite, &invitee).run(input)?;

        self.0.db().transaction(|| {
            self.0.db().accounts().create(&account)?;
            self.0.db().persons().update(&invitee)?;
            self.0.db().invites().update(&invite)?;
            Ok(())
        })?;

        Ok(invitee)
    }

    pub async fn create_candidacy(&self, input: CreateCandidacyInput) -> Result<Candidacy> {
        let account = self
            .0
            .db()
            .accounts()
            .by_advertisement_id(&input.advertisement_id)?;
        let account_owner = self.0.db().persons().by_account_id_first(&account.id)?;

        let CreateCandidacyPayload {
            candidacy,
            candidate,
            warrants,
            discussion,
            messages,
        } = CreateCandidacy::new(&account, &account_owner).run(input)?;

        self.0.db().transaction(|| {
            self.0.db().persons().create(&candidate)?;
            self.0.db().candidacies().create(&candidacy)?;
            if let Some(warrants) = &warrants {
                self.0.db().warrants().create_many(warrants)?;
            }
            self.0.db().discussions().create(&discussion)?;
            self.0.db().messages().create_many(&messages)?;
            dispatcher::dispatch(&self.0, vec![Event::CandidacyCreated(candidacy.clone())])?;
            Ok(())
        })?;

        self.0
            .mailer()
            .batch(vec![CandidacyCreatedMail::try_new(&candidacy, &candidate)?])
            .await?;

        Ok(candidacy)
    }

    pub async fn accept_candidacy(
        &self,
        auth_id: &AuthId,
        input: AcceptCandidacyInput,
    ) -> Result<Candidacy> {
        let AcceptCandidacyPayload { candidacy } =
            AcceptCandidacy::new(&self.0, auth_id).run(input).await?;

        Ok(candidacy)
    }

    pub async fn create_tenant(
        &self,
        auth_id: &AuthId,
        input: CreateTenantInput,
    ) -> Result<Tenant> {
        let account = self.0.db().accounts().by_auth_id(auth_id)?;
        let account_owner = self.0.db().persons().by_auth_id(auth_id)?;

        let CreateTenantPayload {
            tenant,
            identity,
            warrants,
            discussion,
        } = CreateTenant::new(&account, &account_owner, None).run(input)?;

        dispatcher::dispatch(
            &self.0,
            vec![TenantCreated {
                tenant: tenant.clone(),
                identity,
                warrants,
                discussion,
            }
            .into()],
        )?;

        Ok(tenant)
    }

    pub async fn update_tenant(
        &self,
        _auth_id: &AuthId,
        input: UpdateTenantInput,
    ) -> Result<Tenant> {
        let tenant = self.0.db().tenants().by_id(&input.id)?;

        let UpdateTenantPayload { tenant } = UpdateTenant::new(&tenant).run(input)?;

        dispatcher::dispatch(
            &self.0,
            vec![TenantUpdated {
                tenant: tenant.clone(),
            }
            .into()],
        )?;

        Ok(tenant)
    }

    pub fn delete_tenant(&self, _auth_id: &AuthId, input: DeleteTenantInput) -> Result<TenantId> {
        let tenant_id = DeleteTenant.run(input)?;

        self.0.db().tenants().delete(&tenant_id)?;

        Ok(tenant_id)
    }

    pub async fn create_property(
        &self,
        auth_id: &AuthId,
        input: CreatePropertyInput,
    ) -> Result<Property> {
        let account = self.0.db().accounts().by_auth_id(auth_id)?;
        let (lender, ..) = self
            .0
            .db()
            .lenders()
            .by_account_id(&account.id)?
            .first()
            .cloned()
            .ok_or_else(|| Error::msg("lender_not_found"))?;

        let CreatePropertyPayload { property } =
            CreateProperty::new(&account, &lender).run(input)?;

        dispatcher::dispatch(
            &self.0,
            vec![PropertyCreated {
                property: property.clone(),
            }
            .into()],
        )?;

        Ok(property)
    }

    pub fn update_property(
        &self,
        _auth_id: &AuthId,
        input: UpdatePropertyInput,
    ) -> Result<Property> {
        let property = self.0.db().properties().by_id(&input.id)?;

        let UpdatePropertyPayload { property } = UpdateProperty::new(&property).run(input)?;

        dispatcher::dispatch(
            &self.0,
            vec![PropertyUpdated {
                property: property.clone(),
            }
            .into()],
        )?;

        Ok(property)
    }

    pub fn delete_property(
        &self,
        _auth_id: &AuthId,
        input: DeletePropertyInput,
    ) -> Result<PropertyId> {
        let property_id = DeleteProperty.run(input)?;

        self.0.db().properties().delete(&property_id)?;

        Ok(property_id)
    }

    pub fn create_advertisement(
        &self,
        _auth_id: &AuthId,
        input: CreateAdvertisementInput,
    ) -> Result<Advertisement> {
        let CreateAdvertisementPayload { advertisement } = CreateAdvertisement.run(input)?;

        dispatcher::dispatch(
            &self.0,
            vec![AdvertisementCreated {
                advertisement: advertisement.clone(),
            }
            .into()],
        )?;

        Ok(advertisement)
    }

    pub fn update_advertisement(
        &self,
        _auth_id: &AuthId,
        input: UpdateAdvertisementInput,
    ) -> Result<Advertisement> {
        let advertisement = self.0.db().advertisements().by_id(&input.id)?;

        let UpdateAdvertisementPayload { advertisement } =
            UpdateAdvertisement::new(&advertisement).run(input)?;

        dispatcher::dispatch(
            &self.0,
            vec![AdvertisementUpdated {
                advertisement: advertisement.clone(),
            }
            .into()],
        )?;

        Ok(advertisement)
    }

    pub async fn add_existing_lease(
        &self,
        auth_id: &AuthId,
        input: AddExistingLeaseInput,
    ) -> Result<Lease> {
        let account = self.0.db().accounts().by_auth_id(auth_id)?;
        let account_owner = self.0.db().persons().by_auth_id(auth_id)?;
        let (lender, ..) = self.0.db().lenders().by_account_id_first(&account.id)?;

        let AddExistingLeasePayload {
            lease,
            rents,
            property,
            tenants_with_identities,
        } = AddExistingLease::new(&account, &account_owner, &lender).run(input)?;

        dispatcher::dispatch(
            &self.0,
            vec![
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
                    .into_iter()
                    .map(|(tenant, ..)| LeaseAffected { tenant }.into())
                    .collect::<Vec<_>>(),
            )
            .collect(),
        )?;

        Ok(lease)
    }

    pub fn create_furnished_lease(
        &self,
        auth_id: &AuthId,
        input: CreateFurnishedLeaseInput,
    ) -> Result<Lease> {
        let account = self.0.db().accounts().by_auth_id(auth_id)?;
        let tenants = input
            .tenant_ids
            .iter()
            .map(|&tenant_id| self.0.db().tenants().by_id(&tenant_id))
            .collect::<Result<Vec<_>>>()?;

        let CreateFurnishedLeasePayload {
            lease,
            rents,
            tenants,
        } = CreateFurnishedLease::new(&account, &tenants).run(input)?;

        dispatcher::dispatch(
            &self.0,
            vec![LeaseCreated {
                lease: lease.clone(),
                rents,
            }
            .into()]
            .into_iter()
            .chain(
                tenants
                    .clone()
                    .into_iter()
                    .map(|tenant| TenantUpdated { tenant }.into())
                    .collect::<Vec<_>>(),
            )
            .chain(
                tenants
                    .into_iter()
                    .map(|tenant| LeaseAffected { tenant }.into())
                    .collect::<Vec<_>>(),
            )
            .collect(),
        )?;

        Ok(lease)
    }

    pub fn update_furnished_lease(
        &self,
        _auth_id: &AuthId,
        input: UpdateFurnishedLeaseInput,
    ) -> Result<Lease> {
        let lease = self.0.db().leases().by_id(&input.id)?;

        let UpdateFurnishedLeasePayload { lease } = UpdateFurnishedLease::new(&lease).run(input)?;

        self.0.db().leases().update(&lease)?;

        Ok(lease)
    }

    pub fn delete_lease(&self, _auth_id: &AuthId, input: DeleteLeaseInput) -> Result<LeaseId> {
        let lease_id = DeleteLease.run(input)?;

        self.0.db().leases().delete(&lease_id)?;

        Ok(lease_id)
    }

    pub fn update_individual_lender(
        &self,
        _auth_id: &AuthId,
        input: UpdateIndividualLenderInput,
    ) -> Result<Lender> {
        let (lender, identity) = self.0.db().lenders().by_id(&input.id)?;

        let UpdateIndividualLenderPayload {
            lender: (lender, identity),
        } = UpdateIndividualLender::new(&(lender, identity)).run(input)?;

        match identity {
            LegalIdentity::Individual(person) => self.0.db().persons().update(&person)?,
            _ => return Err(Error::msg("lender is not an individual")),
        };

        Ok(lender)
    }

    pub async fn create_receipts(
        &self,
        auth_id: &AuthId,
        input: CreateReceiptsInput,
    ) -> Result<Vec<Receipt>> {
        crate::leases::create_receipts(&self.0, auth_id, input).await
    }

    pub async fn send_receipts(&self, input: SendReceiptsInput) -> Result<Vec<Receipt>> {
        dispatcher::dispatch_async(&self.0, send_receipts(input)?).await?;

        Ok(vec![])
    }

    pub async fn create_notices(
        &self,
        auth_id: &AuthId,
        input: CreateNoticesInput,
    ) -> Result<Vec<Notice>> {
        crate::leases::create_notices(&self.0, auth_id, input).await
    }

    pub fn delete_discussion(
        &self,
        _auth_id: &AuthId,
        input: DeleteDiscussionInput,
    ) -> Result<DiscussionId> {
        let discussion_id = DeleteDiscussion.run(input)?;

        self.0.db().discussions().delete(&discussion_id)?;

        Ok(discussion_id)
    }

    pub async fn push_message(&self, input: PushMessageInput) -> Result<Message> {
        let discussion = self.0.db().discussions().by_id(&input.discussion_id)?;

        let PushMessagePayload {
            message,
            discussion,
        } = PushMessage::new(&discussion).run(input)?;

        self.0.db().transaction(|| {
            self.0.db().messages().create(&message)?;
            self.0.db().discussions().update(&discussion)?;
            Ok(())
        })?;

        Ok(message)
    }

    pub fn complete_step(&self, input: CompleteStepInput) -> Result<Step> {
        let step = self.0.db().steps().by_id(&input.id)?;

        let CompleteStepPayload { step } = CompleteStep::new(&step).run(input)?;

        dispatcher::dispatch(&self.0, vec![Event::StepCompleted(step.clone())])?;

        Ok(step)
    }

    pub async fn batch_mails(&self, mails: Vec<impl IntoMail>) -> Result<Vec<Mail>> {
        self.0.mailer().batch(mails).await
    }
}

pub fn init() -> Result<Client> {
    Ok(Client::new(
        providers::Pg::init(),
        providers::Pdfmonkey::init(),
        providers::Sendinblue::init(),
        providers::Messagerie::init(),
        providers::Stripe::init(),
    ))
}
