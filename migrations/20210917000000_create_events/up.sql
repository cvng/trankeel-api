CREATE TYPE eventtype AS ENUM (
    'candidacy_created',
    'candidacy_accepted',
    'candidacy_rejected',
    'discussion_created',
    'discussion_updated',
    'lease_created',
    'message_created',
    'notice_created',
    'notice_sent',
    'payment_created',
    'receipt_created',
    'receipt_sent',
    'step_completed',
    'tenant_created',
    'warrant_created'
);

CREATE TABLE events (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    type EVENTTYPE NOT NULL,
    payload JSONB NOT NULL
);
