CREATE TABLE plans (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    code TEXT NOT NULL,
    price NUMERIC,
    subtitle TEXT,
    title TEXT
);
