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
use trankeel_core::database::StepStore;
use trankeel_core::database::TenantStore;
use trankeel_core::database::WarrantStore;
use trankeel_core::database::WorkflowStore;
use trankeel_core::dispatcher;
use trankeel_core::dispatcher::Event;
use trankeel_core::error::Error;
use trankeel_core::handlers::AdvertisementCreated;
use trankeel_core::handlers::AdvertisementUpdated;
use trankeel_core::handlers::CandidacyAccepted;
use trankeel_core::handlers::CandidacyCreated;
use trankeel_core::handlers::LeaseAffected;
use trankeel_core::handlers::LeaseCreated;
use trankeel_core::handlers::NoticeCreated;
use trankeel_core::handlers::PropertyCreated;
use trankeel_core::handlers::PropertyUpdated;
use trankeel_core::handlers::ReceiptCreated;
use trankeel_core::handlers::ReceiptSent;
use trankeel_core::handlers::TenantCreated;
use trankeel_core::handlers::TenantUpdated;
use trankeel_core::mailer::Mailer;
use trankeel_core::providers;
use trankeel_core::providers::Messagerie;
use trankeel_core::providers::Pdfmonkey;
use trankeel_core::providers::Pg;
use trankeel_core::providers::Sendinblue;
use trankeel_core::providers::Stripe;
use trankeel_core::templates::CandidacyCreatedMail;
use trankeel_core::templates::CandidacyRejectedMail;
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
use trankeel_data::MessageId;
use trankeel_data::Notice;
use trankeel_data::Person;
use trankeel_data::Property;
use trankeel_data::PropertyId;
use trankeel_data::Receipt;
use trankeel_data::Step;
use trankeel_data::Tenant;
use trankeel_data::TenantId;
use trankeel_ops::auth::CreateUserWithAccount;
use trankeel_ops::auth::CreateUserWithAccountInput;
use trankeel_ops::auth::CreateUserWithAccountPayload;
use trankeel_ops::auth::SignupUserFromInvite;
use trankeel_ops::auth::SignupUserFromInviteInput;
use trankeel_ops::auth::SignupUserFromInvitePayload;
use trankeel_ops::candidacies::AcceptCandidacy;
use trankeel_ops::candidacies::AcceptCandidacyInput;
use trankeel_ops::candidacies::CreateCandidacy;
use trankeel_ops::candidacies::CreateCandidacyInput;
use trankeel_ops::candidacies::CreateCandidacyPayload;
use trankeel_ops::error::Result;
use trankeel_ops::leases::CreateFurnishedLease;
use trankeel_ops::leases::CreateFurnishedLeaseInput;
use trankeel_ops::leases::CreateFurnishedLeasePayload;
use trankeel_ops::leases::CreateLease;
use trankeel_ops::leases::CreateLeaseInput;
use trankeel_ops::leases::CreateLeasePayload;
use trankeel_ops::leases::CreateNotices;
use trankeel_ops::leases::CreateNoticesInput;
use trankeel_ops::leases::CreateReceipts;
use trankeel_ops::leases::CreateReceiptsInput;
use trankeel_ops::leases::DeleteLease;
use trankeel_ops::leases::DeleteLeaseInput;
use trankeel_ops::leases::SendReceipts;
use trankeel_ops::leases::SendReceiptsInput;
use trankeel_ops::leases::UpdateFurnishedLease;
use trankeel_ops::leases::UpdateFurnishedLeaseInput;
use trankeel_ops::leases::UpdateFurnishedLeasePayload;
use trankeel_ops::lenders::UpdateIndividualLender;
use trankeel_ops::lenders::UpdateIndividualLenderInput;
use trankeel_ops::lenders::UpdateIndividualLenderPayload;
use trankeel_ops::messaging::push_message2::PushMessageCommand;
use trankeel_ops::messaging::DeleteDiscussionCommand;
use trankeel_ops::messaging::DeleteDiscussionInput;
use trankeel_ops::messaging::PushMessageInput;
use trankeel_ops::properties::CreateAdvertisement;
use trankeel_ops::properties::CreateAdvertisementInput;
use trankeel_ops::properties::CreateAdvertisementPayload;
use trankeel_ops::properties::CreateProperty;
use trankeel_ops::properties::CreatePropertyInput;
use trankeel_ops::properties::CreatePropertyPayload;
use trankeel_ops::properties::DeleteProperty;
use trankeel_ops::properties::DeletePropertyInput;
use trankeel_ops::properties::UpdateAdvertisement;
use trankeel_ops::properties::UpdateAdvertisementInput;
use trankeel_ops::properties::UpdateAdvertisementPayload;
use trankeel_ops::properties::UpdateProperty;
use trankeel_ops::properties::UpdatePropertyInput;
use trankeel_ops::properties::UpdatePropertyPayload;
use trankeel_ops::tenants::CreateTenant;
use trankeel_ops::tenants::CreateTenantInput;
use trankeel_ops::tenants::CreateTenantPayload;
use trankeel_ops::tenants::DeleteTenant;
use trankeel_ops::tenants::DeleteTenantInput;
use trankeel_ops::tenants::UpdateTenant;
use trankeel_ops::tenants::UpdateTenantInput;
use trankeel_ops::tenants::UpdateTenantPayload;
use trankeel_ops::workflows::CompleteStepCommand;
use trankeel_ops::workflows::CompleteStepInput;
use trankeel_ops::Command;

