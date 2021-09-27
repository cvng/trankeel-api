CREATE TABLE "warrants" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT current_timestamp,
    "updated_at" TIMESTAMPTZ,
    "type" TEXT NOT NULL,
    "tenant_id" UUID NOT NULL,
    "individual_id" UUID,
    "professional_id" UUID,

    CONSTRAINT "warrants_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "warrants" ADD CONSTRAINT "warrants_tenant_id_fkey" FOREIGN KEY ("tenant_id") REFERENCES "tenants"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "warrants" ADD CONSTRAINT "warrants_individual_id_fkey" FOREIGN KEY ("individual_id") REFERENCES "persons"("id") ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE "warrants" ADD CONSTRAINT "warrants_professional_id_fkey" FOREIGN KEY ("professional_id") REFERENCES "professional_warrants"("id") ON DELETE SET NULL ON UPDATE CASCADE;

SELECT manage_updated_at('warrants');
