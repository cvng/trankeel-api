CREATE TYPE warranttype AS ENUM (
    'person',
    'visale',
    'company'
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

SELECT manage_updated_at('warrants');
