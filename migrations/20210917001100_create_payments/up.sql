CREATE TABLE payments (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    rent_id UUID NOT NULL REFERENCES rents(id),
    amount NUMERIC NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    type TEXT NOT NULL,
    label TEXT
);

SELECT manage_updated_at('payments');
