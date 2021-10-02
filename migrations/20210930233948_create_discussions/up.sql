CREATE TABLE discussions (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    initiator_id UUID NOT NULL REFERENCES persons(id),
    subject_id UUID,
    type TEXT NOT NULL
);

SELECT manage_updated_at('discussions');
