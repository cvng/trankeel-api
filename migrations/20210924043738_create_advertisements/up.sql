CREATE TABLE advertisements (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    published BOOLEAN NOT NULL,
    lease_type TEXT NOT NULL,
    rent_amount DECIMAL(65,30) NOT NULL,
    rent_charges_amount DECIMAL(65,30),
    deposit_amount DECIMAL(65,30),
    effect_date TIMESTAMPTZ NOT NULL,
    flexibility TEXT,
    referral_lease_id UUID REFERENCES leases(id),
    property_id UUID NOT NULL REFERENCES properties(id)
);

SELECT manage_updated_at('advertisements');
