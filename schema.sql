-- # Enums

CREATE TYPE plancode AS ENUM (
    'solo'
);

CREATE TYPE accountstatus AS ENUM (
    'active',
    'canceled',
    'incomplete',
    'incomplete_expired',
    'past_due',
    'trialing',
    'unpaid'
);

CREATE TYPE personrole AS ENUM (
    'admin',
    'user',
    'viewer',
    'tenant',
    'warrant'
);

CREATE TYPE legalentitytype AS ENUM (
    'eurl',
    'other',
    'sa',
    'sarl',
    'sas',
    'sasu',
    'sci',
    'scp',
    'snc'
);

CREATE TYPE propertystatus AS ENUM (
    'for_sale',
    'inactive',
    'rented',
    'under_construction',
    'unrented'
);

CREATE TYPE propertyroomtype AS ENUM (
    'other',
    't1',
    't2',
    't3',
    't4',
    't5',
    't6'
);

CREATE TYPE propertybuildperiodtype AS ENUM (
    'before_1949',
    'from_1949_1974',
    'from_1975_1989',
    'from_1990_2005',
    'from_2005'
);

CREATE TYPE propertyenergyclass AS ENUM (
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g'
);

CREATE TYPE propertygasemission AS ENUM (
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g'
);

CREATE TYPE propertybuildinglegalstatus AS ENUM (
    'copro',
    'mono'
);

CREATE TYPE propertyhabitationusagetype AS ENUM (
    'habitation',
    'mixte'
);

CREATE TYPE propertyusagetype AS ENUM (
    'collective',
    'individual'
);

CREATE TYPE filestatus AS ENUM (
    'draft',
    'failure',
    'generating',
    'pending',
    'success'
);

CREATE TYPE filetype AS ENUM (
    'lease_document',
    'payment_notice',
    'rent_receipt'
);

CREATE TYPE leasetype AS ENUM (
    'furnished',
    'naked'
);

CREATE TYPE leaseduration AS ENUM (
    'nine_months',
    'one_year'
);

CREATE TYPE rentstatus AS ENUM (
    'open',
    'paid',
    'partially_paid'
);

CREATE TYPE tenantstatus AS ENUM (
    'candidate',
    'gone',
    'late',
    'new',
    'uptodate'
);

CREATE TYPE transactiontype AS ENUM (
    'insurance_hab',
    'insurance_pno',
    'invoice',
    'loan_interest',
    'loan_payment',
    'other',
    'rent'
);

CREATE TYPE entryflexibility AS ENUM (
    'one_day',
    'three_days',
    'seven_days'
);

CREATE TYPE candidacystatus AS ENUM (
    'pending',
    'rejected',
    'accepted'
);

CREATE TYPE warranttype AS ENUM (
    'person',
    'visale',
    'company'
);

CREATE TYPE eventtype AS ENUM (
    'candidacy_created',
    'candidacy_accepted',
    'candidacy_rejected',
    'notice_created',
    'notice_sent',
    'receipt_created',
    'receipt_sent',
    'payment_created'
);

CREATE TYPE discussionstatus AS ENUM (
    'active',
    'candidacy'
);

CREATE TYPE invitestatus AS ENUM (
  'pending',
  'accepted'
);

CREATE TYPE invitereason AS ENUM (
  'candidacy_accepted'
);

-- # Types

CREATE TABLE plans (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    code PLANCODE NOT NULL,
    price NUMERIC,
    subtitle TEXT,
    title TEXT
);

CREATE TABLE accounts (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    plan_id UUID REFERENCES plans(id),
    status ACCOUNTSTATUS NOT NULL,
    stripe_customer_id TEXT,
    stripe_subscription_id TEXT,
    trial_end TIMESTAMPTZ
);

CREATE TABLE persons (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    auth_id TEXT UNIQUE,
    email TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    address JSONB,
    photo_url TEXT,
    role PERSONROLE NOT NULL,
    phone_number TEXT
);

CREATE TABLE companies (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    address JSONB,
    email TEXT NOT NULL,
    legal_entity TEXT NOT NULL,
    legal_entity_identifier TEXT,
    legal_entity_type LEGALENTITYTYPE,
    legal_entity_type_other TEXT,
    phone_number TEXT
);

CREATE TABLE lenders (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    individual_id UUID REFERENCES persons(id),
    company_id UUID REFERENCES companies(id)
);

CREATE TABLE properties (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    address JSONB NOT NULL,
    build_period PROPERTYBUILDPERIODTYPE,
    building_legal_status PROPERTYBUILDINGLEGALSTATUS,
    common_spaces TEXT,
    energy_class PROPERTYENERGYCLASS,
    equipments TEXT,
    gas_emission PROPERTYGASEMISSION,
    heating_method PROPERTYUSAGETYPE,
    housing_type PROPERTYUSAGETYPE,
    name TEXT NOT NULL,
    note TEXT,
    ntic_equipments TEXT,
    other_spaces TEXT,
    tax NUMERIC,
    room_count PROPERTYROOMTYPE NOT NULL,
    status PROPERTYSTATUS NOT NULL,
    surface REAL NOT NULL,
    tenant_private_spaces TEXT,
    usage_type PROPERTYHABITATIONUSAGETYPE,
    water_heating_method PROPERTYUSAGETYPE,
    lender_id UUID NOT NULL REFERENCES lenders(id)
);

