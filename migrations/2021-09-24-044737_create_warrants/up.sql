CREATE TABLE warrants (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    type TEXT NOT NULL,
    identifier TEXT,
    person_id UUID REFERENCES persons(id),
    tenant_id UUID NOT NULL REFERENCES tenants(id)
);

SELECT diesel_manage_updated_at('warrants');
