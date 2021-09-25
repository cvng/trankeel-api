-- CreateTable
CREATE TABLE "tenants" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "account_id" UUID NOT NULL,
    "apl" BOOLEAN,
    "birthdate" DATE NOT NULL,
    "birthplace" TEXT,
    "email" TEXT NOT NULL,
    "first_name" TEXT NOT NULL,
    "last_name" TEXT NOT NULL,
    "note" TEXT,
    "phone_number" TEXT,
    "status" TEXT NOT NULL,
    "lease_id" UUID,
    "is_student" BOOLEAN,

    CONSTRAINT "tenants_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "tenants" ADD CONSTRAINT "tenants_account_id_fkey" FOREIGN KEY ("account_id") REFERENCES "accounts"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tenants" ADD CONSTRAINT "tenants_lease_id_fkey" FOREIGN KEY ("lease_id") REFERENCES "leases"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- ManageUpdatedAt
SELECT manage_updated_at('tenants');
