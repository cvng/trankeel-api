CREATE TYPE filestatus AS ENUM (
    'draft',
    'failure',
    'generating',
    'pending',
    'success'
);

CREATE TYPE filetype AS ENUM (
    'lease_document',
    'payment_notice',
    'rent_receipt'
);

CREATE TABLE files (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    download_url TEXT,
    external_id TEXT,
    filename TEXT,
    preview_url TEXT,
    status FILESTATUS,
    type FILETYPE NOT NULL
);

SELECT manage_updated_at('files');
