CREATE OR REPLACE VIEW reports AS
WITH
summary_1 AS (
    SELECT
        3000 AS amount_expected,
        1000 AS amount_settled,
        500 AS amount_partial,
        --
        3 AS n_expected,
        1 AS n_settled,
        1 AS n_partial,
        --
        3 AS n_units_owned,
        3 AS n_units_rented
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
)
SELECT
    current_timestamp AS since,
    current_timestamp AS until,
    --
    amount_expected,
    amount_received,
    amount_settled,
    amount_partial,
    amount_pending,
    --
    n_expected,
    n_received,
    n_settled,
    n_partial,
    n_pending,
    --
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
FROM summary_1, summary_2, summary_3;
