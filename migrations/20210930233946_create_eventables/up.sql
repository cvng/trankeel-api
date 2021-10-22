CREATE TABLE eventables (
    id UUID NOT NULL PRIMARY KEY,
    file_id UUID REFERENCES files(id),
    rent_id UUID REFERENCES rents(id),
    step_id UUID REFERENCES steps(id),
    lease_id UUID REFERENCES leases(id),
    payment_id UUID REFERENCES payments(id),
    candidacy_id UUID REFERENCES candidacies(id),

    CHECK (num_nonnulls(file_id, rent_id, step_id, lease_id, payment_id, candidacy_id) = 1)
);

SELECT manage_id('eventables');
