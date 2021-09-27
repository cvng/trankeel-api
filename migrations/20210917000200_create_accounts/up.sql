CREATE TYPE accountstatus AS ENUM ('active', 'canceled', 'incomplete', 'incomplete_expired', 'past_due', 'trialing', 'unpaid');

CREATE TABLE "accounts" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT current_timestamp,
    "updated_at" TIMESTAMPTZ,
    "plan_id" UUID,
    "status" ACCOUNTSTATUS,
    "stripe_customer_id" TEXT,
    "stripe_subscription_id" TEXT,
    "trial_end" TIMESTAMPTZ,

    CONSTRAINT "accounts_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "accounts" ADD CONSTRAINT "accounts_plan_id_fkey" FOREIGN KEY ("plan_id") REFERENCES "plans"("id") ON DELETE SET NULL ON UPDATE CASCADE;

SELECT manage_updated_at('accounts');
