CREATE TABLE eventables (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    file_id UUID REFERENCES files(id),
    rent_id UUID REFERENCES rents(id),
    payment_id UUID REFERENCES payments(id),
    candidacy_id UUID REFERENCES candidacies(id)
);

ALTER TABLE eventables ADD CONSTRAINT eventables_check CHECK (num_nonnulls(file_id, rent_id, payment_id, candidacy_id) = 1);
