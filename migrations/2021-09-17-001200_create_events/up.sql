CREATE TABLE events (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    eventable_id UUID NOT NULL,
    eventable_model TEXT NOT NULL,
    type TEXT NOT NULL
);

SELECT diesel_manage_updated_at('events');
