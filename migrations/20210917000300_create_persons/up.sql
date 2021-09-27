CREATE TYPE personrole AS ENUM ('admin', 'user', 'viewer', 'tenant', 'warrant');

CREATE TABLE persons (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    auth_id TEXT UNIQUE,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    address JSONB,
    photo_url TEXT,
    role PERSONROLE NOT NULL,
    phone_number TEXT
);

SELECT manage_updated_at('persons');
