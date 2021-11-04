#![allow(dead_code)]

use crate::error::Result;
use crate::messaging;
use crate::messaging::CreateDiscussionState;
use crate::tenants;
use crate::tenants::CreateTenantState;
use crate::tenants::CreateWarrantInput;
use crate::tenants::CreateWarrantPayload;
use crate::tenants::CreateWarrantState;
use crate::CreateDiscussionInput;
use crate::CreateTenantInput;
use trankeel_data::DiscussionWithMessages;
use trankeel_data::TenantWithProfile;
use trankeel_data::WarrantWithIdentity;

type Step = dyn Fn(
    &CreateTenantState,
    &CreateTenantInput,
    CreateTenantPayload,
) -> Result<CreateTenantPayload>;

#[derive(Default)]
struct CreateTenantPayload {
    pub tenant: Option<TenantWithProfile>,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
    pub discussion: Option<DiscussionWithMessages>,
}

pub(crate) struct CreateTenant;

impl CreateTenant {
    fn run(state: CreateTenantState, input: CreateTenantInput) -> Result<CreateTenantPayload> {
        let payload = CreateTenantPayload::default();

        let steps: Vec<&Step> = vec![
            //
            &create_tenant,
            //
            &create_warrants,
            //
            &create_discussion,
        ];

        let payload = steps
            .into_iter()
            .try_fold(payload, |payload, step| step(&state, &input, payload))?;

        Ok(payload)
    }
}

fn create_tenant(
    state: &CreateTenantState,
    input: &CreateTenantInput,
    mut payload: CreateTenantPayload,
) -> Result<CreateTenantPayload> {
    let tenant = tenants::create_tenant(input.clone(), state.clone())?.tenant;

    payload.tenant = Some(tenant);

    Ok(payload)
}

fn create_warrants(
    state: &CreateTenantState,
    input: &CreateTenantInput,
    mut payload: CreateTenantPayload,
) -> Result<CreateTenantPayload> {
    let tenant = payload.tenant.clone().unwrap();

    if let Some(warrants_input) = input.warrants.clone() {
        let warrants = warrants_input
            .into_iter()
            .map(|input| {
                tenants::create_warrant(
                    CreateWarrantInput { ..input },
                    CreateWarrantState {
                        account: state.account.clone(),
                        tenant: tenant.0.clone(),
                    },
                )
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|CreateWarrantPayload { warrant }| warrant)
            .collect();

        payload.warrants = Some(warrants);
    }

    Ok(payload)
}

fn create_discussion(
    state: &CreateTenantState,
    _input: &CreateTenantInput,
    mut payload: CreateTenantPayload,
) -> Result<CreateTenantPayload> {
    let tenant = payload.tenant.clone().unwrap();

    let discussion = messaging::create_discussion(
        CreateDiscussionInput {
            initiator_id: tenant.1.id,
            recipient_id: state.account_owner.id,
            message: None,
        },
        CreateDiscussionState {
            account: state.account.clone(),
        },
    )?
    .discussion;

    payload.discussion = Some(discussion);

    Ok(payload)
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_create_tenant() {
        // let state = CreateTenantState::default();
        // let input = CreateTenantInput::default();
        // assert!(CreateTenant::run(state, input).is_ok());
    }
}
