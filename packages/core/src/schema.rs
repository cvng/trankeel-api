table! {
    account (id) {
        planId -> Nullable<Uuid>,
        status -> Nullable<Text>,
        stripeCustomerId -> Nullable<Text>,
        stripeSubscriptionId -> Nullable<Text>,
        trialEnd -> Nullable<Timestamp>,
        ownerId -> Text,
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
        endedDate -> Nullable<Timestamp>,
        signatureDate -> Nullable<Timestamp>,
        rentAmount -> Numeric,
        rentChargesAmount -> Nullable<Numeric>,
        #[sql_name = "type"]
        type_ -> Text,
        leaseId -> Nullable<Uuid>,
        propertyId -> Uuid,
        id -> Uuid,
        data -> Nullable<Jsonb>,
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
        accountId -> Uuid,
        individualId -> Nullable<Uuid>,
        companyId -> Nullable<Uuid>,
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
        // address -> Jsonb,
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
    }
}

table! {
    tenant (id) {
        accountId -> Uuid,
        apl -> Bool,
        authId -> Nullable<Text>,
        birthdate -> Timestamp,
        birthplace -> Nullable<Text>,
        email -> Text,
        firstName -> Text,
        lastName -> Text,
        note -> Nullable<Text>,
        phoneNumber -> Nullable<Text>,
        role -> Nullable<Text>,
        id -> Uuid,
        leaseId -> Nullable<Uuid>,
        visaleId -> Nullable<Text>,
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
        authId -> Nullable<Text>,
        email -> Text,
        firstName -> Nullable<Text>,
        lastName -> Nullable<Text>,
        // address -> Nullable<Jsonb>,
        photoURL -> Nullable<Text>,
        role -> Nullable<Text>,
        id -> Uuid,
        phoneNumber -> Nullable<Text>,
        accountId -> Nullable<Uuid>,
    }
}

joinable!(account -> plan (planId));
joinable!(lease -> account (accountId));
joinable!(lease -> file (leaseId));
joinable!(lease -> property (propertyId));
joinable!(leasetenant -> lease (leaseId));
joinable!(leasetenant -> tenant (tenantId));
joinable!(lender -> account (accountId));
joinable!(property -> lender (lenderId));
joinable!(rent -> lease (leaseId));
joinable!(rent -> transaction (transactionId));
joinable!(tenant -> lease (leaseId));
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
