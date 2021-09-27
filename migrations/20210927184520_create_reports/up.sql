CREATE OR REPLACE VIEW reports AS
SELECT
    current_timestamp AS since,
    current_timestamp AS until,
    --
    0 AS amount_expected,
    0 AS amount_received,
    0 AS amount_settled,
    0 AS amount_partial,
    0 AS amount_pending,
    --
    0 AS n_expected,
    0 AS n_received,
    0 AS n_settled,
    0 AS n_partial,
    0 AS n_pending,
    --
    0 AS ratio_expected,
    0 AS ratio_received,
    0 AS ratio_settled,
    0 AS ratio_partial,
    0 AS ratio_pending,
    --
    0 AS variation_expected,
    0 AS variation_received,
    0 AS variation_settled,
    0 AS variation_partial,
    0 AS variation_pending,
    --
    0 AS payment_rate,
    0 AS occupation_rate;
