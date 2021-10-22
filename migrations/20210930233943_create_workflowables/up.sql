CREATE TABLE workflowables (
    id UUID NOT NULL PRIMARY KEY,
    candidacy_id UUID REFERENCES candidacies(id),

    CHECK (num_nonnulls(candidacy_id) = 1)
);

SELECT manage_id('workflowables');
