CREATE TYPE expected_rents AS (
    account_id UUID,
    amount_expected NUMERIC,
    amount_received NUMERIC,
    amount_settled NUMERIC,
    amount_partial NUMERIC,
    amount_pending NUMERIC,
    n_expected INTEGER,
    n_received INTEGER,
    n_settled INTEGER,
    n_partial INTEGER,
    n_pending INTEGER
);

CREATE TYPE summary_reports AS (
    account_id UUID,
    created_at TIMESTAMPTZ,
    amount_expected NUMERIC,
    amount_received NUMERIC,
    amount_settled NUMERIC,
    amount_partial NUMERIC,
    amount_pending NUMERIC,
    n_expected INTEGER,
    n_received INTEGER,
    n_settled INTEGER,
    n_partial INTEGER,
    n_pending INTEGER,
    ratio_expected FLOAT,
    ratio_received FLOAT,
    ratio_settled FLOAT,
    ratio_partial FLOAT,
    ratio_pending FLOAT,
    variation_expected FLOAT,
    variation_received FLOAT,
    variation_settled FLOAT,
    variation_partial FLOAT,
    variation_pending FLOAT,
    payment_rate FLOAT,
    occupation_rate FLOAT
);

CREATE OR REPLACE FUNCTION variation(x NUMERIC, y NUMERIC) RETURNS NUMERIC AS $$
BEGIN
    IF y = 0 THEN
        RETURN 0;
    END IF;
    RETURN (x - y) / y * 100;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_expected_rents(current_month TIMESTAMPTZ) RETURNS SETOF expected_rents AS $$
BEGIN
    RETURN QUERY
    WITH
    expected_rents AS (
        SELECT
            accounts.id AS account_id,

            sum(full_amount) AS amount_expected,
            sum(full_amount) FILTER (WHERE rents.status = 'paid') AS amount_settled,
            sum(full_amount) FILTER (WHERE rents.status = 'partially_paid') AS amount_partial,

            count(rents.id) AS n_expected,
            count(rents.id) FILTER (WHERE rents.status = 'paid') AS n_settled,
            count(rents.id) FILTER (WHERE rents.status = 'partially_paid') AS n_partial
        FROM rents
        LEFT JOIN leases ON leases.id = rents.lease_id
        LEFT JOIN accounts ON accounts.id = leases.account_id
        WHERE date_trunc('month', period_start) = date_trunc('month', current_month)
        GROUP BY accounts.id
        HAVING accounts.id IS NOT NULL
    ),
    received_rents AS (
        SELECT
            amount_settled + amount_partial AS amount_received,
            n_settled + n_partial AS n_received
        FROM expected_rents
    ),
    pending_rents AS (
        SELECT
            amount_expected - amount_received AS amount_pending,
            n_expected - n_received AS n_pending
        FROM expected_rents, received_rents
    )
    SELECT
        account_id,
        amount_expected,
        amount_received,
        amount_settled,
        amount_partial,
        amount_pending,
        n_expected::INTEGER,
        n_received::INTEGER,
        n_settled::INTEGER,
        n_partial::INTEGER,
        n_pending::INTEGER
    FROM expected_rents, received_rents, pending_rents;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION generate_summary_reports(current_month TIMESTAMPTZ) RETURNS SETOF summary_reports AS $$
BEGIN
    RETURN QUERY
    WITH
    expected_rents AS (
        SELECT * FROM get_expected_rents(current_month)
    ),
    rented_properties AS (
        SELECT
            count(properties.id) AS n_units_owned,
            count(properties.id) FILTER (WHERE properties.status = 'rented') AS n_units_rented
        FROM expected_rents
        LEFT JOIN properties ON properties.account_id = expected_rents.account_id
    ),
    ratio_and_variations AS (
        SELECT
            100.0 * amount_expected / amount_expected AS ratio_expected,
            100.0 * amount_received / amount_expected AS ratio_received,
            100.0 * amount_settled / amount_expected AS ratio_settled,
            100.0 * amount_partial / amount_expected AS ratio_partial,
            100.0 * amount_pending / amount_expected AS ratio_pending,

            variation(0, 0) AS variation_expected, -- TODO
            variation(0, 0) AS variation_received, -- TODO
            variation(0, 0) AS variation_settled, -- TODO
            variation(0, 0) AS variation_partial, -- TODO
            variation(0, 0) AS variation_pending, -- TODO

            100.0 * n_received / n_expected AS payment_rate,
            100.0 * n_units_rented / n_units_owned AS occupation_rate
        FROM expected_rents, rented_properties
    )
    SELECT
        account_id,
        current_month AS created_at,

        coalesce(amount_expected, 0)::NUMERIC amount_expected,
        coalesce(amount_received, 0)::NUMERIC amount_received,
        coalesce(amount_settled, 0)::NUMERIC amount_settled,
        coalesce(amount_partial, 0)::NUMERIC amount_partial,
        coalesce(amount_pending, 0)::NUMERIC amount_pending,

        coalesce(n_expected, 0)::INTEGER AS n_expected,
        coalesce(n_received, 0)::INTEGER AS n_received,
        coalesce(n_settled, 0)::INTEGER AS n_settled,
        coalesce(n_partial, 0)::INTEGER AS n_partial,
        coalesce(n_pending, 0)::INTEGER AS n_pending,

        coalesce(ratio_expected, 0)::FLOAT AS ratio_expected,
        coalesce(ratio_received, 0)::FLOAT AS ratio_received,
        coalesce(ratio_settled, 0)::FLOAT AS ratio_settled,
        coalesce(ratio_partial, 0)::FLOAT AS ratio_partial,
        coalesce(ratio_pending, 0)::FLOAT AS ratio_pending,

        coalesce(variation_expected, 0)::FLOAT AS variation_expected,
        coalesce(variation_received, 0)::FLOAT AS variation_received,
        coalesce(variation_settled, 0)::FLOAT AS variation_settled,
        coalesce(variation_partial, 0)::FLOAT AS variation_partial,
        coalesce(variation_pending, 0)::FLOAT AS variation_pending,

        coalesce(payment_rate, 0)::FLOAT AS payment_rate,
        coalesce(occupation_rate, 0)::FLOAT AS occupation_rate
    FROM expected_rents, rented_properties, ratio_and_variations;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE VIEW reports AS
SELECT * FROM generate_summary_reports(current_timestamp);
