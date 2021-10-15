CREATE TYPE plancode AS ENUM (
    'solo'
);

CREATE TABLE plans (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    code PLANCODE NOT NULL,
    price NUMERIC,
    subtitle TEXT,
    title TEXT
);
