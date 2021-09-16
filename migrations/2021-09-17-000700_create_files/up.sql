CREATE TABLE files (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    download_url TEXT,
    external_id TEXT,
    filename TEXT,
    preview_url TEXT,
    status TEXT,
    type TEXT NOT NULL
);

SELECT diesel_manage_updated_at('files');
