CREATE TABLE "candidacies" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "status" TEXT NOT NULL,
    "advertisement_id" UUID NOT NULL,
    "tenant_id" UUID NOT NULL,
    "move_in_date" TIMESTAMPTZ NOT NULL,
    "description" TEXT NOT NULL,

    CONSTRAINT "candidacies_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "candidacies" ADD CONSTRAINT "candidacies_advertisement_id_fkey" FOREIGN KEY ("advertisement_id") REFERENCES "advertisements"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "candidacies" ADD CONSTRAINT "candidacies_tenant_id_fkey" FOREIGN KEY ("tenant_id") REFERENCES "tenants"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

SELECT manage_updated_at('candidacies');
