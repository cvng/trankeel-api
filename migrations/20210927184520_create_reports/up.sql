CREATE OR REPLACE VIEW reports AS
WITH
summary_1 AS (
    SELECT
        accounts.id AS account_id,
        date_trunc('month', current_timestamp) AS current_month,
        --
        sum(full_amount) AS amount_expected,
        sum(full_amount) FILTER (WHERE rents.status = 'paid') AS amount_settled,
        sum(full_amount) FILTER (WHERE rents.status = 'partially_paid') AS amount_partial,
        --
        count(rents.id) AS n_expected,
        count(rents.id) FILTER (WHERE rents.status = 'paid') AS n_settled,
        count(rents.id) FILTER (WHERE rents.status = 'partially_paid') AS n_partial
    FROM rents
    LEFT JOIN leases ON leases.id = rents.lease_id
    LEFT JOIN accounts ON accounts.id = leases.account_id
    AND date_trunc('month', period_start) = date_trunc('month', current_timestamp)
    GROUP BY accounts.id
    HAVING accounts.id IS NOT NULL
),
summary_2 AS (
    SELECT
        amount_settled + amount_partial AS amount_received,
        n_settled + n_partial AS n_received
    FROM summary_1
),
summary_3 AS (
    SELECT
        amount_expected - amount_received AS amount_pending,
        n_expected - n_received AS n_pending
    FROM summary_1, summary_2
),
summary_4 AS (
    SELECT
        count(properties.id) AS n_units_owned,
        count(properties.id) FILTER (WHERE properties.status = 'rented') AS n_units_rented
    FROM summary_1
    LEFT JOIN properties ON properties.account_id = summary_1.account_id
),
summary_5 AS (
    SELECT
        100.0 * amount_expected / amount_expected AS ratio_expected,
        100.0 * amount_received / amount_expected AS ratio_received,
        100.0 * amount_settled / amount_expected AS ratio_settled,
        100.0 * amount_partial / amount_expected AS ratio_partial,
        100.0 * amount_pending / amount_expected AS ratio_pending,
        --
        0 AS variation_expected, -- TODO
        0 AS variation_received, -- TODO
        0 AS variation_settled, -- TODO
        0 AS variation_partial, -- TODO
        0 AS variation_pending, -- TODO
        --
        100.0 * n_received / n_expected AS payment_rate,
        100.0 * n_units_rented / n_units_owned AS occupation_rate
    FROM summary_1, summary_2, summary_3, summary_4
)
SELECT
    account_id,
    current_month,
    --
    coalesce(amount_expected, 0)::numeric amount_expected,
    coalesce(amount_received, 0)::numeric amount_received,
    coalesce(amount_settled, 0)::numeric amount_settled,
    coalesce(amount_partial, 0)::numeric amount_partial,
    coalesce(amount_pending, 0)::numeric amount_pending,
    --
    coalesce(n_expected, 0)::integer AS n_expected,
    coalesce(n_received, 0)::integer AS n_received,
    coalesce(n_settled, 0)::integer AS n_settled,
    coalesce(n_partial, 0)::integer AS n_partial,
    coalesce(n_pending, 0)::integer AS n_pending,
    --
    coalesce(ratio_expected, 0)::float AS ratio_expected,
    coalesce(ratio_received, 0)::float AS ratio_received,
    coalesce(ratio_settled, 0)::float AS ratio_settled,
    coalesce(ratio_partial, 0)::float AS ratio_partial,
    coalesce(ratio_pending, 0)::float AS ratio_pending,
    --
    coalesce(variation_expected, 0)::float AS variation_expected,
    coalesce(variation_received, 0)::float AS variation_received,
    coalesce(variation_settled, 0)::float AS variation_settled,
    coalesce(variation_partial, 0)::float AS variation_partial,
    coalesce(variation_pending, 0)::float AS variation_pending,
    --
    coalesce(payment_rate, 0)::float AS payment_rate,
    coalesce(occupation_rate, 0)::float AS occupation_rate
FROM summary_1, summary_2, summary_3, summary_4, summary_5;
