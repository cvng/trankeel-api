CREATE TYPE leasetype AS ENUM (
    'furnished',
    'naked'
);

CREATE TYPE leaseduration AS ENUM (
    'nine_months',
    'one_year'
);

CREATE TABLE leases (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    deposit_amount NUMERIC NOT NULL,
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

SELECT manage_updated_at('leases');
