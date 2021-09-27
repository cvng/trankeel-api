CREATE TABLE "payments" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT current_timestamp,
    "updated_at" TIMESTAMPTZ,
    "rent_id" UUID NOT NULL,
    "amount" DECIMAL(65,30) NOT NULL,
    "date" TIMESTAMPTZ NOT NULL,
    "type" TEXT NOT NULL,
    "label" TEXT,

    CONSTRAINT "payments_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "payments" ADD CONSTRAINT "payments_rent_id_fkey" FOREIGN KEY ("rent_id") REFERENCES "rents"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

SELECT manage_updated_at('payments');
