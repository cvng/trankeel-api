CREATE TYPE workflowtype AS ENUM (
    'candidacy'
);

CREATE TABLE workflows (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    workflowable_id UUID NOT NULL REFERENCES workflowables(id),
    type WORKFLOWTYPE NOT NULL,
    completed BOOLEAN NOT NULL
);

SELECT manage_updated_at('workflows');
