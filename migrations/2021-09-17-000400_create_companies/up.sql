CREATE TABLE companies (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    address JSONB,
    email TEXT NOT NULL,
    legal_entity TEXT NOT NULL,
    legal_entity_identifier TEXT,
    legal_entity_type TEXT,
    legal_entity_type_other TEXT,
    phone_number TEXT
);

SELECT diesel_manage_updated_at('companies');
