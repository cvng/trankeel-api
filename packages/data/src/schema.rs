#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    accounts (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        plan_id -> Nullable<Uuid>,
        status -> Accountstatus,
        stripe_customer_id -> Nullable<Text>,
        stripe_subscription_id -> Nullable<Text>,
        trial_end -> Nullable<Timestamptz>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    advertisements (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        published -> Bool,
        lease_type -> Leasetype,
        rent_amount -> Numeric,
        rent_charges_amount -> Nullable<Numeric>,
        deposit_amount -> Numeric,
        effect_date -> Timestamptz,
        flexibility -> Nullable<Entryflexibility>,
        referral_lease_id -> Nullable<Uuid>,
        property_id -> Uuid,
        title -> Text,
        description -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    candidacies (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        status -> Candidacystatus,
        advertisement_id -> Uuid,
        tenant_id -> Uuid,
        move_in_date -> Timestamptz,
        description -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    companies (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        address -> Nullable<Jsonb>,
        email -> Text,
        legal_entity -> Text,
        legal_entity_identifier -> Nullable<Text>,
        legal_entity_type -> Nullable<Legalentitytype>,
        legal_entity_type_other -> Nullable<Text>,
        phone_number -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    discussions (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        initiator_id -> Uuid,
        status -> Discussionstatus,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    events (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        #[sql_name = "type"]
        type_ -> Eventtype,
        payload -> Jsonb,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    files (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        download_url -> Nullable<Text>,
        external_id -> Nullable<Text>,
        filename -> Nullable<Text>,
        preview_url -> Nullable<Text>,
        status -> Nullable<Filestatus>,
        #[sql_name = "type"]
        type_ -> Filetype,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    invites (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Nullable<Uuid>,
        invitee_id -> Uuid,
        token -> Text,
        status -> Invitestatus,
        reason -> Invitereason,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    leases (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        deposit_amount -> Numeric,
        effect_date -> Timestamptz,
        signature_date -> Nullable<Timestamptz>,
        rent_amount -> Numeric,
        rent_charges_amount -> Nullable<Numeric>,
        #[sql_name = "type"]
        type_ -> Leasetype,
        lease_id -> Nullable<Uuid>,
        property_id -> Uuid,
        details -> Nullable<Jsonb>,
        expired_at -> Nullable<Timestamptz>,
        renew_date -> Nullable<Timestamptz>,
        duration -> Leaseduration,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    lenders (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        individual_id -> Nullable<Uuid>,
        company_id -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    messages (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        discussion_id -> Uuid,
        sender_id -> Uuid,
        content -> Nullable<Text>,
        event_id -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    payments (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        rent_id -> Uuid,
        amount -> Numeric,
        date -> Timestamptz,
        #[sql_name = "type"]
        type_ -> Transactiontype,
        label -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    persons (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        auth_id -> Nullable<Text>,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        address -> Nullable<Jsonb>,
        photo_url -> Nullable<Text>,
        role -> Personrole,
        phone_number -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    plans (id) {
        id -> Uuid,
        code -> Plancode,
        price -> Nullable<Numeric>,
        subtitle -> Nullable<Text>,
        title -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    professional_warrants (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        name -> Text,
        identifier -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    properties (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        address -> Jsonb,
        build_period -> Nullable<Propertybuildperiodtype>,
        building_legal_status -> Nullable<Propertybuildinglegalstatus>,
        common_spaces -> Nullable<Text>,
        energy_class -> Nullable<Propertyenergyclass>,
        equipments -> Nullable<Text>,
        gas_emission -> Nullable<Propertygasemission>,
        heating_method -> Nullable<Propertyusagetype>,
        housing_type -> Nullable<Propertyusagetype>,
        name -> Text,
        note -> Nullable<Text>,
        ntic_equipments -> Nullable<Text>,
        other_spaces -> Nullable<Text>,
        tax -> Nullable<Numeric>,
        room_count -> Propertyroomtype,
        status -> Propertystatus,
        surface -> Float4,
        tenant_private_spaces -> Nullable<Text>,
        usage_type -> Nullable<Propertyhabitationusagetype>,
        water_heating_method -> Nullable<Propertyusagetype>,
        lender_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    public_events (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        event_id -> Uuid,
        event_type -> Eventtype,
        eventable_type -> Eventabletype,
        eventable -> Jsonb,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    rents (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        period_end -> Timestamptz,
        period_start -> Timestamptz,
        amount -> Numeric,
        charges_amount -> Nullable<Numeric>,
        full_amount -> Numeric,
        status -> Rentstatus,
        lease_id -> Uuid,
        receipt_id -> Nullable<Uuid>,
        notice_id -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    steps (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        workflow_id -> Uuid,
        label -> Text,
        completed -> Bool,
        confirmation -> Nullable<Text>,
        requirements -> Nullable<Jsonb>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    tenants (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        person_id -> Uuid,
        apl -> Nullable<Bool>,
        birthdate -> Nullable<Date>,
        birthplace -> Nullable<Text>,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        note -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        status -> Tenantstatus,
        lease_id -> Nullable<Uuid>,
        is_student -> Nullable<Bool>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    warrants (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[sql_name = "type"]
        type_ -> Warranttype,
        tenant_id -> Uuid,
        individual_id -> Nullable<Uuid>,
        professional_id -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    workflowables (id) {
        id -> Uuid,
        candidacy_id -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    workflows (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        workflowable_id -> Uuid,
        #[sql_name = "type"]
        type_ -> Workflowtype,
        completed -> Bool,
    }
}

joinable!(accounts -> plans (plan_id));
joinable!(advertisements -> leases (referral_lease_id));
joinable!(advertisements -> properties (property_id));
joinable!(candidacies -> advertisements (advertisement_id));
joinable!(candidacies -> tenants (tenant_id));
joinable!(discussions -> accounts (account_id));
joinable!(discussions -> persons (initiator_id));
joinable!(files -> accounts (account_id));
joinable!(invites -> accounts (account_id));
joinable!(invites -> persons (invitee_id));
joinable!(leases -> accounts (account_id));
joinable!(leases -> files (lease_id));
joinable!(leases -> properties (property_id));
joinable!(lenders -> accounts (account_id));
joinable!(lenders -> companies (company_id));
joinable!(lenders -> persons (individual_id));
joinable!(messages -> discussions (discussion_id));
joinable!(messages -> events (event_id));
joinable!(messages -> persons (sender_id));
joinable!(payments -> rents (rent_id));
joinable!(persons -> accounts (account_id));
joinable!(properties -> accounts (account_id));
joinable!(properties -> lenders (lender_id));
joinable!(public_events -> accounts (account_id));
joinable!(public_events -> events (event_id));
joinable!(rents -> leases (lease_id));
joinable!(steps -> workflows (workflow_id));
joinable!(tenants -> accounts (account_id));
joinable!(tenants -> leases (lease_id));
joinable!(tenants -> persons (person_id));
joinable!(warrants -> persons (individual_id));
joinable!(warrants -> professional_warrants (professional_id));
joinable!(warrants -> tenants (tenant_id));
joinable!(workflowables -> candidacies (candidacy_id));
joinable!(workflows -> workflowables (workflowable_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    advertisements,
    candidacies,
    companies,
    discussions,
    events,
    files,
    invites,
    leases,
    lenders,
    messages,
    payments,
    persons,
    plans,
    professional_warrants,
    properties,
    public_events,
    rents,
    steps,
    tenants,
    warrants,
    workflowables,
    workflows,
);
