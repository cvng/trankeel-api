table! {
    accounts (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        plan_id -> Nullable<Uuid>,
        status -> Nullable<Text>,
        stripe_customer_id -> Nullable<Text>,
        stripe_subscription_id -> Nullable<Text>,
        trial_end -> Nullable<Timestamptz>,
    }
}

table! {
    companies (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        address -> Nullable<Jsonb>,
        email -> Text,
        legal_entity -> Text,
        legal_entity_identifier -> Nullable<Text>,
        legal_entity_type -> Nullable<Text>,
        legal_entity_type_other -> Nullable<Text>,
        phone_number -> Nullable<Text>,
    }
}

table! {
    events (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        eventable_id -> Uuid,
        eventable_type -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

table! {
    files (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        download_url -> Nullable<Text>,
        external_id -> Nullable<Text>,
        filename -> Nullable<Text>,
        preview_url -> Nullable<Text>,
        status -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

table! {
    leases (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        deposit_amount -> Nullable<Numeric>,
        effect_date -> Timestamptz,
        signature_date -> Nullable<Timestamptz>,
        rent_amount -> Numeric,
        rent_charges_amount -> Nullable<Numeric>,
        #[sql_name = "type"]
        type_ -> Text,
        lease_id -> Nullable<Uuid>,
        property_id -> Uuid,
        details -> Nullable<Jsonb>,
        expired_at -> Nullable<Timestamptz>,
        renew_date -> Nullable<Timestamptz>,
        duration -> Text,
    }
}

table! {
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
    payments (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        rent_id -> Uuid,
        amount -> Numeric,
        date -> Timestamptz,
        #[sql_name = "type"]
        type_ -> Text,
        label -> Nullable<Text>,
    }
}

table! {
    persons (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        auth_id -> Text,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        address -> Nullable<Jsonb>,
        photo_url -> Nullable<Text>,
        role -> Nullable<Text>,
        phone_number -> Nullable<Text>,
    }
}

table! {
    plans (id) {
        id -> Uuid,
        code -> Text,
        price -> Nullable<Numeric>,
        subtitle -> Nullable<Text>,
        title -> Nullable<Text>,
    }
}

table! {
    properties (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        address -> Jsonb,
        build_period -> Nullable<Text>,
        building_legal_status -> Nullable<Text>,
        common_spaces -> Nullable<Text>,
        energy_class -> Nullable<Text>,
        equipments -> Nullable<Text>,
        gas_emission -> Nullable<Text>,
        heating_method -> Nullable<Text>,
        housing_type -> Nullable<Text>,
        name -> Text,
        note -> Nullable<Text>,
        description -> Nullable<Text>,
        ntic_equipments -> Nullable<Text>,
        other_spaces -> Nullable<Text>,
        tax -> Nullable<Numeric>,
        room_count -> Text,
        status -> Nullable<Text>,
        surface -> Float4,
        tenant_private_spaces -> Nullable<Text>,
        usage_type -> Nullable<Text>,
        water_heating_method -> Nullable<Text>,
        lender_id -> Uuid,
    }
}

table! {
    rents (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        period_end -> Timestamptz,
        period_start -> Timestamptz,
        amount -> Numeric,
        charges_amount -> Nullable<Numeric>,
        full_amount -> Numeric,
        status -> Text,
        lease_id -> Uuid,
        receipt_id -> Nullable<Uuid>,
        notice_id -> Nullable<Uuid>,
    }
}

table! {
    tenants (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        account_id -> Uuid,
        apl -> Nullable<Bool>,
        birthdate -> Date,
        birthplace -> Nullable<Text>,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        note -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        lease_id -> Nullable<Uuid>,
        is_student -> Nullable<Bool>,
    }
}

joinable!(accounts -> plans (plan_id));
joinable!(events -> accounts (account_id));
joinable!(leases -> accounts (account_id));
joinable!(leases -> files (lease_id));
joinable!(leases -> properties (property_id));
joinable!(lenders -> accounts (account_id));
joinable!(lenders -> companies (company_id));
joinable!(lenders -> persons (individual_id));
joinable!(payments -> rents (rent_id));
joinable!(persons -> accounts (account_id));
joinable!(properties -> accounts (account_id));
joinable!(properties -> lenders (lender_id));
joinable!(rents -> leases (lease_id));
joinable!(tenants -> accounts (account_id));
joinable!(tenants -> leases (lease_id));

allow_tables_to_appear_in_same_query!(
    accounts, companies, events, files, leases, lenders, payments, persons, plans, properties,
    rents, tenants,
);
