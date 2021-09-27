CREATE TABLE "advertisements" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "published" BOOLEAN NOT NULL,
    "lease_type" TEXT NOT NULL,
    "rent_amount" DECIMAL(65,30) NOT NULL,
    "rent_charges_amount" DECIMAL(65,30),
    "deposit_amount" DECIMAL(65,30),
    "effect_date" TIMESTAMPTZ NOT NULL,
    "flexibility" TEXT,
    "referral_lease_id" UUID,
    "property_id" UUID NOT NULL,

    CONSTRAINT "advertisements_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "advertisements" ADD CONSTRAINT "advertisements_referral_lease_id_fkey" FOREIGN KEY ("referral_lease_id") REFERENCES "leases"("id") ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE "advertisements" ADD CONSTRAINT "advertisements_property_id_fkey" FOREIGN KEY ("property_id") REFERENCES "properties"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

SELECT manage_updated_at('advertisements');
