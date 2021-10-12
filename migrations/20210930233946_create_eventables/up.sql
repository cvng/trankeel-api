CREATE TABLE eventables (
    id UUID NOT NULL PRIMARY KEY,
    file_id UUID REFERENCES files(id),
    rent_id UUID REFERENCES rents(id),
    payment_id UUID REFERENCES payments(id),
    candidacy_id UUID REFERENCES candidacies(id)
);

ALTER TABLE eventables ADD CONSTRAINT eventables_check CHECK (num_nonnulls(file_id, rent_id, payment_id, candidacy_id) = 1);

CREATE OR REPLACE FUNCTION eventables_set_id() RETURNS trigger AS $$
BEGIN
    NEW.id := coalesce(NEW.file_id, NEW.rent_id, NEW.payment_id, NEW.candidacy_id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_id BEFORE INSERT OR UPDATE ON eventables FOR EACH ROW EXECUTE PROCEDURE eventables_set_id();
