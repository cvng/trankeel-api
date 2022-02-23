CREATE TYPE eventtype AS ENUM (
    'account_created',
    'advertisement_created',
    'advertisement_updated',
    'candidacy_accepted',
    'candidacy_created',
    'candidacy_rejected',
    'company_created',
    'discussion_created',
    'discussion_deleted',
    'document_generated',
    'invite_accepted',
    'invite_created',
    'lease_affected',
    'lease_created',
    'lease_deleted',
    'lease_file_requested',
    'lease_updated',
    'lender_created',
    'lender_updated',
    'message_pushed',
    'notice_created',
    'payment_created',
    'person_created',
    'property_created',
    'property_deleted',
    'property_updated',
    'receipt_created',
    'receipt_sent',
    'step_completed',
    'step_created',
    'subscription_requested',
    'tenant_created',
    'tenant_updated',
    'tenant_deleted',
    'warrant_created',
    'workflow_created'
);

CREATE TABLE events (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    type EVENTTYPE NOT NULL,
    payload JSONB NOT NULL
);

SELECT manage_payload('events');
