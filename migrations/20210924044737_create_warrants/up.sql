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
    tenant_id UUID REFERENCES tenants(id),
    individual_id UUID REFERENCES persons(id),
    professional_id UUID REFERENCES professional_warrants(id),
    candidacy_id UUID REFERENCES candidacies(id),

    CHECK (num_nonnulls(tenant_id, candidacy_id) = 1),
    CHECK (num_nonnulls(individual_id, professional_id) = 1)
);

SELECT manage_updated_at('warrants');
