CREATE TABLE messages (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    discussion_id UUID NOT NULL REFERENCES discussions(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES persons(id),
    content TEXT,
    event_id UUID REFERENCES events(id)
);

ALTER TABLE messages ADD CONSTRAINT content_or_event_id_check CHECK (num_nonnulls(content, event_id) = 1);

SELECT manage_updated_at('messages');
