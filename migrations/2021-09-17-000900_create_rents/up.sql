CREATE TABLE rents (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    period_end TIMESTAMPTZ NOT NULL,
    period_start TIMESTAMPTZ NOT NULL,
    amount NUMERIC NOT NULL,
    charges_amount NUMERIC,
    full_amount NUMERIC NOT NULL,
    status TEXT NOT NULL,
    lease_id UUID NOT NULL REFERENCES leases(id),
    receipt_id UUID REFERENCES files(id),
    notice_id UUID REFERENCES files(id)
);

SELECT diesel_manage_updated_at('rents');