CREATE TABLE files (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    download_url TEXT,
    external_id TEXT,
    filename TEXT,
    preview_url TEXT,
    status FILESTATUS,
    type FILETYPE NOT NULL
);

CREATE TABLE leases (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    deposit_amount NUMERIC,
    effect_date TIMESTAMPTZ NOT NULL,
    signature_date TIMESTAMPTZ,
    rent_amount NUMERIC NOT NULL,
    rent_charges_amount NUMERIC,
    type LEASETYPE NOT NULL,
    lease_id UUID REFERENCES files(id),
    property_id UUID NOT NULL REFERENCES properties(id),
    details JSONB,
    expired_at TIMESTAMPTZ,
    renew_date TIMESTAMPTZ,
    duration LEASEDURATION NOT NULL
);

CREATE TABLE rents (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    period_end TIMESTAMPTZ NOT NULL,
    period_start TIMESTAMPTZ NOT NULL,
    amount NUMERIC NOT NULL,
    charges_amount NUMERIC,
    full_amount NUMERIC NOT NULL,
    status RENTSTATUS NOT NULL,
    lease_id UUID NOT NULL REFERENCES leases(id) ON DELETE CASCADE,
    receipt_id UUID REFERENCES files(id),
    notice_id UUID REFERENCES files(id)
);

CREATE TABLE tenants (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    person_id UUID NOT NULL REFERENCES persons(id),
    apl BOOLEAN,
    birthdate DATE,
    birthplace TEXT,
    email TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    note TEXT,
    phone_number TEXT,
    status TENANTSTATUS NOT NULL,
    lease_id UUID REFERENCES leases(id) ON DELETE SET NULL,
    is_student BOOLEAN
);

CREATE TABLE payments (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    rent_id UUID NOT NULL REFERENCES rents(id),
    amount NUMERIC NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    type TRANSACTIONTYPE NOT NULL,
    label TEXT
);

CREATE TABLE advertisements (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    published BOOLEAN NOT NULL,
    lease_type LEASETYPE NOT NULL,
    rent_amount NUMERIC NOT NULL,
    rent_charges_amount NUMERIC,
    deposit_amount NUMERIC,
    effect_date TIMESTAMPTZ NOT NULL,
    flexibility ENTRYFLEXIBILITY,
    referral_lease_id UUID REFERENCES leases(id),
    property_id UUID NOT NULL REFERENCES properties(id),
    title TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE candidacies (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    status CANDIDACYSTATUS NOT NULL,
    advertisement_id UUID NOT NULL REFERENCES advertisements(id),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    move_in_date TIMESTAMPTZ NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE professional_warrants (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    name TEXT NOT NULL,
    identifier TEXT NOT NULL
);

CREATE TABLE warrants (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    type WARRANTTYPE NOT NULL,
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    individual_id UUID REFERENCES persons(id),
    professional_id UUID REFERENCES professional_warrants(id)
);

CREATE TABLE eventables (
    id UUID NOT NULL PRIMARY KEY,
    file_id UUID REFERENCES files(id),
    rent_id UUID REFERENCES rents(id),
    payment_id UUID REFERENCES payments(id),
    candidacy_id UUID REFERENCES candidacies(id),

    CHECK (num_nonnulls(file_id, rent_id, payment_id, candidacy_id) = 1)
);

CREATE TABLE events (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    participant_id UUID NOT NULL REFERENCES persons(id),
    eventable_id UUID NOT NULL REFERENCES eventables(id),
    type EVENTTYPE NOT NULL
);

CREATE TABLE discussions (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    initiator_id UUID NOT NULL REFERENCES persons(id),
    status DISCUSSIONSTATUS NOT NULL
);

CREATE TABLE messages (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    discussion_id UUID NOT NULL REFERENCES discussions(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES persons(id),
    content TEXT,
    event_id UUID REFERENCES events(id),

    CHECK (num_nonnulls(content, event_id) = 1)
);

CREATE TABLE invites (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID REFERENCES accounts(id),
    invitee_id UUID NOT NULL REFERENCES persons(id),
    token TEXT NOT NULL,
    status INVITESTATUS NOT NULL,
    reason INVITEREASON NOT NULL
);

-- # Utils

SELECT manage_updated_at('accounts');
SELECT manage_updated_at('persons');
SELECT manage_updated_at('companies');
SELECT manage_updated_at('lenders');
SELECT manage_updated_at('properties');
SELECT manage_updated_at('files');
SELECT manage_updated_at('leases');
SELECT manage_updated_at('rents');
SELECT manage_updated_at('tenants');
SELECT manage_updated_at('payments');
SELECT manage_updated_at('advertisements');
SELECT manage_updated_at('candidacies');
SELECT manage_updated_at('professional_warrants');
SELECT manage_updated_at('warrants');
SELECT manage_updated_at('events');
SELECT manage_updated_at('discussions');
SELECT manage_updated_at('messages');
SELECT manage_updated_at('invites');

SELECT manage_id('eventables');
