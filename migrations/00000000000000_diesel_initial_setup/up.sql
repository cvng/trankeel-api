-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

--

CREATE OR REPLACE FUNCTION manage_id(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_id BEFORE INSERT OR UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE app_set_id()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION app_set_id() RETURNS trigger AS $$
DECLARE
    _cols TEXT[];
BEGIN
    SELECT array_agg(column_name::TEXT)
    FROM information_schema.columns
    WHERE table_name = TG_TABLE_NAME
    INTO _cols;

    EXECUTE format('SELECT coalesce(%s)
                    FROM (SELECT $1.*) AS NEW', array_to_string(_cols, ','))
    USING NEW
    INTO NEW.id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

--

CREATE OR REPLACE FUNCTION manage_payload(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER notify_payload AFTER INSERT ON %s
                    FOR EACH ROW EXECUTE PROCEDURE app_notify_payload()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION app_notify_payload() RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('events', NEW.payload::TEXT);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
