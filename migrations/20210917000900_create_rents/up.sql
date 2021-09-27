-- CreateTable
CREATE TABLE "rents" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "period_end" TIMESTAMPTZ NOT NULL,
    "period_start" TIMESTAMPTZ NOT NULL,
    "amount" DECIMAL(65,30) NOT NULL,
    "charges_amount" DECIMAL(65,30),
    "full_amount" DECIMAL(65,30) NOT NULL,
    "status" TEXT NOT NULL,
    "lease_id" UUID NOT NULL,
    "receipt_id" UUID,
    "notice_id" UUID,

    CONSTRAINT "rents_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "rents" ADD CONSTRAINT "rents_lease_id_fkey" FOREIGN KEY ("lease_id") REFERENCES "leases"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- ManageUpdatedAt
SELECT manage_updated_at('rents');
