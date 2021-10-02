CREATE TABLE messages (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    discussion_id UUID NOT NULL REFERENCES discussions(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES persons(id),
    content TEXT NOT NULL
);

SELECT manage_updated_at('messages');
