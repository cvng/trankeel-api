CREATE TYPE candidacystatus AS ENUM (
    'pending',
    'rejected',
    'accepted'
);

CREATE TABLE candidacies (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    status CANDIDACYSTATUS NOT NULL,
    advertisement_id UUID NOT NULL REFERENCES advertisements(id),
    person_id UUID NOT NULL REFERENCES persons(id),
    move_in_date TIMESTAMPTZ NOT NULL,
    description TEXT NOT NULL,
    birthdate DATE,
    birthplace TEXT,
    is_student BOOLEAN
);

SELECT manage_updated_at('candidacies');
