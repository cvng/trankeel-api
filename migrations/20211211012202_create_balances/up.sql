CREATE OR REPLACE VIEW balances AS
WITH
tenant_balances AS (
    SELECT
        tenants.id AS tenant_id,
        0 - sum(full_amount) FILTER (WHERE rents.status IN ('open') AND rents.period_start < current_timestamp) AS balance
    FROM tenants
    LEFT JOIN leases ON leases.id = tenants.lease_id
    LEFT JOIN rents ON rents.lease_id = leases.id
    GROUP BY tenants.id
)
SELECT
    tenant_id AS id,
    tenant_id AS tenant_id,
    coalesce(balance, 0)::NUMERIC AS balance
FROM tenant_balances;
