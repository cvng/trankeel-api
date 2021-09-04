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
        accountId -> Uuid,
        depositAmount -> Nullable<Numeric>,
        effectDate -> Timestamp,
        signatureDate -> Nullable<Timestamp>,
        rentAmount -> Numeric,
        rentChargesAmount -> Nullable<Numeric>,
        #[sql_name = "type"]
        type_ -> Text,
        leaseId -> Nullable<Uuid>,
        propertyId -> Uuid,
        id -> Uuid,
        data -> Nullable<Jsonb>,
        expiredAt -> Nullable<Timestamp>,
        renewDate -> Nullable<Timestamp>,
    }
}

table! {
    leasetenant (leaseId, tenantId) {
        leaseId -> Uuid,
        tenantId -> Uuid,
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
        accountId -> Nullable<Uuid>,
        address -> Jsonb,
        buildPeriod -> Nullable<Text>,
        buildingLegalStatus -> Nullable<Text>,
        commonSpaces -> Nullable<Text>,
        energyClass -> Nullable<Text>,
        equipments -> Nullable<Text>,
        gasEmission -> Nullable<Text>,
        heatingMethod -> Nullable<Text>,
        housingType -> Nullable<Text>,
        name -> Text,
        note -> Nullable<Text>,
        nticEquipments -> Nullable<Text>,
        otherSpaces -> Nullable<Text>,
        tax -> Nullable<Float8>,
        roomCount -> Text,
        status -> Nullable<Text>,
        surface -> Int4,
        tenantPrivateSpaces -> Nullable<Text>,
        usageType -> Nullable<Text>,
        waterHeatingMethod -> Nullable<Text>,
        id -> Uuid,
        lenderId -> Uuid,
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
joinable!(lease -> account (accountId));
joinable!(lease -> file (leaseId));
joinable!(lease -> property (propertyId));
joinable!(leasetenant -> lease (leaseId));
joinable!(leasetenant -> tenant (tenantId));
joinable!(lender -> account (account_id));
joinable!(property -> lender (lenderId));
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
