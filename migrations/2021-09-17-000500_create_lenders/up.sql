CREATE TABLE lenders (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    individual_id UUID REFERENCES persons(id),
    company_id UUID REFERENCES companies(id)
);

SELECT diesel_manage_updated_at('lenders');
