CREATE TYPE eventtype AS ENUM (
    'candidacy_accepted',
    'candidacy_created',
    'candidacy_rejected',
    'lease_created',
    'notice_created',
    'notice_sent',
    'payment_created',
    'receipt_created',
    'receipt_sent',
    'step_completed'
);

CREATE TABLE events (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    participant_id UUID NOT NULL REFERENCES persons(id),
    eventable_id UUID NOT NULL REFERENCES eventables(id),
    type EVENTTYPE NOT NULL
);

SELECT manage_updated_at('events');
