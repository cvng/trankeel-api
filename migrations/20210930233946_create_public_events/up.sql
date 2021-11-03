CREATE TYPE eventabletype AS ENUM (
    'advertisement',
    'candidacy',
    'discussion',
    'file',
    'lease',
    'message',
    'payment',
    'person',
    'rent',
    'step',
    'tenant',
    'warrant'
);

CREATE TABLE public_events (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    event_id UUID NOT NULL REFERENCES events(id),
    event_type EVENTTYPE NOT NULL,
    eventable_type EVENTABLETYPE NOT NULL,
    eventable JSONB NOT NULL
);

SELECT manage_updated_at('public_events');
