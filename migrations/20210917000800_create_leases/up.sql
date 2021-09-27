-- CreateTable
CREATE TABLE "leases" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "account_id" UUID NOT NULL,
    "deposit_amount" DECIMAL(65,30),
    "effect_date" TIMESTAMPTZ NOT NULL,
    "signature_date" TIMESTAMPTZ,
    "rent_amount" DECIMAL(65,30) NOT NULL,
    "rent_charges_amount" DECIMAL(65,30),
    "type" TEXT NOT NULL,
    "lease_id" UUID,
    "property_id" UUID NOT NULL,
    "details" JSONB,
    "expired_at" TIMESTAMPTZ,
    "renew_date" TIMESTAMPTZ,
    "duration" TEXT NOT NULL,

    CONSTRAINT "leases_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "leases" ADD CONSTRAINT "leases_account_id_fkey" FOREIGN KEY ("account_id") REFERENCES "accounts"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "leases" ADD CONSTRAINT "leases_lease_id_fkey" FOREIGN KEY ("lease_id") REFERENCES "files"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "leases" ADD CONSTRAINT "leases_property_id_fkey" FOREIGN KEY ("property_id") REFERENCES "properties"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- ManageUpdatedAt
SELECT manage_updated_at('leases');