#[derive(Clone)]
pub struct Client(context::Context);

impl Client {
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

    pub fn steps(&self) -> Box<dyn StepStore + '_> {
        self.0.db().steps()
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
            Ok(())
        })?;

        dispatcher::dispatch(
            &self.0,
            vec![CandidacyCreated {
                candidacy: candidacy.clone(),
            }
            .into()],
        )
        .await?;

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
        let account = self.accounts().by_auth_id(auth_id)?;
        let account_owner = self.persons().by_auth_id(auth_id)?;
        let advertisement = self.advertisements().by_candidacy_id(&input.id)?;
        let candidacy = self.candidacies().by_id(&input.id)?;
        let candidacy_warrants = self.warrants().by_candidacy_id(&candidacy.id)?;
        let candidate = self.persons().by_candidacy_id(&candidacy.id)?;
        let discussion = self.discussions().by_candidacy_id(&candidacy.id)?;
        let other_candidacies = self
            .candidacies()
            .by_advertisement_id(&advertisement.id)?
            .into_iter()
            .filter(|candidacy| candidacy.id != input.id);
        let other_candidacies = other_candidacies
            .clone()
            .map(|candidacy| self.discussions().by_candidacy_id(&candidacy.id))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .zip(other_candidacies);
        let other_candidacies = other_candidacies
            .clone()
            .map(|(_, candidacy)| self.persons().by_candidacy_id(&candidacy.id))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|candidate| CandidacyRejectedMail::try_new(&candidate))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .zip(other_candidacies)
            .map(|(message, (discussion, candidacy))| (candidacy, discussion, message.to_string()))
            .collect::<Vec<_>>();

        dispatcher::dispatch(
            &self.0,
            vec![AcceptCandidacy::new(
                &candidacy,
                &account,
                &account_owner,
                &advertisement,
                &candidacy_warrants,
                &candidate,
                &discussion,
                &other_candidacies,
            )
            .run(input)
            .map(|payload| {
                CandidacyAccepted {
                    candidacy: payload.candidacy,
                    rejected_candidacies: payload.rejected_candidacies,
                    tenant: payload.tenant,
                    identity: payload.identity,
                    warrants: payload.warrants,
                    discussion: payload.discussion,
                    lease: payload.lease,
                    rents: payload.rents,
                    lease_file: payload.lease_file,
                    workflow: payload.workflow,
                    workflowable: payload.workflowable,
                    invite: payload.invite,
                }
                .into()
            })?],
        )
        .await?;

        self.candidacies().by_id(&candidacy.id)
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
        )
        .await?;

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
        )
        .await?;

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
        )
        .await?;

        Ok(property)
    }

    pub async fn update_property(
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
        )
        .await?;

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

    pub async fn create_advertisement(
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
        )
        .await?;

        Ok(advertisement)
    }

    pub async fn update_advertisement(
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
        )
        .await?;

        Ok(advertisement)
    }

    pub async fn create_lease(&self, auth_id: &AuthId, input: CreateLeaseInput) -> Result<Lease> {
        let account = self.0.db().accounts().by_auth_id(auth_id)?;
        let account_owner = self.0.db().persons().by_auth_id(auth_id)?;
        let (lender, ..) = self.0.db().lenders().by_account_id_first(&account.id)?;

        let CreateLeasePayload {
            lease,
            rents,
            property,
            tenants_with_identities,
        } = CreateLease::new(&account, &account_owner, &lender).run(input)?;

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
        )
        .await?;

        Ok(lease)
    }

    pub async fn create_furnished_lease(
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
        )
        .await?;

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
        _auth_id: &AuthId,
        input: CreateReceiptsInput,
    ) -> Result<Vec<Receipt>> {
        let rents = self.rents().by_id_many(&input.rent_ids)?;

        dispatcher::dispatch(
            &self.0,
            CreateReceipts::new(&rents).run(input).map(|payload| {
                payload
                    .receipts
                    .into_iter()
                    .map(|(receipt, rent, payment)| {
                        ReceiptCreated {
                            receipt,
                            rent,
                            payment,
                        }
                        .into()
                    })
                    .collect()
            })?,
        )
        .await?;

        Ok(vec![])
    }

    pub async fn send_receipts(&self, input: SendReceiptsInput) -> Result<Vec<Receipt>> {
        let rent_ids = SendReceipts.run(input)?;

        dispatcher::dispatch(
            &self.0,
            rent_ids
                .into_iter()
                .map(|rent_id| ReceiptSent { rent_id }.into())
                .collect(),
        )
        .await?;

        Ok(vec![])
    }

    pub async fn create_notices(
        &self,
        _auth_id: &AuthId,
        input: CreateNoticesInput,
    ) -> Result<Vec<Notice>> {
        let rents = self.rents().by_id_many(&input.rent_ids)?;

        dispatcher::dispatch(
            &self.0,
            CreateNotices::new(&rents).run(input).map(|payload| {
                payload
                    .notices
                    .into_iter()
                    .map(|(notice, rent)| NoticeCreated { notice, rent }.into())
                    .collect()
            })?,
        )
        .await?;

        Ok(vec![])
    }

    pub async fn delete_discussion(&self, input: DeleteDiscussionInput) -> Result<DiscussionId> {
        let discussion_id = input.id;

        dispatcher::dispatch(&self.0, DeleteDiscussionCommand.run(input)?)
            .await
            .map(|_| discussion_id)
    }

    pub async fn push_message(&self, input: PushMessageInput) -> Result<Message> {
        let message_id = MessageId::new();

        dispatcher::dispatch(&self.0, PushMessageCommand::new(&message_id).run(input)?)
            .await
            .and_then(|_| self.messages().by_id(&message_id))
    }

    pub async fn complete_step(&self, input: CompleteStepInput) -> Result<Step> {
        let step = self.steps().by_id(&input.id)?;

        dispatcher::dispatch(&self.0, CompleteStepCommand::new(&step).run(input)?)
            .await
            .and_then(|_| self.steps().by_id(&step.id))
    }

    pub async fn dispatch(&self, events: Vec<Event>) -> Result<()> {
        dispatcher::dispatch(&self.0, events).await
    }
}

pub fn init() -> Result<Client> {
    let pg = providers::Pg::init();
    let pdfmonkey = providers::Pdfmonkey::init();
    let sendinblue = providers::Sendinblue::init();
    let messagerie = providers::Messagerie::init(pg.clone());
    let stripe = providers::Stripe::init();

    Ok(Client::new(pg, pdfmonkey, sendinblue, messagerie, stripe))
}
