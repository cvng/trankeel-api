CREATE TABLE steps (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    workflow_id UUID NOT NULL REFERENCES workflows(id),
    label TEXT NOT NULL,
    completed BOOLEAN NOT NULL,
    confirmation TEXT,
    requirements JSONB
);

SELECT manage_updated_at('steps');
