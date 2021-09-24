CREATE TABLE candidacies (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    status TEXT NOT NULL,
    advertisement_id UUID NOT NULL REFERENCES advertisements(id),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    move_in_date TIMESTAMPTZ NOT NULL,
    description TEXT NOT NULL
);

SELECT diesel_manage_updated_at('candidacies');
