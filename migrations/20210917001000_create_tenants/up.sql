CREATE TABLE tenants (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    apl BOOLEAN,
    birthdate DATE NOT NULL,
    birthplace TEXT,
    email TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    note TEXT,
    phone_number TEXT,
    status TEXT NOT NULL,
    lease_id UUID REFERENCES leases(id) ON DELETE SET NULL,
    is_student BOOLEAN
);

SELECT manage_updated_at('tenants');