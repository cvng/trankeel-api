CREATE TYPE invitestatus AS ENUM (
  'pending',
  'accepted'
);

CREATE TYPE invitereason AS ENUM (
  'candidacy_accepted'
);

CREATE TABLE invites (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    invitee_id UUID NOT NULL REFERENCES persons(id),
    token TEXT NOT NULL,
    status INVITESTATUS NOT NULL,
    reason INVITEREASON NOT NULL
);

SELECT manage_updated_at('invites');
