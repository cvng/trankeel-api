table! {
    account (id) {
        #[sql_name = "planId"]
        plan_id -> Nullable<Uuid>,
        status -> Nullable<Text>,
        #[sql_name = "stripeCustomerId"]
        stripe_customer_id -> Nullable<Text>,
        #[sql_name = "stripeSubscriptionId"]
        stripe_subscription_id -> Nullable<Text>,
        #[sql_name = "trialEnd"]
        trial_end -> Nullable<Timestamp>,
        #[sql_name = "ownerId"]
        owner_id -> Text,
        id -> Uuid,
    }
}

table! {
    company (id) {
        address -> Nullable<Jsonb>,
        email -> Text,
        legalEntity -> Text,
        legalEntityIdentifier -> Nullable<Text>,
        legalEntityType -> Nullable<Text>,
        legalEntityTypeOther -> Nullable<Text>,
        phoneNumber -> Nullable<Text>,
        id -> Uuid,
    }
}

table! {
    file (id) {
        createdAt -> Nullable<Timestamp>,
        downloadUrl -> Nullable<Text>,
        externalId -> Nullable<Text>,
        filename -> Nullable<Text>,
        previewUrl -> Nullable<Text>,
        status -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        updatedAt -> Nullable<Timestamp>,
        id -> Uuid,
    }
}

table! {
    lease (id) {
        #[sql_name = "accountId"]
        account_id -> Uuid,
        #[sql_name = "depositAmount"]
        deposit_amount -> Nullable<Numeric>,
        #[sql_name = "effectDate"]
        effect_date -> Timestamp,
        #[sql_name = "signatureDate"]
        signature_date -> Nullable<Timestamp>,
        #[sql_name = "rentAmount"]
        rent_amount -> Numeric,
        #[sql_name = "rentChargesAmount"]
        rent_charges_amount -> Nullable<Numeric>,
        #[sql_name = "type"]
        type_ -> Text,
        #[sql_name = "leaseId"]
        lease_id -> Nullable<Uuid>,
        #[sql_name = "propertyId"]
        property_id -> Uuid,
        id -> Uuid,
        #[sql_name = "data"]
        details -> Nullable<Jsonb>,
        #[sql_name = "expiredAt"]
        expired_at -> Nullable<Timestamp>,
        #[sql_name = "renewDate"]
        renew_date -> Nullable<Timestamp>,
    }
}

table! {
    leasetenant (lease_id, tenant_id) {
        #[sql_name = "leaseId"]
        lease_id -> Uuid,
        #[sql_name = "tenantId"]
        tenant_id -> Uuid,
    }
}

table! {
    lender (id) {
        id -> Uuid,
        #[sql_name="accountId"]
        account_id -> Uuid,
        #[sql_name="individualId"]
        individual_id -> Nullable<Uuid>,
        #[sql_name="companyId"]
        company_id -> Nullable<Uuid>,
    }
}

table! {
    plan (id) {
        code -> Text,
        price -> Nullable<Numeric>,
        subtitle -> Nullable<Text>,
        title -> Nullable<Text>,
        id -> Uuid,
    }
}

table! {
    property (id) {
        #[sql_name="accountId"]
        account_id -> Nullable<Uuid>,
        address -> Jsonb,
        #[sql_name="buildPeriod"]
        build_period -> Nullable<Text>,
        #[sql_name="buildingLegalStatus"]
        building_legal_status -> Nullable<Text>,
        #[sql_name="commonSpaces"]
        common_spaces -> Nullable<Text>,
        #[sql_name="energyClass"]
        energy_class -> Nullable<Text>,
        equipments -> Nullable<Text>,
        #[sql_name="gasEmission"]
        gas_emission -> Nullable<Text>,
        #[sql_name="heatingMethod"]
        heating_method -> Nullable<Text>,
        #[sql_name="housingType"]
        housing_type -> Nullable<Text>,
        name -> Text,
        note -> Nullable<Text>,
        #[sql_name="nticEquipments"]
        ntic_equipments -> Nullable<Text>,
        #[sql_name="otherSpaces"]
        other_spaces -> Nullable<Text>,
        tax -> Nullable<Numeric>,
        #[sql_name="roomCount"]
        room_count -> Text,
        status -> Nullable<Text>,
        surface -> Float8,
        #[sql_name="tenantPrivateSpaces"]
        tenant_private_spaces -> Nullable<Text>,
        #[sql_name="usageType"]
        usage_type -> Nullable<Text>,
        #[sql_name="waterHeatingMethod"]
        water_heating_method -> Nullable<Text>,
        id -> Uuid,
        #[sql_name="lenderId"]
        lender_id -> Uuid,
    }
}

table! {
    rent (id) {
        id -> Uuid,
        periodEnd -> Timestamp,
        periodStart -> Timestamp,
        amount -> Numeric,
        chargesAmount -> Nullable<Numeric>,
        fullAmount -> Numeric,
        status -> Text,
        accountId -> Uuid,
        leaseId -> Uuid,
        receiptId -> Nullable<Uuid>,
        transactionId -> Nullable<Uuid>,
        noticeId -> Nullable<Uuid>,
    }
}

table! {
    tenant (id) {
        #[sql_name="accountId"]
        account_id -> Uuid,
        apl -> Bool,
        #[sql_name="authId"]
        auth_id -> Nullable<Text>,
        birthdate -> Timestamp,
        birthplace -> Nullable<Text>,
        email -> Text,
        #[sql_name="firstName"]
        first_name -> Text,
        #[sql_name="lastName"]
        last_name -> Text,
        note -> Nullable<Text>,
        #[sql_name="phoneNumber"]
        phone_number -> Nullable<Text>,
        role -> Nullable<Text>,
        id -> Uuid,
        #[sql_name="leaseId"]
        lease_id -> Nullable<Uuid>,
        #[sql_name="visaleId"]
        visale_id -> Nullable<Text>,
    }
}

table! {
    transaction (id) {
        accountId -> Nullable<Uuid>,
        amount -> Numeric,
        leaseId -> Nullable<Uuid>,
        date -> Timestamp,
        label -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        id -> Uuid,
    }
}

table! {
    user (id) {
        #[sql_name = "authId"]
        auth_id -> Text,
        email -> Text,
        #[sql_name = "firstName"]
        first_name -> Nullable<Text>,
        #[sql_name = "lastName"]
        last_name -> Nullable<Text>,
        address -> Nullable<Jsonb>,
        #[sql_name = "photoURL"]
        photo_url -> Nullable<Text>,
        role -> Nullable<Text>,
        id -> Uuid,
        #[sql_name = "phoneNumber"]
        phone_number -> Nullable<Text>,
        #[sql_name = "accountId"]
        account_id -> Nullable<Uuid>,
    }
}

joinable!(account -> plan (plan_id));
joinable!(lease -> account (account_id));
joinable!(lease -> file (lease_id));
joinable!(lease -> property (property_id));
joinable!(leasetenant -> lease (lease_id));
joinable!(leasetenant -> tenant (tenant_id));
joinable!(lender -> account (account_id));
joinable!(property -> lender (lender_id));
joinable!(rent -> lease (leaseId));
joinable!(rent -> transaction (transactionId));
joinable!(tenant -> lease (lease_id));
joinable!(transaction -> lease (leaseId));

allow_tables_to_appear_in_same_query!(
    account,
    company,
    file,
    lease,
    leasetenant,
    lender,
    plan,
    property,
    rent,
    tenant,
    transaction,
    user,
);
