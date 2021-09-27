CREATE TABLE professional_warrants (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    name TEXT NOT NULL,
    identifier TEXT NOT NULL
);

SELECT manage_updated_at('professional_warrants');
