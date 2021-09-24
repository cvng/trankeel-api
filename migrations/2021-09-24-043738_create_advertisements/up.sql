CREATE TABLE advertisements (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    published BOOLEAN NOT NULL,
    lease_type TEXT NOT NULL,
    rent_amount NUMERIC NOT NULL,
    rent_charges_amount NUMERIC NOT NULL,
    deposit_amount NUMERIC NOT NULL,
    effect_date TIMESTAMPTZ NOT NULL,
    flexibility TEXT NOT NULL,
    referral_lease_id UUID REFERENCES leases(id),
    property_id UUID NOT NULL REFERENCES properties(id)
);

SELECT diesel_manage_updated_at('advertisements');
