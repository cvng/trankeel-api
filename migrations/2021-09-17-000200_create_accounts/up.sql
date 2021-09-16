CREATE TABLE accounts (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    plan_id UUID REFERENCES plans(id),
    status TEXT,
    stripe_customer_id TEXT,
    stripe_subscription_id TEXT,
    trial_end TIMESTAMPTZ
);

SELECT diesel_manage_updated_at('accounts');
